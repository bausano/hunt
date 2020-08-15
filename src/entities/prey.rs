//! Prey entities are simple creatures. They flock around a few individuals,
//! flock leaders, and when threatened, they run away from the closer predator
//! along the vector formed by predators position and its own position.
//!
//!
//!                         🐺
//!                        /
//!                      |/_
//!                      🐑<--------🐺---->🐑 . . . >
//!                     .
//!                    .
//!                  \._
//!
//! The catch is that the prey is faster than then predator. If the predators
//! are not organized, they won't get fed.

use {bevy::prelude::*, cgmath::MetricSpace};

use crate::prelude::*;

/// Creates initial batch of prey.
pub fn init(mut commands: Commands) {
    for _ in 0..conf::prey::INITIAL_COUNT {
        commands.spawn((Prey::new(),));
    }
}

/// Prey is represented with a position vector [TODO: and a velocity vector].
#[derive(Debug, Shrinkwrap)]
pub struct Prey {
    #[shrinkwrap(main_field)]
    pub pos: Vector2,
}

impl Prey {
    // Randomly positions the prey in the map.
    fn new() -> Self {
        let rand_coord = || (rand::random::<usize>() % conf::MAP_SIZE) as f32;

        Self {
            pos: Vector2::new(rand_coord(), rand_coord()),
        }
    }

    /// Moves the prey towards another prey, a flock leader, in a straight line.
    pub fn move_towards(&mut self, other: &Self) {
        let distance = self.distance2(other.pos);

        // Based on the position of the leader and the self, decide how close
        // should we keep to the leader.
        let pseudorandom_offset = (self.x + other.y) % 5.0;
        // We don't want to move too close to the flock leader.
        if distance < conf::prey::RADIUS * (2.0 + pseudorandom_offset) {
            return;
        }

        // Based on current value of x/y coord, and given another value, move
        // that value closer to the latter.
        let move_in_direction = |current, towards| {
            if towards > current {
                current + conf::prey::MAX_SPEED
            } else if current > conf::prey::MAX_SPEED {
                current - conf::prey::MAX_SPEED
            } else {
                0.0
            }
        };

        self.pos.x = move_in_direction(self.x, other.x);
        self.pos.y = move_in_direction(self.y, other.y);
    }
}
