pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use bevy::prelude::*;

/// Iterates over all prey in the system. If the prey is
pub fn move_prey_and_mark_in_danger(
    mut prey_query: Query<&mut Prey>,
    mut predator_query: Query<&mut Predator>,
) {
    // This is an inefficient n*k loop, however for our purposes of running the
    // game with < 10 predators and < 1000 prey, we don't have to be so worried.
    for prey in &mut prey_query.iter() {
        for predator in &mut predator_query.iter() {
            //
        }
    }
}
