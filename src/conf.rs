//! Contains global game configuration.

/// The map is a square where a = MAP_SIZE.
pub const MAP_SIZE: f32 = 1000.0;

pub mod prey {
    //! Configuration for prey entity.

    use std::time::Duration;

    /// Location of the prey sprite relative to the root.
    /// TODO: Consider compiling the asset into the binary.
    pub const ICON: &str = "assets/prey.png";

    /// Avoids bumping into another prey by repelling each other if they're too
    /// close.
    pub const AVOID_RADIUS: f32 = 80.0;

    /// This many prey will be spawned when the game starts and as predators
    /// eat prey, we respawn it in such a manner that there's approximately
    /// this much prey throughout the game.
    pub const COUNT: usize = 50;

    /// How many pixels per tick can a prey move. Make sure that this settings
    /// is always larger than the predators max speed.
    pub const MAX_SPEED: f32 = 350.0;

    /// We want the prey to be always on the move.
    pub const MIN_SPEED: f32 = 100.0;

    /// Puts upper bounds on how much can a prey shift its position.
    pub const MAX_STEERING_FORCE: f32 = 250.0;

    /// How much around itself does a prey see.
    pub const VIEW_RADIUS: f32 = 150.0;

    /// Calculating the flocking behavior is expensive. Let's do it only every
    /// now and then and cache the direction vec.
    pub const RECALCULATE_FLOCKING: Duration = Duration::from_millis(50);

    pub mod weights {
        pub const WALL_REPELLING_FORCE: f32 = 1.0;
        pub const ALIGNMENT_FORCE: f32 = 1.0;
        pub const SEPARATION_FORCE: f32 = 1.5;
        pub const COHESION_FORCE: f32 = 1.0;
    }
}

pub mod predator {
    //! Configuration for predator entity.

    /// Location of the prey sprite relative to the root.
    /// TODO: Consider compiling the asset into the binary.
    pub const ICON: &str = "assets/predator.png";

    /// It's important that the max speed is less than the preys.
    pub const MAX_SPEED: f32 = 300.0;

    /// From what distance do predators spot prey. It should be higher or equal
    /// to prey's view radius.
    pub const VIEW_RADIUS: f32 = 250.0;

    /// If predator gets at least this close to a prey, it eats it.
    pub const STRIKE_RADIUS: f32 = 30.0;

    /// How many seconds does it take for the predator to go from max speed
    /// velocity to 0.
    pub const FRICTION: f32 = 5.0;
}
