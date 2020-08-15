#[macro_use]
extern crate shrinkwraprs;

use bevy::prelude::*;

pub mod conf;
mod entities;
mod prelude;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(entities::prey::init.system())
        .add_system(entities::move_prey_and_mark_in_danger.system())
        .run();
}
