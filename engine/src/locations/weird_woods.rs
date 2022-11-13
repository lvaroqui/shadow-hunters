use crate::game_logic::GameLogic;
use shared::PlayerId;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct WeirdWoods;

#[async_trait::async_trait]
impl LocationBehavior for WeirdWoods {
    fn name() -> &'static str {
        "Weird Woods"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![9]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {
        use shared::state::Mutation;

        let choices = game_logic.state.players().map(|p| {
            (
                shared::Action::DamagePlayer(p.id(), Some(2)),
                Mutation::DamagePlayer(p.id(), 2),
            )
        });
        let choices = choices.chain(game_logic.state.players().map(|p| {
            (
                shared::Action::HealPlayer(p.id(), Some(1)),
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
