use std::io::BufRead;

use shared::FromPlayer;
use tungstenite::connect;
use url::Url;

fn main() {
    let (mut socket, _response) = connect(Url::parse("ws://localhost:3001/api/join").unwrap())
        .map_err(|e| dbg!(e))
        .expect("Can't connect");

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock().lines();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        match msg {
            tungstenite::Message::Text(msg) => {
                let msg: shared::ToPlayer = serde_json::from_str(&msg).unwrap();
                match msg {
                    shared::ToPlayer::ActionRequest { choices } => {
                        let choice = loop {
                            println!("Action request for player");
                            for (i, c) in choices.iter().enumerate() {
                                print!("  {}: ", i,);
                                match c {
                                    shared::Action::Location(l) => {
                                        println!("{}", shared::Locations::from_id(*l))
                                    }
                                    shared::Action::Player(p) => {
                                        println!("{:?}", p)
                                    }
                                    shared::Action::Skip => {
                                        println!("Do nothing")
                                    }
                                    shared::Action::DiceRoll(dices) => match dices {
                                        shared::Dices::D4 => println!("Roll D4"),
                                        shared::Dices::D6 => println!("Roll D6"),
                                        shared::Dices::Both => println!("Roll both dice"),
                                    },
                                }
                            }
                            let input = stdin.next().unwrap().unwrap();
                            if let Ok(choice) = input.parse() {
                                if choice < choices.len() {
                                    break choice;
                                }
                                println!("Invalid input");
                            }
                        };
                        socket
                            .write_message(tungstenite::Message::Text(
                                serde_json::to_string(&FromPlayer::ActionChoice(choice)).unwrap(),
                            ))
                            .unwrap();
                    }
                    msg => println!("Received: {:?}", msg),
                }
            }
            tungstenite::Message::Binary(_) => todo!(),
            tungstenite::Message::Ping(_) => todo!(),
            tungstenite::Message::Pong(_) => todo!(),
            tungstenite::Message::Close(_) => todo!(),
            tungstenite::Message::Frame(_) => unreachable!(),
        }
    }
}
