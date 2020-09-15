//! A predator is an entity which is controlled by an agent. When an agent
//! joins the game over a UDP socket, a new predator is added to the game.
//!
//! A predator is slower than a prey, therefore predators must cooperate with
//! each other in order to score points.
//!
//! A predator can also be controlled by keyboard for debugging purposes.

use crate::{
    components::{camera, KeyboardControlled, Velocity},
    prelude::*,
};

pub struct Predator {
    // Lists positions of prey nearby. With each tick, this value should get
    // reset and repopulated.
    nearby_prey: Vec<Vec3>,
}

/// Predators are actors that join over UDP or keyboard actors. When a predator
/// joins a game, new window with camera focused on them is created.
/// TODO: Allow predators join over UDP and make keyboard predator optional.
pub fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
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
            camera::Focus,
        ));
}

/// Moves those predators which are controlled by keyboard.
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut predator_query: Query<(&mut Velocity, &Predator, &KeyboardControlled)>,
) {
    let max_speed = conf::predator::MAX_SPEED;
    for (mut vel, ..) in &mut predator_query.iter() {
        let pressed_up = keyboard_input.pressed(KeyCode::Up);

        // Left right keys rotate the entity. Holding right or left key indefinitely
        // makes the entity go in circles.
        // If vel was zero, then normalizing would give us gibberish.
        let left_right = if vel.is_zero() {
            Vec3::unit_x()
        } else {
            let vel_norm = vel.normalize();
            let vel_perpendicular = vel_norm.perpendicular();
            if keyboard_input.pressed(KeyCode::Left) {
                -vel_perpendicular
            } else if keyboard_input.pressed(KeyCode::Right) {
                vel_perpendicular
            } else if pressed_up {
                // Travels along the current direction, aka continues forward.
                vel_norm
            } else {
                Vec3::zero()
            }
        };

        // Up and down keys respectively speed up and slow down the acceleration.
        let speed = if pressed_up {
            max_speed * 2.0
        } else if keyboard_input.pressed(KeyCode::Down) {
            max_speed * 0.5
        } else {
            max_speed
        };

        let acc = left_right * time.delta_seconds * speed;
        if !acc.is_zero() {
            // And adds the change in speed to the entity.
            *vel = ((**vel + acc).normalize() * max_speed).into();
        }
    }
}

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
    /// Adds a new prey position into its world view.
    pub fn spot_prey(&mut self, at: Vec3) {
        self.nearby_prey.push(at);
    }

    /// TODO
    pub fn score(&mut self) {
        println!("Prey eaten!");
    }

    fn new() -> Self {
        Self {
            nearby_prey: Vec::new(),
        }
    }
}
