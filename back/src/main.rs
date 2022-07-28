use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use shadow_hunters::{Command, GameBuilder, PlayerId, ShadowHunters};
use tokio::sync::{mpsc, Mutex};

use axum::{
    extract::Extension,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;

struct Player {
    id: PlayerId,
    tx: mpsc::Sender<Command>,
}

impl Player {
    fn new(id: PlayerId, tx: mpsc::Sender<Command>) -> Self {
        Self { id, tx }
    }
}

struct Room {
    players: Vec<Player>,
}

impl Room {
    fn new() -> Self {
        Self { players: vec![] }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let room = Arc::new(Mutex::new(Room::new()));

    let api_routes = Router::new()
        .route(
            "/ws/:id",
            get({
                let room = Arc::clone(&room);
                move |id, ws| handler(id, ws, room)
            }),
        )
        .route(
            "/run",
            get({
                let room = Arc::clone(&room);
                move || run(room)
            }),
        );

    let app = Router::new().nest("/api", api_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn run(room: Arc<Mutex<Room>>) -> StatusCode {
    let room = room.lock().await;

    let mut gb = GameBuilder::new();

    for p in room.players.iter() {
        gb.register_player(p.id, p.tx.clone(), "toto".to_string(), "toto".to_string())
    }

    let sh = gb.build();

    tokio::spawn(async move {
        sh.play().await;
    });

    println!("Game Started!");

    StatusCode::OK
}

async fn handler(Path(id): Path<u32>, ws: WebSocketUpgrade, room: Arc<Mutex<Room>>) -> Response {
    let (tx, rx) = mpsc::channel(10);
    room.lock().await.players.push(Player::new(id, tx));
    println!("Registered player {}", id);
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
enum Commands {
    WaitForAction { title: String, choices: Vec<String> },
    StateChange { message: Option<String> },
}

async fn handle_socket(mut socket: WebSocket, mut receiver: mpsc::Receiver<Command>) {
    while let Some(cmd) = receiver.recv().await {
        match cmd {
            Command::WaitForAction {
                title,
                choices,
                response_channel,
            } => {
                socket
                    .send(Message::Text(
                        serde_json::to_string(&Commands::WaitForAction { title, choices }).unwrap(),
                    ))
                    .await
                    .unwrap();
                println!("Waiting for socket");
                while let Some(msg) = socket.recv().await {
                    match msg {
                        Ok(msg) => match msg {
                            Message::Text(text) => {
                                println!("received text: {}", text);
                                response_channel.send(text.parse().unwrap()).unwrap();
                                break;
                            }
                            Message::Close(_) => {
                                println!("WebSocket closed");
                                return;
                            }
                            Message::Ping(data) => {
                                println!("Received Ping message");
                                socket.send(Message::Pong(data)).await.unwrap();
                            }
                            m => {
                                println!("{:?}", m);
                            }
                        },
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
            Command::StateChange { message, ack } => {
                socket
                    .send(Message::Text(
                        serde_json::to_string(&Commands::StateChange {
                            message: message.deref().clone(),
                        })
                        .unwrap(),
                    ))
                    .await
                    .unwrap();
                ack.send(()).unwrap();
            }
        }
    }
}
