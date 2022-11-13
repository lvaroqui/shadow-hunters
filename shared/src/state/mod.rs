use std::fmt::Debug;

mod character;
mod location;
mod player;

pub use self::character::{Character, CharacterId};
pub use self::location::{Location, LocationId, Locations};
pub use self::player::{Player, PlayerId, PlayerStorage};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct State {
    players: Vec<PlayerStorage>,
    current_player: PlayerId,
    locations: Locations,
    characters: Vec<Character>,
}

impl State {
    pub fn new(
        players: Vec<PlayerStorage>,
        locations: Locations,
        first_player: PlayerId,
        characters: Vec<Character>,
    ) -> State {
        State {
            players,
            current_player: first_player,
            locations,
            characters,
        }
    }

    pub fn prepare_for_player(&self, player_id: PlayerId) -> Self {
        let mut res = self.clone();
        for p in &mut res.players {
            if p.id != player_id && !p.revealed {
                p.character = None;
            }
        }
        res
    }

    pub fn current_player(&self) -> Player {
        Player::new(self.current_player, self)
    }

    pub fn players(&self) -> impl ExactSizeIterator<Item = Player> + Clone {
        self.players.iter().map(|p| Player::new(p.id, self))
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
