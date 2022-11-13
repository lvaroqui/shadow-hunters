use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use shared::{
    state::Mutation,
    state::{Location, State},
    Action, Dices, InfoMessage, PlayerId,
};

use tokio::sync::{mpsc, oneshot};

mod dice;

pub(crate) use dice::Dice;

use crate::locations::{location_behaviors, LocationBehavior};

#[derive(Debug)]
pub enum Command {
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

pub struct GameLogic {
    pub(crate) message_channel: MessageChannel,
    pub(crate) state: State,
    pub(crate) dice: Dice,
    pub(crate) location_behaviors: [&'static dyn LocationBehavior; 6],
}

impl std::fmt::Debug for GameLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameLogic")
            .field("message_channel", &self.message_channel)
            .field("state", &self.state)
            .field("dice", &self.dice)
            .finish_non_exhaustive()
    }
}

impl GameLogic {
    pub fn new(player_count: usize, command_channel: mpsc::Sender<Command>) -> Self {
        let location_behaviors = location_behaviors();

        let locations = {
            let locations = {
                let mut i = 0;
                location_behaviors.map(|lb| {
                    let res = Location::new(i, lb.name().to_owned(), lb.dice_numbers());
                    i += 1;
                    res
                })
            };
            let mut layout = locations.map(|l| l.id());
            layout.shuffle(&mut rand::thread_rng());

            shared::state::Locations::new(locations, layout)
        };

        GameLogic {
            message_channel: MessageChannel(command_channel),
            state: State::new(players, locations, first_player, characters),
            dice: Dice::new(),
            location_behaviors,
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
            .filter(|p| attackable_locations.contains(&p.location().id()))
            .filter(|p| p.id() != self.state.current_player().id())
            .map(|p| (Action::DamagePlayer(p.id(), None), Some(p.id())));
        if let Some(player_id) = self
            .message_channel
            .request_action_map(
                self.state.current_player().id(),
                attackable_players.chain(std::iter::once((Action::Skip, None))),
            )
            .await?
        {
            self.broadcast_info(InfoMessage::Basic(format!(
                "{:?} is preparing an attack on {:?}",
                self.state.current_player().id(),
                player_id
            )))
            .await?;

            self.message_channel
                .request_action_map(
                    self.state.current_player().id(),
                    [(Action::DiceRoll(Dices::Both), ())],
                )
                .await?;

            let roll = self.dice.roll();
            self.broadcast_info(InfoMessage::Roll {
                from: self.state.current_player().id(),
                roll,
            })
            .await?;
            let damage = roll.diff();
            self.mutate_state(Mutation::DamagePlayer(player_id, damage))
                .await?;
        } else {
            self.broadcast_info(InfoMessage::Basic(format!(
                "{:?} did not attack",
                self.state.current_player().id()
            )))
            .await?;
        }
        Ok(())
    }

    async fn movement(&mut self) -> Result<(), anyhow::Error> {
        self.message_channel
            .request_action_map(
                self.state.current_player().id(),
                [(Action::DiceRoll(Dices::Both), ())],
            )
            .await?;
        let roll = loop {
            let roll = self.dice.roll();
            if !self
                .state
                .current_player()
                .location()
                .dice_numbers()
                .contains(&roll.sum())
            {
                break roll;
            }
        };
        self.broadcast_info(InfoMessage::Roll {
            from: self.state.current_player().id(),
            roll,
        })
        .await?;

        let location_id = if roll.sum() == 7 {
            let choices = self
                .state
                .locations()
                .iter()
                .filter(|l| l.id() != self.state.current_player().location().id())
                .map(|l| (Action::Location(l.id()), l.id()));
            self.message_channel
                .request_action_map(self.state.current_player().id(), choices)
                .await?
        } else {
            self.state.locations().from_dice_number(roll.sum()).id()
        };
        self.mutate_state(Mutation::Move(
            self.state.current_player().id(),
            location_id,
        ))
        .await?;

        self.location_behaviors[usize::from(location_id)]
            .handle(self, self.state.current_player().id())
            .await;
        Ok(())
    }

    async fn next_player(&mut self) -> Result<(), anyhow::Error> {
        let p = self
            .state
            .players()
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

    pub(crate) async fn mutate_state(&mut self, mutation: Mutation) -> Result<()> {
        self.state.mutate(mutation);
        self.message_channel
            .send(Command::StateMutation(mutation))
            .await?;
        // TODO: check victory condition
        Ok(())
    }

    pub(crate) async fn broadcast_info(&mut self, message: InfoMessage) -> Result<()> {
        self.message_channel
            .send(Command::Info {
                destination: self.state.players().map(|p| p.id()).collect(),
                payload: message,
            })
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct MessageChannel(mpsc::Sender<Command>);

impl MessageChannel {
    pub(crate) async fn request_action_map<T>(
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
            .send(Command::ActionRequest {
                player: from_player,
                choices,
                response: snd,
            })
            .await?;
        let r = rcv.await?;
        Ok(res[r])
    }

    pub(crate) async fn send(&mut self, message: Command) -> Result<()> {
        self.0.send(message).await?;
        Ok(())
    }
}
