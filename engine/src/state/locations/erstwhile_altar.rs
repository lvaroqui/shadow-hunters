use super::Location;
use super::LocationId;
use std::fmt::Display;

#[derive(Debug)]
pub(crate) struct ErstwhileAltar {
    pub(crate) id: LocationId,
}

impl Location for ErstwhileAltar {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[10]
    }
}

impl Display for ErstwhileAltar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ertswhile Altar")
    }
}
