
struct Server<Stage = GameBuilder> {
    stdin_receiver: Arc<Mutex<mpsc::Receiver<String>>>,
    players: Vec<Player>,
    stage: Stage,
    next_id: PlayerId,
}

impl Server<GameBuilder> {
    fn new() -> Self {
        let (stdin_sender, stdin_receiver) = tokio::sync::mpsc::channel(10);

        tokio::task::spawn_blocking(move || loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            stdin_sender.blocking_send(buffer).unwrap();
        });

        Self {
            stdin_receiver: Arc::new(Mutex::new(stdin_receiver)),
            players: vec![],
            stage: GameBuilder::new(),
            next_id: 0,
        }
    }

    fn register_player(&mut self, name: String, color: String) {
        let (tx, rx) = mpsc::channel(10);
        self.players.push(Player::new(self.next_id, rx));
        self.stage.register_player(self.next_id, tx, name, color);
        self.next_id += 1;
    }

    fn build(self) -> Server<ShadowHunters> {
        Server::<ShadowHunters> {
            stdin_receiver: self.stdin_receiver,
            players: self.players,
            stage: self.stage.build(),
            next_id: self.next_id,
        }
    }
}

impl Server<ShadowHunters> {
    async fn play(self) {
        for mut p in self.players.into_iter() {
            let stdin_receiver = Arc::clone(&self.stdin_receiver);
            tokio::spawn(async move {
                while let Some(cmd) = p.rx.recv().await {
                    let mut stdin_receiver = stdin_receiver.lock().await;
                    Self::player_handle(stdin_receiver.deref_mut(), p.id, cmd).await;
                }
            });
        }

        self.stage.play().await;
    }

    async fn player_handle(
        stdin_receiver: &mut mpsc::Receiver<String>,
        id: PlayerId,
        cmd: Command,
    ) {
        print!("Player {}, received: ", id);
        match cmd {
            shadow_hunters::Command::WaitForAction {
                title,
                choices,
                response_channel,
            } => {
                println!("{}", title);
                for (i, c) in choices.iter().enumerate() {
                    println!("{}: {}", i, c);
                }

                let res = loop {
                    if let Ok(res) = stdin_receiver.recv().await.unwrap().trim().parse() {
                        if res < choices.len() {
                            break res;
                        }
                    }
                    println!("Not a valid number");
                };

                response_channel.send(res).unwrap();
            }
            shadow_hunters::Command::StateChange { message, ack } => {
                println!("State Change");
                if let Some(message) = message {
                    println!("{}", message);
                }
                ack.send(()).unwrap();
            }
        }
    }
}