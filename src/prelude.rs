pub use bevy::prelude::*;

pub use crate::conf;

use std::error::Error;

/// This will work just fine for us, there isn't need for custom error.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait InstantiateRandom {
    fn random() -> Self;
}

impl InstantiateRandom for Translation {
    fn random() -> Self {
        let rand_coord = || (rand::random::<usize>() % conf::MAP_SIZE) as f32;
        Self::new(rand_coord(), rand_coord(), 0.0)
    }
}
