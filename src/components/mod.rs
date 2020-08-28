use crate::prelude::*;

/// Marks some entity as being controlled by the keyboard.
#[derive(Clone, Copy)]
pub struct KeyboardControlled;

/// Velocity represents into which direction and with how much magnitude an
/// entity wants to move.
#[derive(Shrinkwrap, Default, Clone, Copy)]
pub struct Velocity(pub Vec3);

impl From<Vec3> for Velocity {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}
