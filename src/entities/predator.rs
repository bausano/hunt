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
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        // Acquires the lock to update velocity.
        let mut vel = predator.vel.lock();

        // And adds a bit of speed to the entity.
        *vel.x_mut() +=
            time.delta_seconds * direction * conf::predator::MAX_SPEED;

        // bound the paddle within the walls
        // *translation.0.x_mut() = f32::max(-380.0, f32::min(380.0, translation.0.x()));
    }
}

/// Moves each predator based on its velocity vector.
/// TODO: Deduplicate this code with prey translation possibly.
pub fn translate(
    time: Res<Time>,
    mut predator_query: Query<(&Predator, &mut Translation, &mut Rotation)>,
) {
    for (predator, mut pos, mut rot) in &mut predator_query.iter() {
        let vel = *predator.vel.lock();
        let pos_vec = **pos + vel * time.delta_seconds;
        let pos_vec = pos_vec
            .truncate()
            .min(Vec2::splat(conf::MAP_SIZE as f32))
            .max(Vec2::zero())
            .extend(0.0);
        *pos = pos_vec.into();

        // If the velocity vector is not zero vector, rotate the entity in the
        // direction of its velocity.
        if vel.cmpne(Vec3::zero()).all() {
            let vel_norm = vel.normalize();
            let curr_rot = rot.w();
            // Normalized velocity, find the angle based on the size of the
            // x component, and then shift it if the y component is negative.
            let new_rot = vel_norm.x().acos() * vel_norm.y().signum();
            *rot = Rotation::from_rotation_z(curr_rot - (curr_rot - new_rot));
        }
    }
}

impl Predator {
    fn new() -> Self {
        Self {
            vel: Arc::new(Mutex::new(Vec3::zero())),
        }
    }
}
