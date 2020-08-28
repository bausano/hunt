
use crate::prelude::*;

/// Calculation of flocking behavior is expensive. We undergo this calculation
/// only few times a second.
pub struct FlockUpdateTimer(Timer);

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
