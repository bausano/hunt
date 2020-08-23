use bevy::prelude::*;

pub mod conf;
mod entities;
mod prelude;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_startup_system(entities::prey::init.system())
        // .add_system(entities::move_prey_and_mark_in_danger.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        // TODO: Let the viewer choose which predator to focus on or every
        // 10s change predator focus.
        translation: Translation::new(1000.0, 1000.0, 0.0),
        // Let the viewer zoom in and out.
        scale: 2f32.into(),
        ..Default::default()
    });
}
