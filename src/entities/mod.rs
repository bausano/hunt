pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use bevy::prelude::*;

/// Iterates over all prey in the system and all predators. If a prey is close
/// to a predator, it checks whether the predator can see it or whether it's
/// been eaten. Also if
pub fn move_prey_and_mark_in_danger(
    mut prey_query: Query<&mut Prey>,
    mut predator_query: Query<&mut Predator>,
) {
    // This is an inefficient n*k loop, however for our purposes of running the
    // game with < 10 predators and < 1000 prey, we don't have to be so worried.
    for prey in &mut prey_query.iter() {
        let is_close_to_predator = false;
        for predator in &mut predator_query.iter() {
            let distance = predator.pos.distance2(prey);

            // If the prey is out of visibility radius, it has nothing to worry
            // about. Predator visibility radius is also larger than the one
            // of prey.
            if distance > conf::predator::VISIBILITY_RADIUS {
                continue;
            }

            // Prey is within grasp of a predator - eaten.
            if distance <= conf::predator::RADIUS {
                // TODO: Add point to predator.
                // TODO: Reset prey.
                continue;
            }

            // TODO: Add prey to predator's world view.

            if distance < conf::prey::VISIBILITY_RADIUS {
                is_close_to_predator = true;
            }
        }
    }
}
