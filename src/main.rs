//! ECS system splits our logic into three main modules:
//! * entities: predators and prey
//! * components
//! * resources
//!
//! TODO: There's also net modules for UDP communication with actors that
//! control predator entities.

#[macro_use]
extern crate shrinkwraprs;

mod components;
pub mod conf;
mod entities;
mod prelude;
mod resources;

use crate::prelude::*;

fn main() {
    App::build()
        .add_resource(bevy::render::pass::ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        // We only do update to the prey velocity every N ms to avoid needless
        // expensive computation.
        .add_resource(resources::FlockUpdateTimer::default())
        .add_default_plugins()
        .add_startup_system(components::camera::new.system())
        .add_startup_system(components::walls::new.system())
        .add_startup_system(entities::predator::init.system())
        .add_startup_system(entities::prey::init.system())
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
        // Allows for zooming of camera and following focused predator.
        .add_system(components::camera::zoom.system())
        .add_system(components::camera::follow.system())
        .run();
}
