//! A predator is an entity which is controlled by an agent. When an agent
//! joins the game over a UDP socket, a new predator is added to the game.
//!
//! A predator is slower than a prey, therefore predators must cooperate with
//! each other in order to score points.
//!
//! A predator can also be controlled by keyboard for debugging purposes.

use crate::{
    components::{KeyboardControlled, Velocity},
    prelude::*,
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
/// TODO: It'd be nice to have this method deleted and use only the parent once.
/// However I don't know how to check whether an entity is prey or predator.
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut predator_query: Query<(&mut Velocity, &Predator, &KeyboardControlled)>,
) {
    for (mut vel, ..) in &mut predator_query.iter() {
        super::keyboard_movement(
            &time,
            &keyboard_input,
            &mut vel,
            conf::predator::MAX_SPEED,
        )
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
