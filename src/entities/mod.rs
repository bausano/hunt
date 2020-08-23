pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use bevy::prelude::*;

fn move_prey(prey: Query<(Prey, &mut Translation)>) {
    for (_, translation) in prey {
        //
    }
}
