#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CharacterId(usize);

pub trait Character: core::fmt::Debug + Send + Sync {
    fn hit_point(&self) -> usize;
}

lazy_static::lazy_static! {
    static ref CHARACTERS: Vec<&'static dyn Character> = {
        vec![
            &Gregor,
            &Metamorphe,
            &Allie,
            &Bob,
            &Liche,
        ]
    };
}

#[derive(Debug)]
pub struct Characters;

impl Characters {
    pub fn from_id(id: CharacterId) -> &'static dyn Character {
        CHARACTERS[id.0]
    }

    #[cfg(feature = "game-logic")]
    pub fn generate(player_count: usize) -> Vec<CharacterId> {
        use rand::seq::IteratorRandom;
        assert!(player_count <= CHARACTERS.len());
        (0..CHARACTERS.len())
            .into_iter()
            .map(CharacterId)
            .choose_multiple(&mut rand::thread_rng(), player_count)
    }
}

#[derive(Debug)]
struct Gregor;
impl Character for Gregor {
    fn hit_point(&self) -> usize {
        14
    }
}

#[derive(Debug)]
struct Metamorphe;
impl Character for Metamorphe {
    fn hit_point(&self) -> usize {
        11
    }
}

#[derive(Debug)]
struct Allie;
impl Character for Allie {
    fn hit_point(&self) -> usize {
        8
    }
}

#[derive(Debug)]
struct Bob;
impl Character for Bob {
    fn hit_point(&self) -> usize {
        13
    }
}

#[derive(Debug)]
struct Liche;
impl Character for Liche {
    fn hit_point(&self) -> usize {
        14
    }
}
