use shared::PlayerId;

use crate::GameLogic;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct Church;

#[async_trait::async_trait]
impl LocationBehavior for Church {
    fn name() -> &'static str {
        "Church"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![6]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {}
}
