use std::time::Duration;

use crate::prelude::*;

/// Marks some entity as being controlled by the keyboard.
#[derive(Clone, Copy)]
pub struct KeyboardControlled;

/// Velocity represents into which direction and with how much magnitude an
/// entity wants to move.
#[derive(Shrinkwrap, Default, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct Velocity(pub Vec3);

impl From<Vec3> for Velocity {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl Velocity {
    /// Applies acceleration to self given how long should the acceleration
    /// last and max speed.
    pub fn apply_acceleration(
        &mut self,
        acc: Vec3,
        time: Duration,
        speed_clamp: impl FnOnce(f32) -> f32,
    ) {
        // Updates the velocity vector of the prey.
        let mut vel = **self;
        let dv = acc * time.as_millis() as f32 / 1000.0;
        vel += dv;
        let speed = vel.length();
        let direction = vel / speed;
        self.0 = direction * speed_clamp(speed);
    }
}
