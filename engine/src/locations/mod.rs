mod cemetry;
mod church;
mod erstwhile_altar;
mod hermits_cabin;
mod underworld_gate;
mod weird_woods;

use shared::PlayerId;

use crate::GameLogic;

pub(crate) fn location_behaviors() -> [&'static dyn LocationBehavior; 6] {
    [
        &cemetry::Cemetry,
        &church::Church,
        &erstwhile_altar::ErstwhileAltar,
        &hermits_cabin::HermitsCabin,
        &underworld_gate::UnderworldGate,
        &weird_woods::WeirdWoods,
    ]
}

#[async_trait::async_trait]
pub(crate) trait LocationBehavior: Send + Sync {
    fn name(&self) -> &'static str;
    fn dice_numbers(&self) -> Vec<usize>;

    async fn handle(&self, game_logic: &mut GameLogic, player_id: PlayerId);
}
