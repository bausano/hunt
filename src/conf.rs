//! Contains global game configuration.

/// The map is a square where a = MAP_SIZE. We use usize instead of f32 because
/// the size of the map is tightly related to pixels, which are discreet.
pub const MAP_SIZE: usize = 2000;

pub mod prey {
    //! Configuration for prey entity.

    /// How large prey is.
    pub const RADIUS: f32 = 3.0;

    /// This many prey will be spawned when the game starts and as predators
    /// eat prey, we respawn it in such a manner that there's approximately
    /// this much prey throughout the game.
    pub const COUNT: usize = 100;

    /// How many pixels per tick can a prey move. Make sure that this settings
    /// is always larger than the predators max speed.
    pub const MAX_SPEED: f32 = 3.0;
}
