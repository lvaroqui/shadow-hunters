use std::fmt::Display;

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
    #[cfg(test)]
    pub fn unshuffled() -> Self {
        Self {
            locations: LOCATIONS,
        }
    }

    pub fn generate() -> Self {
        let mut locations = LOCATIONS;
        locations.shuffle(&mut StdRng::from_entropy());
        Self { locations }
    }

    pub fn from_dice_number(dice_number: usize) -> &'static dyn Location {
        for l in LOCATIONS {
            if l.dice_numbers().contains(&dice_number) {
                return l;
            }
        }
        panic!("Provided number ({dice_number}) does not correspond to a location.");
    }

    pub fn from_id(id: LocationId) -> &'static dyn Location {
        LOCATIONS[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static dyn Location> + Clone + '_ {
        self.locations.iter().copied()
    }

    pub fn in_group_iter(
        &self,
        id: LocationId,
    ) -> impl Iterator<Item = &'static dyn Location> + Clone + '_ {
        self.group_iter()
            .filter(move |g| g.iter().any(|l| l.id() == id))
            .flatten()
            .copied()
    }

    pub fn out_group_iter(
        &self,
        id: LocationId,
    ) -> impl Iterator<Item = &'static dyn Location> + Clone + '_ {
        self.group_iter()
            .filter(move |g| !g.iter().any(|l| l.id() == id))
            .flatten()
            .copied()
    }

    pub fn adjacent(&self, id: LocationId) -> [&'static dyn Location; 2] {
        let pos = self
            .locations
            .iter()
            .position(|l| l.id() == id)
            .expect("Invalid id") as isize;
        let len = self.locations.len() as isize;
        [
            self.locations[((pos - 1).rem_euclid(len)) as usize],
            self.locations[((pos + 1).rem_euclid(len)) as usize],
        ]
    }

    fn group_iter(&self) -> impl Iterator<Item = &[&'static dyn Location]> + Clone + '_ {
        self.locations.chunks(2)
    }
}

pub trait Location: core::fmt::Debug + Send + Sync + Display {
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
impl Display for HermitsCabin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hermits Cabin")
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
impl Display for UnderworldGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Underworld Gate")
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
impl Display for Church {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Church")
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
impl Display for Cemetry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cemetry")
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
impl Display for WeirdWoods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Weird Woods")
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
impl Display for ErstwhileAltar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ertswhile Altar")
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

    #[test]
    fn in_group_iter() {
        let locations = Locations::unshuffled();
        assert!(locations.in_group_iter(LocationId(0)).map(|l| l.id()).eq([
            LocationId(0),
            LocationId(1)
        ]
        .into_iter()));
        assert!(locations.in_group_iter(LocationId(1)).map(|l| l.id()).eq([
            LocationId(0),
            LocationId(1)
        ]
        .into_iter()));
        assert!(locations.in_group_iter(LocationId(3)).map(|l| l.id()).eq([
            LocationId(2),
            LocationId(3)
        ]
        .into_iter()));
    }

    #[test]
    fn out_group_iter() {
        let locations = Locations::unshuffled();
        assert!(locations.out_group_iter(LocationId(0)).map(|l| l.id()).eq([
            LocationId(2),
            LocationId(3),
            LocationId(4),
            LocationId(5)
        ]
        .into_iter()));
        assert!(locations.out_group_iter(LocationId(1)).map(|l| l.id()).eq([
            LocationId(2),
            LocationId(3),
            LocationId(4),
            LocationId(5)
        ]
        .into_iter()));
        assert!(locations.out_group_iter(LocationId(3)).map(|l| l.id()).eq([
            LocationId(0),
            LocationId(1),
            LocationId(4),
            LocationId(5)
        ]
        .into_iter()));
    }

    #[test]
    fn adjacent() {
        let locations = Locations::unshuffled();
        assert_eq!(
            locations.adjacent(LocationId(0)).map(|l| l.id()),
            [LocationId(5), LocationId(1)]
        );
        assert_eq!(
            locations.adjacent(LocationId(1)).map(|l| l.id()),
            [LocationId(0), LocationId(2)]
        );
        assert_eq!(
            locations.adjacent(LocationId(4)).map(|l| l.id()),
            [LocationId(3), LocationId(5)]
        );
        assert_eq!(
            locations.adjacent(LocationId(5)).map(|l| l.id()),
            [LocationId(4), LocationId(0)]
        );
    }
}
