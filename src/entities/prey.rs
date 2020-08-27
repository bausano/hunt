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

use crate::prelude::*;

/// Prey is represented with a position vector.
#[derive(Debug)]
pub struct Prey {
    /// In what direction does the prey move and how fast.
    pub vel: Vec3,
}

/// Calculation of flocking behavior is expensive. We undergo this calculation
/// only few times a second.
pub struct FlockUpdateTimer(Timer);

/// Creates initial batch of prey.
pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server
        .load(conf::prey::ICON)
        .expect("Cannot load prey sprite");
    for _ in 0..conf::prey::COUNT {
        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with_bundle((
                Prey::new(),
                Translation::random(),
                Rotation::default(),
            ));
    }
}

impl Default for FlockUpdateTimer {
    fn default() -> Self {
        Self(Timer::new(conf::prey::RECALCULATE_FLOCKING, true))
    }
}

impl Prey {
    fn new() -> Self {
        Self { vel: Vec3::zero() }
    }

    fn steer_towards(&self, v: Vec3) -> Vec3 {
        let v = v.normalize() * conf::prey::MAX_SPEED - self.vel;
        v.min(Vec3::splat(conf::prey::MAX_STEERING_FORCE))
    }
}

/// Moves the prey based on its velocity vector and rotates it in the direction
/// of the vel.
pub fn nudge(
    time: Res<Time>,
    mut prey_query: Query<(&Prey, &mut Translation, &mut Rotation)>,
) {
    for (prey, mut pos, mut rot) in &mut prey_query.iter() {
        super::nudge_entity(&time, prey.vel, &mut pos, &mut rot);
    }
}

/// Simulates flocking behavior.
/// Based on [source code][seb-boids] of an amazing [video][seb-vid], which is
/// in turn based on [this paper][flocking-paper].
///
/// [seb-boids]: https://github.com/SebLague/Boids/tree/master
/// [seb-vid]: https://www.youtube.com/watch?v=bqtqltqcQhw
/// [flocking-paper]: http://www.cs.toronto.edu/~dt/siggraph97-course/cwr87
pub fn flocking_behavior(
    time: Res<Time>,
    mut timer: ResMut<FlockUpdateTimer>,
    mut prey_query: Query<(&mut Prey, &Translation)>,
) {
    // Ticks and checks that enough time has passed and its time to update the
    // flocking again.
    timer.0.tick(time.delta_seconds);
    if !timer.0.finished {
        return;
    }

    struct PreyData<'a> {
        rf: Mut<'a, Prey>,
        pos: Vec3,
    }

    // We collect all prey into a vec since we need to run a loop which
    // calculates update to velocity vec with respect to all other prey in the
    // game. This is not currently possible with the iterator.
    let prey_iter = &mut prey_query.iter();
    let mut prey = Vec::with_capacity(conf::prey::COUNT);
    for (p, translation) in prey_iter {
        prey.push(PreyData {
            rf: p,
            pos: **translation,
        });
    }

    for prey_index in 0..prey.len() {
        let iterated_prey = &prey[prey_index];

        // How many other prey is nearby.
        let mut flockmates = 0;
        // Sums all heading vectors of all nearby flockmates.
        let mut heading_dir = Vec3::zero();
        // Sums all position vectors of all nearby flockmates.
        let mut center_total = Vec3::zero();
        // We calculate in which direction should we move to avoid other prey.
        // We don't want prey to be too close to one another.
        let mut separation_dir = Vec3::zero();

        for other_index in 0..prey.len() {
            if prey_index == other_index {
                continue;
            }
            let other_prey = &prey[other_index];
            let offset = iterated_prey.pos - other_prey.pos;
            let sq_distance = offset.length_squared();

            if sq_distance < conf::prey::VIEW_RADIUS.powi(2) {
                flockmates += 1;
                heading_dir += other_prey.rf.vel;
                center_total += other_prey.pos;

                // If prey is too close to each other, try change its direction
                // so that they don't bump.
                if sq_distance < conf::prey::AVOID_RADIUS.powi(2) {
                    separation_dir += offset / (sq_distance + f32::EPSILON);
                }
            }
        }

        let iterated_prey = &mut prey[prey_index];
        let mut acc = Vec3::zero();

        // If the prey gets too close to a wall, we push it out.
        if let Some(f) = wall_repelling_force(iterated_prey.pos) {
            acc += iterated_prey.rf.steer_towards(f)
                * conf::prey::weights::WALL_REPELLING_FORCE;
        }

        if flockmates > 0 {
            let cohesion_force = {
                let offset_to_flock_center =
                    (center_total / flockmates as f32) - iterated_prey.pos;
                iterated_prey.rf.steer_towards(offset_to_flock_center)
            };
            let alignment_force = iterated_prey.rf.steer_towards(heading_dir);

            acc += alignment_force * conf::prey::weights::ALIGNMENT_FORCE;
            acc += cohesion_force * conf::prey::weights::COHESION_FORCE;

            if separation_dir != Vec3::zero() {
                let separation_force =
                    iterated_prey.rf.steer_towards(separation_dir);
                acc += separation_force * conf::prey::weights::SEPARATION_FORCE;
            }
        }

        // Updates the velocity vector of the prey.
        let vel = &mut iterated_prey.rf.vel;
        let dv =
            acc * conf::prey::RECALCULATE_FLOCKING.as_millis() as f32 / 1000.0;
        *vel += dv;
        let speed = vel.length();
        if speed > 0.0 {
            let direction = *vel / speed;
            // Unfortunately clamp is still in nightly.
            let speed =
                speed.max(conf::prey::MIN_SPEED).min(conf::prey::MAX_SPEED);
            *vel = direction * speed;
        }
    }
}

// If the prey is too close to the wall, it attempts to run away from it.
fn wall_repelling_force(pos: Vec3) -> Option<Vec3> {
    let map_10p = conf::MAP_SIZE / 10.0;
    let x = if pos.x() < map_10p {
        Some(conf::prey::MAX_SPEED)
    } else if pos.x() > conf::MAP_SIZE - map_10p {
        Some(-conf::prey::MAX_SPEED)
    } else {
        None
    };
    let y = if pos.y() < map_10p {
        Some(conf::prey::MAX_SPEED)
    } else if pos.y() > conf::MAP_SIZE - map_10p {
        Some(-conf::prey::MAX_SPEED)
    } else {
        None
    };

    if x.is_some() || y.is_some() {
        Some(Vec3::new(x.unwrap_or(0.0), y.unwrap_or(0.0), 0.0))
    } else {
        None
    }
}
