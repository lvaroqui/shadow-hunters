#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LocationId(usize);

impl From<LocationId> for usize {
    fn from(v: LocationId) -> Self {
        v.0
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    id: LocationId,
    name: String,
    dice_numbers: Vec<usize>,
}

impl Location {
    pub fn new(id: usize, name: String, dice_numbers: Vec<usize>) -> Self {
        Self {
            id: LocationId(id),
            name,
            dice_numbers,
        }
    }

    pub fn id(&self) -> LocationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dice_numbers(&self) -> &[usize] {
        &self.dice_numbers
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Locations {
    locations: [Location; 6],
    layout: [LocationId; 6],
}

impl Locations {
    pub fn new(locations: [Location; 6], layout: [LocationId; 6]) -> Self {
        for (i, l) in locations.iter().enumerate() {
            assert!(l.id.0 == i)
        }
        Self { locations, layout }
    }

    pub fn from_dice_number(&self, dice_number: usize) -> &Location {
        self.locations
            .iter()
            .find(|l| l.dice_numbers.contains(&dice_number))
            .expect("No location corresponds to provided number")
    }

    pub fn from_id(&self, id: LocationId) -> &Location {
        &self.locations[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Location> + Clone + '_ {
        self.locations.iter()
    }

    pub fn in_group_iter(&self, id: LocationId) -> impl Iterator<Item = &Location> + Clone + '_ {
        self.groups()
            .into_iter()
            .filter(move |g| g.iter().any(|&i| i == id))
            .flatten()
            .map(|i| &self.locations[i.0])
    }

    pub fn out_group_iter(&self, id: LocationId) -> impl Iterator<Item = &Location> + Clone + '_ {
        self.groups()
            .into_iter()
            .filter(move |g| !g.iter().any(|&i| i == id))
            .flatten()
            .map(|i| &self.locations[i.0])
    }

    pub fn adjacent(&self, id: LocationId) -> [&Location; 2] {
        let pos = self
            .layout
            .iter()
            .position(|&i| i == id)
            .expect("Invalid id") as isize;
        let len = self.locations.len() as isize;
        [
            self.layout[((pos - 1).rem_euclid(len)) as usize],
            self.layout[((pos + 1).rem_euclid(len)) as usize],
        ]
        .map(|i| &self.locations[i.0])
    }

    fn groups(&self) -> [[LocationId; 2]; 3] {
        [
            [self.layout[0], self.layout[1]],
            [self.layout[2], self.layout[3]],
            [self.layout[4], self.layout[5]],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_unshuffled_locations() -> Locations {
        let layout: [LocationId; 6] = (0..6)
            .into_iter()
            .map(LocationId)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Locations::new(
            layout.map(|id| Location {
                id,
                name: format!("{}", id.0),
                dice_numbers: vec![id.0],
            }),
            layout,
        )
    }

    #[test]
    fn in_group_iter() {
        let locations = new_unshuffled_locations();
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
        let locations = new_unshuffled_locations();
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
        let locations = new_unshuffled_locations();
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
