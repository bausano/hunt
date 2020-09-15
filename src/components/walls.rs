use crate::prelude::*;

/// Displays walls around the play field.
pub fn new(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Map bounds. Walls are created all around it.
    let bounds = Vec2::splat(conf::MAP_SIZE);
    let material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let thickness = 10.0;

    let horizontal = || Vec2::new(bounds.y() + thickness, thickness);
    let vertical = || Vec2::new(thickness, bounds.y() + thickness);
    let create_wall = |center_pos, slope| {
        let (x, y) = center_pos;
        SpriteComponents {
            material,
            translation: Translation::new(x, y, 0.0),
            sprite: Sprite { size: slope },
            ..Default::default()
        }
    };

    commands
        // Left wall.
        .spawn(create_wall((0.0, bounds.y() / 2.0), vertical()))
        // Right wall.
        .spawn(create_wall((bounds.x(), bounds.y() / 2.0), vertical()))
        // Top wall.
        .spawn(create_wall((bounds.x() / 2.0, 0.0), horizontal()))
        // Bottom wall.
        .spawn(create_wall((bounds.x() / 2.0, bounds.x()), horizontal()));
}
