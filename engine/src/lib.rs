#[cfg(feature = "game-logic")]
mod game_logic;
pub mod state;

pub use state::{Location, LocationId, Locations, PlayerId};

#[cfg(feature = "game-logic")]
pub use game_logic::{Command, GameLogic};

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
    DamagePlayer(PlayerId, Option<usize>),
    HealPlayer(PlayerId, Option<usize>),
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
