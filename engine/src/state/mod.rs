use std::fmt::Debug;
use std::ops::{Index, IndexMut};

mod characters;
mod locations;

use crate::dice::Dice;
use crate::PlayerId;

use self::characters::{Character, CharacterId, Characters};
pub use self::locations::{Location, LocationId, Locations};

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

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    damage: usize,
    location: LocationId,
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

#[derive(Debug)]
pub struct State {
    players: Vec<Player>,
    current_player: PlayerId,
    locations: Locations,
}

impl State {
    pub fn new(player_count: usize, dice: &mut Dice) -> State {
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
                    character: characters.pop(),
                })
                .collect(),
            current_player: PlayerId(0),
            locations,
        }
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mutation {
    Move(PlayerId, LocationId),
    ChangeCurrentPlayer(PlayerId),
    DamagePlayer(PlayerId, usize),
}
