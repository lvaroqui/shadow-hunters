use crate::Location;

pub(crate) const LOCATIONS: [Location; 6] = [
    Location {
        name: "Vision",
        numbers: &[2, 3],
    },
    Location {
        name: "Any",
        numbers: &[4, 5],
    },
    Location {
        name: "White",
        numbers: &[6],
    },
    Location {
        name: "Black",
        numbers: &[8],
    },
    Location {
        name: "HauntedForrest",
        numbers: &[9],
    },
    Location {
        name: "Steal",
        numbers: &[10],
    },
];

pub(crate) fn location_by_number(number: usize) -> (usize, &'static Location) {
    LOCATIONS
        .iter()
        .enumerate()
        .find(|(_, l)| l.numbers.contains(&number))
        .expect("Location should exists")
}
