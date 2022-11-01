use std::fmt::Display;

use anyhow::Result;
use engine::{Locations, Message, ShadowHunter};
use tokio::{io::AsyncBufReadExt, sync::mpsc};

async fn read_line() -> Result<String> {
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    stdin.read_line(&mut line).await?;
    Ok(line.trim().to_owned())
}

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, mut receiver) = mpsc::channel(1);
    let mut sh = ShadowHunter::new(5, sender);

    tokio::spawn(async move { sh.run().await });

    while let Some(cmd) = receiver.recv().await {
        match cmd {
            Message::ActionRequest {
                player,
                choices,
                response,
            } => {
                let choice = loop {
                    println!("Action request for player {:?}:", player);
                    for (i, c) in choices.iter().enumerate() {
                        print!("  {}: ", i,);
                        match c {
                            engine::Action::Basic(s) => println!("{}", s),
                            engine::Action::Location(l) => println!("{}", Locations::from_id(*l)),
                        }
                    }
                    let input = read_line().await?;
                    if let Ok(choice) = input.parse() {
                        if choice < choices.len() {
                            break choice;
                        }
                        println!("Invalid input");
                    }
                };
                response.send(choice).unwrap();
            }
            Message::Info {
                destination,
                payload,
            } => {
                println!("Received info for players: {:?}", destination);
                println!("  {:?}", payload);
            }
            Message::StateMutation(mutation) => match mutation {
                engine::Mutation::Move(player_id, location_id) => {
                    println!(
                        "{:?} moved to {}",
                        player_id,
                        Locations::from_id(location_id)
                    );
                }
                engine::Mutation::ChangeCurrentPlayer(player_id) => {
                    println!();
                    println!("Current player is now: {:?}", player_id);
                }
            },
        }
    }

    Ok(())
}
