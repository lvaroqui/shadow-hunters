use shared::PlayerId;

use crate::GameLogic;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct UnderworldGate;

#[async_trait::async_trait]
impl LocationBehavior for UnderworldGate {
    fn name() -> &'static str {
        "Underworld Gate"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![4, 5]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {}
}
