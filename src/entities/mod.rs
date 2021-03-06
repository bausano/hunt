//! Prey and predators, as the name suggests, is a game where predator hunt prey.
//! Is a predator gets to certain distance of prey, they can see it. If the
//! predator gets closer, the prey can see them. If they get very close, they
//! eat the prey.
//!
//! Prey is faster than predator, hence predators must cooperate.
//!
//! ```text
//!                         🐺
//!                        /
//!                      |/_
//!                      🐑<--------🐺---->🐑 . . . >
//!                     .
//!                    .
//!                  \._
//! ```

pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use crate::{components::Velocity, prelude::*};

/// Iterates over all prey in the system and all predators. If a prey is close
/// to a predator, it checks whether the predator can see it or whether it's
/// been eaten.
pub fn interact(
    mut prey_query: Query<(&mut Translation, &mut Velocity, &Prey)>,
    mut predator_query: Query<(&mut Predator, &Translation)>,
) {
    struct PredatorData<'a> {
        rf: Mut<'a, Predator>,
        pos: Vec3,
    }

    // We collect all predator into a vec since we need to refer to it
    // retrospectively. I wish we could collect it, but bevy has some weird type
    // issues.
    let predator_iter = &mut predator_query.iter();
    let mut predators: Vec<_> = Vec::new();
    for (predator, pos) in predator_iter {
        predators.push(PredatorData {
            rf: predator,
            pos: **pos,
        });
    }

    // This is an inefficient n*k loop, however for our purposes of running the
    // game with well < 10 predators and < 1000 prey it's ok.
    for (mut prey_pos, mut prey_vel, ..) in &mut prey_query.iter() {
        // Collects relationships prey has towards predators. We store indexes
        // in the first two arrays. Indexes point to the predator position in
        // the `predators` array.
        let mut predators_which_eat_me = Vec::new();
        let mut predators_which_see_me = Vec::new();
        // In this array we store the predator position and the distance between
        // it and the prey.
        let mut predators_which_i_see: Vec<(Vec3, f32)> = Vec::new();

        // Finds all predators which have one of those relationships with the
        // prey.
        for (predator_index, predator) in predators.iter().enumerate() {
            let distance = predator.pos.distance2(**prey_pos);

            // If the prey is out of visibility radius, it has nothing to worry
            // about. Predator visibility radius is also larger than the one
            // of prey.
            if distance > conf::predator::VIEW_RADIUS {
                continue;
            }

            if distance <= conf::predator::STRIKE_RADIUS {
                // Prey is within a grasp of a predator - eaten.
                predators_which_eat_me.push(predator_index);
            } else {
                predators_which_see_me.push(predator_index);

                // The prey always has lower or same visibility radius.
                if distance < conf::prey::VIEW_RADIUS {
                    predators_which_i_see.push((predator.pos, distance));
                }
            }
        }

        if !predators_which_eat_me.is_empty() {
            for predator_index in predators_which_eat_me {
                if let Some(predator) = predators.get_mut(predator_index) {
                    predator.rf.score();
                }
            }

            // Re-spawns the prey at random place somewhere else. This works ok
            // if the map is very large and there aren't that many predators.
            // Otherwise prey will spawn straight into the predators.
            *prey_pos = Translation::random();
        } else {
            if !predators_which_i_see.is_empty() {
                // Calculates difference between the prey and each predator,
                // which results in a sum of vectors directed opposite to each
                // predators position.
                let escape_force = predators_which_i_see.into_iter().fold(
                    Vec3::zero(),
                    |acc, (predator_pos, distance)| {
                        acc + (**prey_pos - predator_pos) / distance
                    },
                ) * conf::prey::weights::ESCAPE_FORCE;

                let acc = prey::steer_towards(*prey_vel, escape_force);
                prey_vel.apply_acceleration(acc, prey::clamp_speed);
            }

            for predator_index in predators_which_see_me {
                if let Some(predator) = predators.get_mut(predator_index) {
                    predator.rf.spot_prey(**prey_pos);
                }
            }
        }
    }
}

// Translates prey or predator based on velocity vector, and also rotates it in
// the direction of the vector.
//
// The game space is topological torus in 2 dimensions.
pub fn nudge(
    time: Res<Time>,
    mut entity_query: Query<(&mut Velocity, &mut Translation, &mut Rotation)>,
) {
    for (mut vel, mut pos, mut rot) in &mut entity_query.iter() {
        let mut pos_vec = **pos + **vel * time.delta_seconds;

        if pos_vec.x() > conf::MAP_SIZE {
            pos_vec.set_x(pos_vec.x() - conf::MAP_SIZE);
        } else if pos_vec.x() < 0.0 {
            pos_vec.set_x(pos_vec.x() + conf::MAP_SIZE);
        }

        if pos_vec.y() > conf::MAP_SIZE {
            pos_vec.set_y(pos_vec.y() - conf::MAP_SIZE);
        } else if pos_vec.y() < 0.0 {
            pos_vec.set_y(pos_vec.y() + conf::MAP_SIZE);
        }

        pos_vec.set_z(0.0);
        *pos = pos_vec.into();

        // If the velocity vector is not zero vector, rotate the entity in the
        // direction of its velocity.
        if !vel.is_zero() {
            let vel_norm = vel.normalize();
            // Normalized velocity, find the angle based on the size of the
            // x component, and then shift it if the y component is negative.
            let new_rot = vel_norm.x().acos() * vel_norm.y().signum();
            *rot = Rotation::from_rotation_z(new_rot);

            // Also makes the velocity a little bit smaller. Acts as a
            // "friction".
            **vel *= 1.0 - (time.delta_seconds / conf::predator::FRICTION);
        }
    }
}
