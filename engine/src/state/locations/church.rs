use std::fmt::Display;

use super::Location;
use super::LocationId;

#[derive(Debug)]
pub(crate) struct Church {
    pub(crate) id: LocationId,
}

impl Location for Church {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[6]
    }
}

impl Display for Church {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Church")
    }
}
