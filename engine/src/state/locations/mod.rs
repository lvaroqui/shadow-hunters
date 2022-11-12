use std::fmt::Display;

mod cemetry;
mod church;
mod erstwhile_altar;
mod hermits_cabin;
mod underworld_gate;
mod weird_woods;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LocationId(usize);

pub trait Location: core::fmt::Debug + Send + Sync + Display {
    fn id(&self) -> LocationId;
    fn dice_numbers(&self) -> &'static [usize];
}

static LOCATIONS: [&'static dyn Location; 6] = [
    &hermits_cabin::HermitsCabin { id: LocationId(0) },
    &underworld_gate::UnderworldGate { id: LocationId(1) },
    &church::Church { id: LocationId(2) },
    &cemetry::Cemetry { id: LocationId(3) },
    &weird_woods::WeirdWoods { id: LocationId(4) },
    &erstwhile_altar::ErstwhileAltar { id: LocationId(5) },
];

#[derive(Debug)]
pub struct Locations {
    locations: [LocationId; 6],
}

impl Locations {
    #[cfg(test)]
    pub fn unshuffled() -> Self {
        Self {
            locations: LOCATIONS.map(|l| l.id()),
        }
    }

    #[cfg(feature = "game-logic")]
    pub fn generate() -> Self {
        use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
        let mut locations = LOCATIONS.map(|l| l.id());
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
        self.locations.iter().copied().map(|id| LOCATIONS[id.0])
    }

    pub fn in_group_iter(
        &self,
        id: LocationId,
    ) -> impl Iterator<Item = &'static dyn Location> + Clone + '_ {
        self.group_iter()
            .filter(move |g| g.iter().any(|&i| i == id))
            .flatten()
            .map(|i| LOCATIONS[i.0])
    }

    pub fn out_group_iter(
        &self,
        id: LocationId,
    ) -> impl Iterator<Item = &'static dyn Location> + Clone + '_ {
        self.group_iter()
            .filter(move |g| !g.iter().any(|&i| i == id))
            .flatten()
            .map(|i| LOCATIONS[i.0])
    }

    pub fn adjacent(&self, id: LocationId) -> [&'static dyn Location; 2] {
        let pos = self
            .locations
            .iter()
            .position(|&i| i == id)
            .expect("Invalid id") as isize;
        let len = self.locations.len() as isize;
        [
            self.locations[((pos - 1).rem_euclid(len)) as usize],
            self.locations[((pos + 1).rem_euclid(len)) as usize],
        ]
        .map(|i| LOCATIONS[i.0])
    }

    fn group_iter(&self) -> impl Iterator<Item = &[LocationId]> + Clone + '_ {
        self.locations.chunks(2)
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
