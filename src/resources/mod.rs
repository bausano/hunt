use std::time::Duration;

use crate::prelude::*;

/// Calculation of flocking behavior is expensive. We undergo this calculation
/// only few times a second.
pub struct FlockUpdateTimer(Timer);

/// Allows key to be pressed only once in a while. Prevents unwanted bursts.
pub struct KeyPressDelay(Timer);

impl FlockUpdateTimer {
    pub fn tick(&mut self, seconds: f32) {
        self.0.tick(seconds)
    }

    pub fn is_finished(&self) -> bool {
        self.0.finished
    }
}

impl Default for FlockUpdateTimer {
    fn default() -> Self {
        Self(Timer::new(conf::prey::RECALCULATE_FLOCKING, true))
    }
}

impl KeyPressDelay {
    pub fn tick(&mut self, seconds: f32) {
        self.0.tick(seconds)
    }

    pub fn is_finished(&self) -> bool {
        self.0.finished
    }
}

impl Default for KeyPressDelay {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(250), true))
    }
}
