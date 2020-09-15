use crate::prelude::*;

/// Accesses camera.
#[derive(Clone, Copy)]
pub struct Main;

/// Marks some entity as being focused by the camera.
#[derive(Clone, Copy)]
pub struct Focus;

/// Spawns a new zoomed-out camera component. Additionally, each predator has
/// their own window with their own camera.
pub fn new(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default()).with_bundle((
        Main,
        Scale::identity(),
        Translation::new(
            conf::MAP_SIZE as f32 / 2.0,
            conf::MAP_SIZE as f32 / 2.0,
            0.0,
        ),
    ));
}

/// Zooms with "+" and "-" keys.
/// TODO: If there was a way to make the camera a resource then we could avoid
/// the query.
pub fn zoom(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(&mut Scale, &Main)>,
) {
    for (mut scale, ..) in &mut camera_query.iter() {
        if keyboard_input.pressed(KeyCode::Add) {
            // Zoom in on "+".
            *scale = (**scale - 0.05).max(0.3).into();
        } else if keyboard_input.pressed(KeyCode::Subtract) {
            // Zoom out on "-".
            *scale = (**scale + 0.05).min(3.0).into();
        }
    }
}

/// Camera follows around entity which is marked as focused.
pub fn follow(
    mut camera_query: Query<(&mut Translation, &Main)>,
    mut entity_query: Query<(&Translation, &Focus)>,
) {
    for (mut camera_pos, ..) in &mut camera_query.iter() {
        for (entity_pos, ..) in &mut entity_query.iter() {
            *camera_pos = *entity_pos;
            // There can be at most one focused entity.
            break;
        }
        break;
    }
}
