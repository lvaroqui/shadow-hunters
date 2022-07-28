use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::{characters, Player, PlayerId};

#[derive(Debug)]
pub struct State {
    players: HashMap<PlayerId, Player>,
    location_positions: [usize; 6],
    turn_order: Vec<PlayerId>,
    current_player_index: usize,
}

#[derive(Debug)]
pub(crate) struct PlayerInfo {
    id: PlayerId,
    name: String,
    color: String,
}

impl PlayerInfo {
    pub(crate) fn new(id: PlayerId, name: String, color: String) -> Self {
        Self { id, name, color }
    }
}

impl State {
    pub(crate) fn new<I: IntoIterator<Item = PlayerInfo>>(players: I) -> Self {
        let players: HashMap<_, _> = players
            .into_iter()
            .map(|p| {
                (
                    p.id,
                    Player::new(p.id, p.name, p.color, &characters::GREGOR),
                )
            })
            .collect();

        let turn_order = players.iter().map(|(id, _)| *id).collect();

        let mut ret = Self {
            players,
            turn_order,
            location_positions: [0, 1, 2, 3, 4, 5],
            current_player_index: 0,
        };

        // Shuffle turn orders, locations and decks
        let mut rng = rand::thread_rng();
        ret.turn_order.shuffle(&mut rng);
        ret.location_positions.shuffle(&mut rng);

        ret
    }

    pub(crate) fn player(&self, player_id: PlayerId) -> &Player {
        &self.players[&player_id]
    }

    pub(crate) fn current_player(&self) -> &Player {
        self.players
            .get(&self.turn_order[self.current_player_index])
            .unwrap()
    }

    pub(crate) fn next_player(&mut self) {
        self.current_player_index += 1;
        self.current_player_index %= self.turn_order.len();
    }

    pub(crate) fn move_player(&mut self, player_id: PlayerId, location: usize) {
        self.players.get_mut(&player_id).unwrap().location = location;
    }
}
