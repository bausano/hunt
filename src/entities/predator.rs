//! A predator is an entity which is controlled by an agent. When an agent
//! joins the game over a UDP socket, a new predator is added to the game.
//!
//! A predator is slower than a prey, therefore predators must cooperate with
//! each other in order to score points.

use {parking_lot::Mutex, std::sync::Arc};

use crate::prelude::*;

pub struct Predator {
    vel: Arc<Mutex<Vec3>>,
}

pub struct KeyboardControlled;

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
            Translation::random(),
            Rotation::default(),
            KeyboardControlled,
        ));
}

/// Moves those predators which are controlled by keyboard.
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut predator_query: Query<(&Predator, &KeyboardControlled)>,
) {
    for (predator, _) in &mut predator_query.iter() {
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
            // Acquires the lock to update velocity.
            let mut vel = predator.vel.lock();

            // And adds the change in speed to the entity.
            *vel = (*vel + vel_change).normalize() * conf::predator::MAX_SPEED;
        }
    }
}

/// Moves each predator based on its velocity vector.
/// TODO: Make a for-each when bevy is fixed.
pub fn nudge(
    time: Res<Time>,
    mut predator_query: Query<(&Predator, &mut Translation, &mut Rotation)>,
) {
    for (predator, mut pos, mut rot) in &mut predator_query.iter() {
        let vel = *predator.vel.lock();
        super::nudge_entity(&time, vel, &mut pos, &mut rot);
    }
}

impl Predator {
    fn new() -> Self {
        Self {
            vel: Arc::new(Mutex::new(Vec3::zero())),
        }
    }
}
