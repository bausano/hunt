pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use crate::prelude::*;

/// Iterates over all prey in the system and all predators. If a prey is close
/// to a predator, it checks whether the predator can see it or whether it's
/// been eaten.
pub fn mark_prey_in_danger(
    mut prey_query: Query<(&mut Prey, &mut Translation)>,
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
    for (mut predator, pos) in predator_iter {
        predators.push(PredatorData {
            rf: predator,
            pos: **pos,
        });
    }

    // This is an inefficient n*k loop, however for our purposes of running the
    // game with well < 10 predators and < 1000 prey it's ok.
    for (mut prey, mut pos) in &mut prey_query.iter() {
        // Collects relationships prey has towards predators.
        let mut predators_which_eat_me = vec![];
        let mut predators_which_see_me = vec![];
        let mut predators_which_i_see = vec![];

        // Finds all predators which have one of those relationships with the
        // prey.
        for (predator_index, predator) in predators.iter().enumerate() {
            let distance = predator.pos.distance2(**pos);

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
                    predators_which_i_see.push(predator_index);
                }
            }
        }

        if !predators_which_eat_me.is_empty() {
            for predator_index in predators_which_eat_me {
                predators.get_mut(predator_index).map(|p| p.rf.score());
            }

        // TODO: Kill the prey. We can have respawning procedure impl later.
        } else {
            if !predators_which_i_see.is_empty() {
                // TODO: Update the prey's velocity.
            }

            for predator_index in predators_which_see_me {
                predators
                    .get_mut(predator_index)
                    .map(|p| p.rf.spot_prey(**pos));
            }
        }
    }
}

// Translates prey or prey based on velocity vector, and also rotates it in
// the direction of the vector.
// TODO: Much nicer design would be to have velocity as another property of
// each entity, and update the translation solely based on velocity. This would
// make this function work for both prey and predator.
fn nudge_entity(
    time: &Time,
    vel: Vec3,
    pos: &mut Translation,
    rot: &mut Rotation,
) {
    let pos_vec = **pos + vel * time.delta_seconds;
    let pos_vec = pos_vec
        .truncate()
        .min(Vec2::splat(conf::MAP_SIZE as f32))
        .max(Vec2::zero())
        .extend(0.0);
    *pos = pos_vec.into();

    // If the velocity vector is not zero vector, rotate the entity in the
    // direction of its velocity.
    if !vel.cmpeq(Vec3::zero()).all() {
        let vel_norm = vel.normalize();
        // Normalized velocity, find the angle based on the size of the
        // x component, and then shift it if the y component is negative.
        let new_rot = vel_norm.x().acos() * vel_norm.y().signum();
        *rot = Rotation::from_rotation_z(new_rot);
    }
}
