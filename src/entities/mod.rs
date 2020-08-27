pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use crate::prelude::*;

// Translates prey or prey based on velocity vector, and also rotates it in
// the direction of the vector.
fn nudge_entity(
    time: &Time,
    vel: Vec3,
    pos: &mut Translation,
    rot: &mut Rotation,
) {
    let pos_vec = **pos + vel * time.delta_seconds;
    let pos_vec = pos_vec
        .truncate()
        .min(Vec2::splat(conf::MAP_SIZE as f32))
        .max(Vec2::zero())
        .extend(0.0);
    *pos = pos_vec.into();

    // If the velocity vector is not zero vector, rotate the entity in the
    // direction of its velocity.
    if !vel.cmpeq(Vec3::zero()).all() {
        let vel_norm = vel.normalize();
        // Normalized velocity, find the angle based on the size of the
        // x component, and then shift it if the y component is negative.
        let new_rot = vel_norm.x().acos() * vel_norm.y().signum();
        *rot = Rotation::from_rotation_z(new_rot);
    }
}
