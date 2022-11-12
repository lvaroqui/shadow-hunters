use std::fmt::Debug;
use std::ops::{Index, IndexMut};

mod characters;
mod locations;

use self::characters::{Character, CharacterId, Characters};
pub use self::locations::{Location, LocationId, Locations};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerId(usize);

impl PlayerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl Index<PlayerId> for Vec<Player> {
    type Output = Player;

    fn index(&self, index: PlayerId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<PlayerId> for Vec<Player> {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    id: PlayerId,
    damage: usize,
    location: LocationId,
    revealed: bool,
    character: Option<CharacterId>,
}

impl Player {
    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn location(&self) -> &'static dyn Location {
        Locations::from_id(self.location)
    }

    pub fn character(&self) -> Option<&'static dyn Character> {
        self.character.map(Characters::from_id)
    }

    pub fn is_alive(&self) -> bool {
        if let Some(character) = self.character() {
            self.damage < character.hit_point()
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct State {
    players: Vec<Player>,
    current_player: PlayerId,
    locations: Locations,
}

impl State {
    #[cfg(feature = "game-logic")]
    pub fn new(player_count: usize) -> State {
        use crate::game_logic::Dice;
        let mut dice = Dice::new();
        let locations = Locations::generate();
        let mut characters = Characters::generate(player_count);
        State {
            players: (0..player_count)
                .into_iter()
                .map(|id| Player {
                    id: PlayerId(id),
                    damage: 0,
                    location: Locations::from_dice_number(loop {
                        let s = dice.roll().sum();
                        if s != 7 {
                            break s;
                        }
                    })
                    .id(),
                    revealed: false,
                    character: Some(characters.pop().unwrap()),
                })
                .collect(),
            current_player: PlayerId(0),
            locations,
        }
    }

    #[cfg(feature = "game-logic")]
    pub fn prepare_for_player(&self, player_id: PlayerId) -> Self {
        let mut res = self.clone();
        for p in &mut res.players {
            if p.id() != player_id && !p.revealed {
                p.character = None;
            }
        }
        res
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn locations(&self) -> &Locations {
        &self.locations
    }

    pub fn mutate(&mut self, mutation: Mutation) {
        match mutation {
            Mutation::Move(player_id, location_id) => {
                self.players[player_id].location = location_id;
            }
            Mutation::ChangeCurrentPlayer(player_id) => self.current_player = player_id,
            Mutation::DamagePlayer(player_id, damage) => self.players[player_id].damage += damage,
            Mutation::HealPlayer(player_id, hp) => {
                self.players[player_id].damage = self.players[player_id].damage.saturating_sub(hp)
            }
            Mutation::RevealPlayer(player_id, character_id) => {
                let player = &mut self.players[player_id];
                player.revealed = true;
                if let Some(c) = player.character {
                    assert_eq!(c, character_id);
                }
                player.character = Some(character_id);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Mutation {
    Move(PlayerId, LocationId),
    ChangeCurrentPlayer(PlayerId),
    DamagePlayer(PlayerId, usize),
    HealPlayer(PlayerId, usize),
    RevealPlayer(PlayerId, CharacterId),
}
