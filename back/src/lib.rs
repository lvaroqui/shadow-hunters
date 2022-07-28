use std::{collections::HashMap, fmt::Debug, sync::Arc};

use futures::future::join_all;
use locations::{location_by_number, LOCATIONS};
use rand::prelude::*;
use state::State;
use tokio::sync::{mpsc, oneshot};

pub mod characters;
mod locations;
mod state;

pub type PlayerId = u32;

#[derive(Debug)]
pub enum Team {
    Shadow,
    Hunter,
    Neutral,
}

#[derive(Debug)]
pub struct Character {
    name: &'static str,
    hp: usize,
    team: Team,
}

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    name: String,
    color: String,
    damage: usize,
    revealed: bool,
    location: usize,
    character: &'static Character,
}

impl Player {
    pub fn new(id: PlayerId, name: String, color: String, character: &'static Character) -> Self {
        Self {
            id,
            name,
            color,
            damage: 0,
            revealed: false,
            location: 0,
            character,
        }
    }

    fn is_dead(&self) -> bool {
        self.damage >= self.character.hp
    }
}

#[derive(Debug)]
struct Location {
    name: &'static str,
    numbers: &'static [usize],
}

#[derive(Debug)]
pub struct PlayerInfo {
    tx: mpsc::Sender<Command>,
    name: String,
    color: String,
}

#[derive(Debug)]
pub enum Command {
    WaitForAction {
        title: String,
        choices: Vec<String>,
        response_channel: oneshot::Sender<usize>,
    },
    StateChange {
        message: Arc<Option<String>>,
        ack: oneshot::Sender<()>,
    },
}

pub struct ShadowHunters {
    state: State,
    tx: HashMap<PlayerId, mpsc::Sender<Command>>,
    rng: StdRng,
}

pub struct GameBuilder {
    players: HashMap<PlayerId, PlayerInfo>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn register_player(
        &mut self,
        id: PlayerId,
        tx: mpsc::Sender<Command>,
        name: String,
        color: String,
    ) {
        println!("Hello {}", id);
        if self.players.contains_key(&id) {
            panic!("Double key!");
        }
        self.players.insert(id, PlayerInfo { tx, name, color });
    }

    pub fn build(self) -> ShadowHunters {
        let (tx, players): (_, Vec<_>) = self
            .players
            .into_iter()
            .map(|(id, p)| ((id, p.tx), state::PlayerInfo::new(id, p.name, p.color)))
            .unzip();
        ShadowHunters {
            state: State::new(players),
            tx,
            rng: StdRng::from_entropy(),
        }
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadowHunters {
    pub async fn play(mut self) {
        loop {
            self.turn_for(self.state.current_player().id).await;

            self.mutate_state(None, |state| state.next_player()).await;
        }
    }

    async fn wait_for_action<U>(
        &mut self,
        from_player: PlayerId,
        title: String,
        choices: Vec<(String, U)>,
    ) -> U {
        let (s, mut r): (Vec<_>, Vec<_>) = choices.into_iter().unzip();

        let (tx, rx) = oneshot::channel();

        self.tx[&from_player]
            .send(Command::WaitForAction {
                title,
                choices: s,
                response_channel: tx,
            })
            .await
            .unwrap();

        let res = rx.await.unwrap();
        r.remove(res)
    }

    async fn mutate_state<F>(&mut self, message: Option<String>, f: F)
    where
        F: FnOnce(&mut State),
    {
        f(&mut self.state);

        let message = Arc::new(message);

        join_all(self.tx.iter().map(|(_, tx)| {
            let tx = tx.clone();
            let message = message.clone();
            tokio::spawn(async move {
                let (ack_tx, ack_rx) = oneshot::channel();
                tx.send(Command::StateChange {
                    message,
                    ack: ack_tx,
                })
                .await
                .unwrap();
                ack_rx.await.unwrap();
            })
        }))
        .await;
    }

    fn roll_d4(&mut self) -> usize {
        self.rng.gen_range(1..=4)
    }

    fn roll_d6(&mut self) -> usize {
        self.rng.gen_range(1..=6)
    }

    fn roll_dices(&mut self) -> (usize, usize) {
        (self.roll_d6(), self.roll_d4())
    }

    async fn turn_for(&mut self, player_id: PlayerId) {
        self.movement(player_id).await;
    }

    async fn movement(&mut self, player_id: PlayerId) {
        self.wait_for_action(
            player_id,
            "Turn start".to_string(),
            vec![("Throw the dice".to_string(), ())],
        )
        .await;

        // Location selection
        let previous_location = self.state.player(player_id).location;
        let new_location = {
            let roll = loop {
                let roll = self.roll_dices();
                let roll = roll.0 + roll.1;
                if !LOCATIONS[previous_location].numbers.contains(&roll) {
                    break roll;
                }
            };
            if roll == 7 {
                let choice = self
                    .wait_for_action(
                        player_id,
                        "Choose location".to_string(),
                        LOCATIONS
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != previous_location)
                            .map(|(i, l)| (l.name.to_string(), i))
                            .collect(),
                    )
                    .await;

                choice
            } else {
                location_by_number(roll).0
            }
        };

        self.mutate_state(
            Some(format!(
                "{} moved from {} to {}",
                player_id, previous_location, new_location
            )),
            |state| state.move_player(player_id, new_location),
        )
        .await;
    }
}
