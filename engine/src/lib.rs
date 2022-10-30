use anyhow::Result;
use state::State;
use tokio::sync::{mpsc, oneshot};

mod state;

#[derive(Debug, Clone, Copy)]
pub struct PlayerId(usize);

pub struct ShadowHunter {
    request_channel: mpsc::Sender<Command>,
    state: State,
}

#[derive(Debug)]
pub enum Command {
    ActionRequest {
        player: PlayerId,
        choices: Vec<String>,
        response: oneshot::Sender<usize>,
    },
}

impl ShadowHunter {
    pub fn new(player_count: usize, request_channel: mpsc::Sender<Command>) -> Self {
        ShadowHunter {
            request_channel,
            state: State::new(player_count),
        }
    }

    async fn request_action<T>(
        &mut self,
        from_player: PlayerId,
        choices: &[(impl ToString, T)],
    ) -> Result<T>
    where
        T: Copy + Send,
    {
        let (snd, rcv) = oneshot::channel();
        self.request_channel
            .send(Command::ActionRequest {
                player: from_player,
                choices: choices.iter().map(|(s, _)| s.to_string()).collect(),
                response: snd,
            })
            .await?;
        let r = rcv.await?;
        Ok(choices[r].1)
    }

    pub async fn run(&mut self) -> Result<()> {
        self.request_action(PlayerId(0), &[("Throw the dice", ())])
            .await?;
        println!("Dice launched!!!");

        let selected = self
            .request_action(
                PlayerId(0),
                &[("Kill Luc", PlayerId(1)), ("Kill Marie", PlayerId(2))],
            )
            .await?;
        println!("Selected: {:?}", selected);

        Ok(())
    }
}
