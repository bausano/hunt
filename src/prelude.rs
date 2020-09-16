pub use bevy::prelude::*;

pub use crate::components;
pub use crate::conf;
pub use crate::resources;

use std::error::Error;

// /// This will work just fine for us, there isn't need for custom error.
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait InstantiateRandom {
    fn random() -> Self;
}

impl InstantiateRandom for Translation {
    fn random() -> Self {
        let rand_coord = || rand::random::<usize>() as f32 % conf::MAP_SIZE;
        Self::new(rand_coord(), rand_coord(), 0.0)
    }
}

pub trait Vec3Ext {
    fn is_zero(self) -> bool;

    // Calculates euclidean distance in the xy plane.
    fn distance2(self, other: Self) -> f32;

    // Creates a new perpendicular vector.
    fn perpendicular(self) -> Self;
}

impl Vec3Ext for Vec3 {
    fn is_zero(self) -> bool {
        // We don't care about z as we play in 2D.
        self.x() == 0.0 && self.y() == 0.0
    }

    fn distance2(self, other: Self) -> f32 {
        (self - other).length()
    }

    fn perpendicular(self) -> Self {
        // We don't care about z as we play in 2D.
        Self::new(self.y(), self.x() * -1.0, 0.0)
    }
}
