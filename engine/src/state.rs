use std::fmt::Debug;

use crate::PlayerId;

pub trait Character: Debug + Send + Sync {}

#[derive(Debug)]
pub struct Player {
    id: PlayerId,
    damage: usize,
    location: usize,
    character: Option<&'static dyn Character>,
}

#[derive(Debug)]
pub struct State {
    players: Vec<Player>,
}
impl State {
    pub(crate) fn new(player_count: usize) -> State {
        State {
            players: (0..player_count)
                .into_iter()
                .map(|id| Player {
                    id: PlayerId(id),
                    damage: 0,
                    location: 0,
                    character: None,
                })
                .collect(),
        }
    }
}
