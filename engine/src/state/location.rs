use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocationId(usize);

#[derive(Debug)]
pub struct Locations {
    locations: [&'static dyn Location; 6],
}

static LOCATIONS: [&'static dyn Location; 6] = [
    &HermitsCabin { id: LocationId(0) },
    &UnderworldGate { id: LocationId(1) },
    &Church { id: LocationId(2) },
    &Cemetry { id: LocationId(3) },
    &WeirdWoods { id: LocationId(4) },
    &ErstwhileAltar { id: LocationId(5) },
];

impl Locations {
    pub fn new() -> Self {
        let mut locations = LOCATIONS;
        locations.shuffle(&mut StdRng::from_entropy());
        Self { locations }
    }

    pub fn location_from_dice_number(&self, dice_number: usize) -> &'static dyn Location {
        for l in LOCATIONS {
            if l.dice_numbers().contains(&dice_number) {
                return l;
            }
        }
        panic!("Provided number ({dice_number}) does not correspond to a location.");
    }

    pub fn location(&self, id: LocationId) -> &'static dyn Location {
        LOCATIONS[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static dyn Location> + '_ {
        self.locations.iter().copied()
    }
}

pub trait Location: core::fmt::Debug + Send + Sync {
    fn id(&self) -> LocationId;
    fn dice_numbers(&self) -> &'static [usize];
}

#[derive(Debug)]
struct HermitsCabin {
    id: LocationId,
}
impl Location for HermitsCabin {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[2, 3]
    }
}

#[derive(Debug)]
struct UnderworldGate {
    id: LocationId,
}
impl Location for UnderworldGate {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[4, 5]
    }
}

#[derive(Debug)]
struct Church {
    id: LocationId,
}
impl Location for Church {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[6]
    }
}

#[derive(Debug)]
struct Cemetry {
    id: LocationId,
}
impl Location for Cemetry {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[8]
    }
}

#[derive(Debug)]
struct WeirdWoods {
    id: LocationId,
}
impl Location for WeirdWoods {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[9]
    }
}

#[derive(Debug)]
struct ErstwhileAltar {
    id: LocationId,
}
impl Location for ErstwhileAltar {
    fn id(&self) -> LocationId {
        self.id
    }

    fn dice_numbers(&self) -> &'static [usize] {
        &[10]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_identifiers() {
        for (i, l) in LOCATIONS.iter().enumerate() {
            assert_eq!(l.id().0, i);
        }
    }
}
