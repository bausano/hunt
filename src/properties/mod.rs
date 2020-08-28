use crate::prelude::*;

/// Marks some entity as being controlled by the keyboard.
pub struct KeyboardControlled;

/// Velocity represents into which direction and with how much magnitude an
/// entity wants to move.
#[derive(Shrinkwrap)]
pub struct Velocity(pub Vec3);
