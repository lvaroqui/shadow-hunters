use shared::PlayerId;

use crate::GameLogic;

use super::LocationBehavior;

#[derive(Debug)]
pub(crate) struct ErstwhileAltar;

#[async_trait::async_trait]
impl LocationBehavior for ErstwhileAltar {
    fn name() -> &'static str {
        "Erstwhile Altar"
    }

    fn dice_numbers() -> Vec<usize> {
        vec![10]
    }

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId) {}
}
