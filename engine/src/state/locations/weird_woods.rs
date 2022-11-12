use std::fmt::Display;

use super::{Location, LocationId, PlayerId};
#[cfg(feature = "game-logic")]
use crate::game_logic::GameLogic;
#[derive(Debug)]
pub(crate) struct WeirdWoods {
    pub(crate) id: LocationId,
}

#[async_trait::async_trait]
impl Location for WeirdWoods {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[9]
    }

    #[cfg(feature = "game-logic")]
    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {
        use crate::state::Mutation;

        let choices = game_logic.state.players().iter().map(|p| {
            (
                crate::Action::DamagePlayer(p.id(), Some(2)),
                Mutation::DamagePlayer(p.id(), 2),
            )
        });
        let choices = choices.chain(game_logic.state.players().iter().map(|p| {
            (
                crate::Action::HealPlayer(p.id(), Some(1)),
                Mutation::HealPlayer(p.id(), 1),
            )
        }));
        let mutation = game_logic
            .message_channel
            .request_action_map(player_id, choices)
            .await
            .unwrap();
        game_logic.mutate_state(mutation).await.unwrap();
    }
}

impl Display for WeirdWoods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weird Woods")
    }
}
