use bevy::prelude::*;

pub mod conf;
mod entities;
mod prelude;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_startup_system(entities::prey::init.system())
        .add_resource(entities::prey::FlockUpdateTimer::default())
        .add_system(entities::prey::flocking_behavior.system())
        .add_system(entities::prey::translate.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        // TODO: Let the viewer choose which predator to focus on or every
        // 10s change predator focus.
        translation: Translation::new(
            conf::MAP_SIZE as f32 / 2.0,
            conf::MAP_SIZE as f32 / 2.0,
            0.0,
        ),
        // Let the viewer zoom in and out.
        scale: 2f32.into(),
        ..Default::default()
    });
}
