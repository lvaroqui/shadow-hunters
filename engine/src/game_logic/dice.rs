use rand::prelude::*;
use shared::Roll;

#[derive(Debug)]
pub struct Dice {
    rng: StdRng,
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
