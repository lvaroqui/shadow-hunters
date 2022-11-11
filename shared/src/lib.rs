pub use engine::{Action, Dices, InfoMessage, Locations, Mutation};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ToPlayer {
    ActionRequest { choices: Vec<Action> },
    Info { payload: InfoMessage },
    StateMutation(Mutation),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FromPlayer {
    ActionChoice(usize),
}
