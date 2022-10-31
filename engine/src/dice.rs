use rand::prelude::*;

#[derive(Debug)]
pub struct Dice {
    rng: StdRng,
}

#[derive(Debug, Clone, Copy)]
pub struct Roll {
    pub d4: u8,
    pub d6: u8,
}

impl Dice {
    pub fn new() -> Self {
        Dice {
            rng: StdRng::from_entropy(),
        }
    }

    fn roll_d4(&mut self) -> u8 {
        self.rng.gen_range(1..=4)
    }

    fn roll_d6(&mut self) -> u8 {
        self.rng.gen_range(1..=6)
    }

    pub fn roll(&mut self) -> Roll {
        Roll {
            d4: self.roll_d4(),
            d6: self.roll_d6(),
        }
    }
}

impl Roll {
    pub fn sum(&self) -> usize {
        (self.d4 + self.d6).into()
    }

    pub fn diff(&self) -> usize {
        self.d4.abs_diff(self.d6).into()
    }
}
