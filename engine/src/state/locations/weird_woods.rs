use std::fmt::Display;

use super::Location;

use super::LocationId;

#[derive(Debug)]
pub(crate) struct WeirdWoods {
    pub(crate) id: LocationId,
}

impl Location for WeirdWoods {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[9]
    }
}

impl Display for WeirdWoods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weird Woods")
    }
}
