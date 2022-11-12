#[cfg(feature = "game-logic")]
mod game_logic;
pub mod state;

pub use crate::state::{Location, LocationId, Locations};

#[cfg(feature = "game-logic")]
pub use game_logic::{Command, GameLogic};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerId(usize);

impl PlayerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Dices {
    D4,
    D6,
    Both,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Skip,
    DiceRoll(Dices),
    Location(LocationId),
    Player(PlayerId),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InfoMessage {
    Basic(String),
    Roll { from: PlayerId, roll: Roll },
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Roll {
    pub d4: u8,
    pub d6: u8,
}
