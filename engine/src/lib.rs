use anyhow::Result;
use state::State;
use tokio::sync::{mpsc, oneshot};

mod dice;
mod state;

pub use crate::state::Mutation;
pub use crate::state::{Location, LocationId, Locations};
use dice::Dice;
pub use dice::Roll;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(usize);

pub struct ShadowHunter {
    message_channel: mpsc::Sender<Message>,
    state: State,
    dice: Dice,
}

#[derive(Debug)]
pub enum InfoMessage {
    Roll { from: PlayerId, roll: Roll },
}

#[derive(Debug)]
pub enum Message {
    ActionRequest {
        player: PlayerId,
        choices: Vec<String>,
        response: oneshot::Sender<usize>,
    },
    Info {
        destination: Vec<PlayerId>,
        payload: InfoMessage,
    },
    StateMutation(Mutation),
}

impl ShadowHunter {
    pub fn new(player_count: usize, command_channel: mpsc::Sender<Message>) -> Self {
        let mut dice = Dice::new();
        ShadowHunter {
            message_channel: command_channel,
            state: State::new(player_count, &mut dice),
            dice,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("{:#?}", self.state);
        loop {
            // Movements
            self.request_action(self.state.current_player().id(), &[("Roll the dice", ())])
                .await?;
            let current_player_location = self.state.current_player().location();
            let roll = loop {
                let roll = self.dice.roll();
                if !current_player_location.dice_numbers().contains(&roll.sum()) {
                    break roll;
                }
            };
            self.message_channel
                .send(Message::Info {
                    destination: self.state.players().iter().map(|p| p.id()).collect(),
                    payload: InfoMessage::Roll {
                        from: self.state.current_player().id(),
                        roll,
                    },
                })
                .await?;

            let location = if roll.sum() == 7 {
                let choices = self
                    .state
                    .locations()
                    .iter()
                    .filter(|l| l.id() != current_player_location.id())
                    .map(|l| (format!("{:?}", l), l))
                    .collect::<Vec<_>>();
                self.request_action(self.state.current_player().id(), &choices)
                    .await?
            } else {
                self.state.locations().location_from_dice_number(roll.sum())
            };
            self.mutate_state(Mutation::Move(
                self.state.current_player().id(),
                location.id(),
            ))
            .await?;

            // Attack
            // TODO

            // Advance current player
            let p = self
                .state
                .players()
                .iter()
                .cycle() // Make the iterator cycle so we can loop back from last player to first
                .skip_while(|p| p.id() != self.state.current_player().id()) // Find current player
                .skip(1) // Skip him
                .take(self.state.players().len() - 1) // Avoid looping back to current player
                .find(|p| p.is_alive())
                .expect("If there are no other players, the game should be over");

            self.mutate_state(Mutation::ChangeCurrentPlayer(p.id()))
                .await?;
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
        self.message_channel
            .send(Message::ActionRequest {
                player: from_player,
                choices: choices.iter().map(|(s, _)| s.to_string()).collect(),
                response: snd,
            })
            .await?;
        let r = rcv.await?;
        Ok(choices[r].1)
    }

    async fn mutate_state(&mut self, mutation: Mutation) -> Result<()> {
        self.state.mutate(mutation);
        self.message_channel
            .send(Message::StateMutation(mutation))
            .await?;
        Ok(())
    }
}
