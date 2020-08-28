//! A predator is an entity which is controlled by an agent. When an agent
//! joins the game over a UDP socket, a new predator is added to the game.
//!
//! A predator is slower than a prey, therefore predators must cooperate with
//! each other in order to score points.

use crate::{
    prelude::*,
    properties::{KeyboardControlled, Velocity},
};

pub struct Predator {
    // Lists positions of prey nearby. With each tick, this value should get
    // reset and repopulated.
    nearby_prey: Vec<Vec3>,
}

/// Creates initial batch of prey.
pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server
        .load(conf::predator::ICON)
        .expect("Cannot load predator sprite");

    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with_bundle((
            Predator::new(),
            Velocity::default(),
            Translation::random(),
            Rotation::default(),
            KeyboardControlled,
        ));
}

/// Moves those predators which are controlled by keyboard.
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut predator_query: Query<(&Predator, &KeyboardControlled, &mut Velocity)>,
) {
    for (_, _, mut vel) in &mut predator_query.iter() {
        // TODO: This should probably be rotated with respect to the current
        // velocity direction. We use normalized velocity vec as the base, add
        // unit vec in appropriate direction, and change base to standard.
        let x_vel = if keyboard_input.pressed(KeyCode::Left) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        let y_vel = if keyboard_input.pressed(KeyCode::Down) {
            -1.0
        } else if keyboard_input.pressed(KeyCode::Up) {
            1.0
        } else {
            0.0
        };

        let vel_change = Vec3::new(x_vel, y_vel, 0.0)
            * time.delta_seconds
            * conf::predator::MAX_SPEED;

        if vel_change != Vec3::zero() {
            // And adds the change in speed to the entity.
            *vel = ((**vel + vel_change).normalize()
                * conf::predator::MAX_SPEED)
                .into();
        }
    }
}

/// Moves each predator based on its velocity vector.
/// TODO: Make a for-each when bevy is fixed.
// pub fn nudge(
//     time: Res<Time>,
//     mut predator_query: Query<(&Predator, &mut Translation, &mut Rotation)>,
// ) {
//     for (predator, mut pos, mut rot) in &mut predator_query.iter() {
//         let vel = *predator.vel.lock();
//         super::nudge_entity(&time, vel, &mut pos, &mut rot);
//     }
// }

/// Resets the state which is at the end of each tick sent to the actor which
/// controls the predator. This method MUST be called in the beginning of each
/// tick before any world update happens.
/// TODO: It'd be nice to have this as foreach, but bevy types are broken for now.
pub fn reset_world_view(mut predator_query: Query<&mut Predator>) {
    for mut predator in &mut predator_query.iter() {
        predator.nearby_prey.clear();
    }
}

impl Predator {
    fn new() -> Self {
        Self {
            nearby_prey: Vec::new(),
        }
    }

    /// Adds a new prey position into its world view.
    pub fn spot_prey(&mut self, at: Vec3) {
        self.nearby_prey.push(at);
    }

    /// TODO
    pub fn score(&mut self) {
        println!("Prey eaten!");
    }
}
