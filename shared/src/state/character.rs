use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CharacterId(usize);

impl Index<CharacterId> for Vec<Character> {
    type Output = Character;

    fn index(&self, index: CharacterId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<CharacterId> for Vec<Character> {
    fn index_mut(&mut self, index: CharacterId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    id: CharacterId,
    name: String,
    hit_points: usize,
}

impl Character {
    pub fn hit_points(&self) -> usize {
        self.hit_points
    }
}
