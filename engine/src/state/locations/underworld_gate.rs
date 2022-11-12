use std::fmt::Display;

use super::Location;

use super::LocationId;

#[derive(Debug)]
pub(crate) struct UnderworldGate {
    pub(crate) id: LocationId,
}

impl Location for UnderworldGate {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[4, 5]
    }
}

impl Display for UnderworldGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Underworld Gate")
    }
}
