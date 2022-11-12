use std::fmt::Display;

use super::Location;

use super::LocationId;

#[derive(Debug)]
pub(crate) struct Cemetry {
    pub(crate) id: LocationId,
}

impl Location for Cemetry {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[8]
    }
}

impl Display for Cemetry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cemetry")
    }
}
