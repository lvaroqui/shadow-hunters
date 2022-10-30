use anyhow::Result;
use engine::{Command, ShadowHunter};
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
            Command::ActionRequest {
                player,
                choices,
                response,
            } => {
                let choice = loop {
                    println!("{:?} {:?}", player, choices);
                    let input = read_line().await?;
                    if let Ok(choice) = input.parse() {
                        if choice < choices.len() {
                            break choice;
                        }
                    }
                };
                response.send(choice).unwrap();
            }
        }
    }

    Ok(())
}
