use std::ops::{Index, IndexMut};

use crate::{CharacterId, LocationId};

use super::{Character, Location, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerId(usize);

impl PlayerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl Index<PlayerId> for Vec<PlayerStorage> {
    type Output = PlayerStorage;

    fn index(&self, index: PlayerId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<PlayerId> for Vec<PlayerStorage> {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerStorage {
    pub(crate) id: PlayerId,
    pub(crate) damage: usize,
    pub(crate) location: LocationId,
    pub(crate) revealed: bool,
    pub(crate) character: Option<CharacterId>,
}

pub struct Player<'a> {
    id: PlayerId,
    state: &'a State,
}

impl<'a> Player<'a> {
    pub(crate) fn new(id: PlayerId, state: &'a State) -> Self {
        Self { id, state }
    }

    pub(crate) fn storage(&self) -> &PlayerStorage {
        &self.state.players[self.id]
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn damage(&self) -> usize {
        self.storage().damage
    }

    pub fn location(&self) -> &'a Location {
        self.state.locations().from_id(self.storage().location)
    }

    pub fn revealed(&self) -> bool {
        self.storage().revealed
    }

    pub fn character(&self) -> Option<&Character> {
        self.storage().character.map(|c| &self.state.characters[c])
    }

    pub fn is_alive(&self) -> bool {
        if let Some(c) = self.character() {
            self.damage() < c.hit_points()
        } else {
            true
        }
    }
}
