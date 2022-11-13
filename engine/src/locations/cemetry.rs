use shared::PlayerId;

use crate::GameLogic;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct Cemetry;

#[async_trait::async_trait]
impl LocationBehavior for Cemetry {
    fn name() -> &'static str {
        "Cemetry"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![6]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {}
}
