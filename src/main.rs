#[macro_use]
extern crate shrinkwraprs;

mod components;
pub mod conf;
mod entities;
mod prelude;
mod resources;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_startup_system(entities::predator::init.system())
        .add_startup_system(entities::prey::init.system())
        // We only do update to the prey velocity every N ms to avoid needless
        // expensive computation.
        .add_resource(resources::FlockUpdateTimer::default())
        // Must be called before any state updates.
        .add_system(entities::predator::reset_world_view.system())
        // Simulates interactions between prey and predators.
        .add_system(entities::interact.system())
        // Simulates flocking behavior for prey which isn't in danger. We should
        // run the logic which lets prey spot a predator before this system to
        // avoid needless computation.
        .add_system(entities::prey::flocking_behavior.system())
        // Moves the predators which are controlled by keyboard.
        .add_system(entities::predator::keyboard_movement.system())
        // Moves all entities along their velocity vectors.
        .add_system(entities::nudge.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        // TODO: Let the viewer choose which predator to focus on or every
        // 10s change predator focus. Alternatively create a window for each.
        translation: Translation::new(
            conf::MAP_SIZE as f32 / 2.0,
            conf::MAP_SIZE as f32 / 2.0,
            0.0,
        ),
        // Let the viewer zoom in and out.
        scale: 1f32.into(),
        ..Default::default()
    });
}
