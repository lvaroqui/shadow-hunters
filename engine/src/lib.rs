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

#[derive(Debug)]
pub struct MessageChannel(mpsc::Sender<Message>);

#[derive(Debug)]
pub struct GameLogic {
    message_channel: MessageChannel,
    state: State,
    dice: Dice,
}

#[derive(Debug)]
pub enum Message {
    ActionRequest {
        player: PlayerId,
        choices: Vec<Action>,
        response: oneshot::Sender<usize>,
    },
    Info {
        destination: Vec<PlayerId>,
        payload: InfoMessage,
    },
    StateMutation(Mutation),
}

#[derive(Debug)]
pub enum Dices {
    D4,
    D6,
    Both,
}

#[derive(Debug)]
pub enum Action {
    Skip,
    DiceRoll(Dices),
    Location(LocationId),
    Player(PlayerId),
}

#[derive(Debug)]
pub enum InfoMessage {
    Basic(String),
    Roll { from: PlayerId, roll: Roll },
}

impl GameLogic {
    pub fn new(player_count: usize, command_channel: mpsc::Sender<Message>) -> Self {
        let mut dice = Dice::new();
        GameLogic {
            message_channel: MessageChannel(command_channel),
            state: State::new(player_count, &mut dice),
            dice,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.movement().await?;
            self.attack().await?;
            self.next_player().await?;
        }
    }

    async fn attack(&mut self) -> Result<(), anyhow::Error> {
        let attackable_locations = self
            .state
            .locations()
            .in_group_iter(self.state.current_player().location().id())
            .map(|l| l.id())
            .collect::<Vec<_>>();
        let attackable_players = self
            .state
            .players()
            .iter()
            .filter(|p| attackable_locations.contains(&p.location().id()))
            .filter(|p| p.id() != self.state.current_player().id())
            .map(|p| (Action::Player(p.id()), Some(p.id())));
        if let Some(player_id) = self
            .message_channel
            .request_action(
                self.state.current_player().id(),
                attackable_players.chain(std::iter::once((Action::Skip, None))),
            )
            .await?
        {
            self.message_channel
                .send(Message::Info {
                    destination: self.state.players().iter().map(|p| p.id()).collect(),
                    payload: InfoMessage::Basic(format!(
                        "{:?} is preparing an attack on {:?}",
                        self.state.current_player().id(),
                        player_id
                    )),
                })
                .await?;

            self.message_channel
                .request_action(
                    self.state.current_player().id(),
                    [(Action::DiceRoll(Dices::Both), ())],
                )
                .await?;

            let roll = self.dice.roll();
            self.message_channel
                .send(Message::Info {
                    destination: self.state.players().iter().map(|p| p.id()).collect(),
                    payload: InfoMessage::Roll {
                        from: self.state.current_player().id(),
                        roll,
                    },
                })
                .await?;
            let damage = roll.diff();
            self.mutate_state(Mutation::DamagePlayer(player_id, damage))
                .await?;
        } else {
            self.message_channel
                .send(Message::Info {
                    destination: self.state.players().iter().map(|p| p.id()).collect(),
                    payload: InfoMessage::Basic(format!(
                        "{:?} did not attack",
                        self.state.current_player()
                    )),
                })
                .await?;
        }
        Ok(())
    }

    async fn movement(&mut self) -> Result<(), anyhow::Error> {
        self.message_channel
            .request_action(
                self.state.current_player().id(),
                [(Action::DiceRoll(Dices::Both), ())],
            )
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
                .map(|l| (Action::Location(l.id()), l));
            self.message_channel
                .request_action(self.state.current_player().id(), choices)
                .await?
        } else {
            Locations::from_dice_number(roll.sum())
        };
        self.mutate_state(Mutation::Move(
            self.state.current_player().id(),
            location.id(),
        ))
        .await?;
        Ok(())
    }

    async fn next_player(&mut self) -> Result<(), anyhow::Error> {
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
        Ok(())
    }

    async fn mutate_state(&mut self, mutation: Mutation) -> Result<()> {
        self.state.mutate(mutation);
        self.message_channel
            .send(Message::StateMutation(mutation))
            .await?;
        // TODO: check victory condition
        Ok(())
    }
}

impl MessageChannel {
    async fn request_action<T>(
        &mut self,
        from_player: PlayerId,
        choices: impl IntoIterator<Item = (Action, T)>,
    ) -> Result<T>
    where
        T: Copy + Send,
    {
        let (snd, rcv) = oneshot::channel();
        let (choices, res): (_, Vec<_>) = choices.into_iter().unzip();
        self.0
            .send(Message::ActionRequest {
                player: from_player,
                choices,
                response: snd,
            })
            .await?;
        let r = rcv.await?;
        Ok(res[r])
    }

    async fn send(&mut self, message: Message) -> Result<()> {
        self.0.send(message).await?;
        Ok(())
    }
}
