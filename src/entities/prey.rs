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

use bevy::prelude::*;

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
    for _ in 0..conf::prey::COUNT {
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with_bundle((Prey::new(), Translation::random()));
    }
}

/// Prey is represented with a position vector.
#[derive(Debug)]
pub struct Prey {
    /// In what direction does the prey move and how fast.
    pub vel: Vec3,
}

impl Prey {
    fn new() -> Self {
        Self { vel: Vec3::zero() }
    }

    fn steer_towards(&self, v: Vec3) -> Vec3 {
        let v = v.normalized() * conf::prey::MAX_SPEED - self.vel;
        v.max(conf::prey::MAX_STEERING_FORCE)
    }
}

pub fn flock(time: Res<Time>, mut prey_query: Query<(Entity, &mut Prey, &Translation)>) {
    struct PreyData<'a> {
        entity: Entity,
        rf: Mut<'a, Prey>,
        pos: Vec3,
    }

    let prey_iter = &mut prey_query.iter();
    let mut prey = Vec::with_capacity(conf::prey::COUNT);
    for (entity, p, translation) in prey_iter {
        prey.push(PreyData {
            entity,
            rf: p,
            pos: **translation,
        });
    }

    #[derive(Default)]
    struct PreyUpdate {
        // Could be usize, but f32 saves us a conversion.
        flockmates: f32,
        heading_total: Vec3,
        center_total: Vec3,
    }

    for prey_index in 0..prey.len() {
        let iterated_prey = &prey[prey_index];
        let mut update = PreyUpdate::default();
        for other_index in 0..prey.len() {
            if prey_index == other_index {
                continue;
            }
            let other_prey = &prey[other_index];
            let offset = iterated_prey.pos - other_prey.pos;
            let sq_distance = offset.length_squared();

            // TODO: Const
            if sq_distance < conf::prey::VIEW {
                update.flockmates += 1.0;
                update.heading_total += other_prey.rf.vel;
                update.center_total += other_prey.pos;

                // TODO: Separation heading.
            }
        }

        if update.flockmates != 0.0 {
            let mut iterated_prey = &mut prey[prey_index];
            let offset_to_flock_center =
                (update.center_total / update.flockmates) - iterated_prey.pos;
            let cohesion_force = iterated_prey.rf.steer_towards(offset_to_flock_center);
        }
    }
}
