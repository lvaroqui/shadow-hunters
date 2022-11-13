pub mod state;

pub use state::{CharacterId, LocationId, PlayerId};

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

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Roll {
    pub d4: u8,
    pub d6: u8,
}

impl Roll {
    pub fn sum(&self) -> usize {
        (self.d4 + self.d6).into()
    }

    pub fn diff(&self) -> usize {
        self.d4.abs_diff(self.d6).into()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum InfoMessage {
    Basic(String),
    Roll { from: PlayerId, roll: Roll },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ToPlayer {
    ActionRequest(Vec<Action>),
    Info(InfoMessage),
    StateMutation(state::Mutation),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FromPlayer {
    ActionChoice(usize),
}
