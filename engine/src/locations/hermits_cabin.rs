use shared::PlayerId;

use crate::GameLogic;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct HermitsCabin;

#[async_trait::async_trait]
impl LocationBehavior for HermitsCabin {
    fn name() -> &'static str {
        "Hermits Cabin"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![2, 3]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {}
}
