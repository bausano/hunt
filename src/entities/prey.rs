//! Prey entities are simple creatures. They flock around a few individuals,
//! flock leaders, and when threatened, they run away from the closer predator
//! along the vector formed by predators position and its own position.
//!
//!
//!                         🐺
//!                        /
//!                      |/_
//!                      🐑<--------🐺---->🐑 . . . >
//!                     .
//!                    .
//!                  \._
//!
//! The catch is that the prey is faster than then predator. If the predators
//! are not organized, they won't get fed.

use {bevy::prelude::*, cgmath::MetricSpace};

use crate::prelude::*;

// Location of the prey sprite relative to the root.
const PREY_PNG: &str = "assets/prey.png";

/// Creates initial batch of prey.
pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server
        .load(PREY_PNG)
        .expect("Cannot load prey sprite");
    for _ in 0..conf::prey::INITIAL_COUNT {
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with_bundle(Prey::new());
    }
}

/// Prey is represented with a position vector [TODO: and a velocity vector].
#[derive(Debug)]
pub struct Prey;

impl Prey {
    // Randomly positions the prey in the map.
    fn new() -> (Self, Translation) {
        let rand_coord = || (rand::random::<usize>() % conf::MAP_SIZE) as f32;

        (Self, Translation::new(rand_coord(), rand_coord(), 0.0))
    }
}
