//! Contains global game configuration.

/// The map is a square where a = MAP_SIZE. We use usize instead of f32 because
/// the size of the map is tightly related to pixels, which are discreet.
pub const MAP_SIZE: f32 = 1800.0;

pub mod prey {
    //! Configuration for prey entity.

    use std::time::Duration;

    /// Avoids bumping into another prey by repelling each other if they're too
    /// close.
    pub const AVOID_RADIUS: f32 = 120.0;

    /// This many prey will be spawned when the game starts and as predators
    /// eat prey, we respawn it in such a manner that there's approximately
    /// this much prey throughout the game.
    pub const COUNT: usize = 100;

    /// How many pixels per tick can a prey move. Make sure that this settings
    /// is always larger than the predators max speed.
    pub const MAX_SPEED: f32 = 100.0;

    /// We want the prey to be always on the move.
    pub const MIN_SPEED: f32 = 15.0;

    /// Puts upper bounds on how much can a prey shift its position.
    pub const MAX_STEERING_FORCE: f32 = MAX_SPEED;

    /// How much around itself does a prey see.
    pub const VIEW_RADIUS: f32 = 300.0;

    /// Calculating the flocking behavior is expensive. Let's do it only every
    /// now and then and cache the direction vec.
    pub const RECALCULATE_FLOCKING: Duration = Duration::from_millis(50);

    pub const WALL_REPELLING_FORCE_WEIGHT: f32 = 5.0;
    pub const ALIGNMENT_FORCE_WEIGHT: f32 = 0.8;
    pub const SEPARATION_FORCE_WEIGHT: f32 = 1.5;
    pub const COHESION_FORCE_WEIGHT: f32 = 0.8;
}
