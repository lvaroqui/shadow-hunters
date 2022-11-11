use std::{net::SocketAddr, sync::Arc};

use engine::PlayerId;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::{mpsc, oneshot, Mutex};

use axum::{
    extract::ws::{self, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};

struct Player {
    id: PlayerId,
    tx: mpsc::Sender<PlayerMessage>,
    request_answer: Option<oneshot::Sender<usize>>,
}

impl Player {
    fn new(id: PlayerId, tx: mpsc::Sender<PlayerMessage>) -> Self {
        Self {
            id,
            tx,
            request_answer: None,
        }
    }
}

enum RoomState {
    Registration,
    Running,
}

struct Room {
    state: RoomState,
    players: Vec<Player>,
}

impl Room {
    fn new() -> Self {
        Self {
            state: RoomState::Registration,
            players: vec![],
        }
    }
}

#[derive(Debug)]
enum PlayerMessage {
    ActionRequest { choices: Vec<engine::Action> },
    Info { payload: engine::InfoMessage },
    StateMutation(engine::Mutation),
    Pong(Vec<u8>),
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let room = Arc::new(Mutex::new(Room::new()));

    let api_routes = Router::new()
        .route(
            "/join",
            get({
                let room = Arc::clone(&room);
                move |ws| Room::register_player(room, ws)
            }),
        )
        .route(
            "/start",
            get({
                let room = Arc::clone(&room);
                move || Room::start(room)
            }),
        );

    let app = Router::new().nest("/api", api_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

impl Room {
    async fn start(room: Arc<Mutex<Self>>) -> StatusCode {
        let player_count = {
            let mut room = room.lock().await;
            room.state = RoomState::Running;
            room.players.len()
        };

        let (tx, rx) = mpsc::channel(1);
        let mut sh = engine::GameLogic::new(player_count, tx);
        tokio::spawn(async move { sh.run().await });
        tokio::spawn(async move { Self::run(room, rx).await });

        StatusCode::OK
    }

    async fn run(room: Arc<Mutex<Self>>, mut rx: mpsc::Receiver<engine::Command>) {
        while let Some(message) = rx.recv().await {
            match message {
                engine::Command::ActionRequest {
                    player,
                    choices,
                    response,
                } => {
                    let mut room = room.lock().await;

                    let p = room.get_player_mut(player);
                    p.tx.send(PlayerMessage::ActionRequest { choices })
                        .await
                        .unwrap();
                    assert!(p.request_answer.is_none());
                    p.request_answer = Some(response)
                }
                engine::Command::Info {
                    destination,
                    payload,
                } => {
                    let mut room = room.lock().await;
                    for p in room
                        .players
                        .iter_mut()
                        .filter(|p| destination.contains(&p.id))
                    {
                        p.tx.send(PlayerMessage::Info {
                            payload: payload.clone(),
                        })
                        .await
                        .unwrap();
                    }
                }
                engine::Command::StateMutation(mutation) => {
                    let mut room = room.lock().await;
                    for p in &mut room.players {
                        p.tx.send(PlayerMessage::StateMutation(mutation))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }

    async fn register_player(room: Arc<Mutex<Room>>, ws: WebSocketUpgrade) -> Response {
        let (tx, rx) = mpsc::channel(10);
        let id = {
            let mut room = room.lock().await;
            let id = PlayerId::new(room.players.len());
            room.players.push(Player::new(id, tx));
            id
        };
        ws.on_upgrade(move |socket| Self::handle_player(room, id, socket, rx))
    }

    async fn handle_player(
        room: Arc<Mutex<Self>>,
        id: PlayerId,
        socket: WebSocket,
        receiver: mpsc::Receiver<PlayerMessage>,
    ) {
        let (socket_tx, socket_rx) = socket.split();
        {
            let room = Arc::clone(&room);
            tokio::spawn(async move { Self::handle_player_ws(room, id, socket_rx).await });
        }
        Self::handle_player_commands(receiver, socket_tx).await;
    }

    async fn handle_player_commands(
        mut receiver: mpsc::Receiver<PlayerMessage>,
        mut socket: SplitSink<WebSocket, ws::Message>,
    ) {
        while let Some(msg) = receiver.recv().await {
            match msg {
                PlayerMessage::ActionRequest { choices } => {
                    socket
                        .send(ws::Message::Text(
                            serde_json::to_string(&shared::ToPlayer::ActionRequest { choices })
                                .unwrap(),
                        ))
                        .await
                        .unwrap();
                }
                PlayerMessage::Info { payload } => {
                    socket
                        .send(ws::Message::Text(
                            serde_json::to_string(&shared::ToPlayer::Info { payload }).unwrap(),
                        ))
                        .await
                        .unwrap();
                }
                PlayerMessage::StateMutation(mutation) => {
                    socket
                        .send(ws::Message::Text(
                            serde_json::to_string(&shared::ToPlayer::StateMutation(mutation))
                                .unwrap(),
                        ))
                        .await
                        .unwrap();
                }
                PlayerMessage::Pong(data) => {
                    socket.send(ws::Message::Pong(data)).await.unwrap();
                }
            }
        }
    }

    async fn handle_player_ws(
        room: Arc<Mutex<Self>>,
        id: PlayerId,
        mut socket: SplitStream<WebSocket>,
    ) {
        while let Some(msg) = socket.next().await {
            match msg {
                Ok(msg) => match msg {
                    ws::Message::Text(text) => {
                        let msg: shared::FromPlayer = serde_json::from_str(&text).unwrap();
                        match msg {
                            shared::FromPlayer::ActionChoice(choice) => {
                                let mut room = room.lock().await;
                                if let Some(tx) = room.get_player_mut(id).request_answer.take() {
                                    tx.send(choice).unwrap();
                                } else {
                                    println!("Did not expect a choice from {:?}", id);
                                }
                            }
                        }
                    }
                    ws::Message::Close(_) => {
                        println!("WebSocket closed");
                        return;
                    }
                    ws::Message::Ping(data) => {
                        let mut room = room.lock().await;
                        room.get_player_mut(id)
                            .tx
                            .send(PlayerMessage::Pong(data))
                            .await
                            .unwrap();
                    }
                    m => {
                        println!("{:?}", m);
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }

    fn get_player_mut(&mut self, id: PlayerId) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.id == id)
            .expect("Invalid player id")
    }
}
