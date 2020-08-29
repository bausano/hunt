use crate::prelude::*;

/// Spawns a new zoomed-out camera component. Additionally, each predator has
/// their own window with their own camera.
pub fn new(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
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
