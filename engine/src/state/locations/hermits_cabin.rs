use std::fmt::Display;

use super::Location;

use super::LocationId;

#[derive(Debug)]
pub(crate) struct HermitsCabin {
    pub(crate) id: LocationId,
}

impl Location for HermitsCabin {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[2, 3]
    }
}

impl Display for HermitsCabin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hermits Cabin")
    }
}
