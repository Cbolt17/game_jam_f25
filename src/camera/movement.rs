use bevy::prelude::*;
use crate::grid::grid::AttractionGrid;

const CAMERA_SPEED: f32 = 100.0;

pub fn camera_move(
    mut camera_transform: Single<&mut Transform, With<Camera>>,
    grid: Res<AttractionGrid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut dir = Vec2::splat(0.0);
    if keyboard_input.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    // Return if no input
    if dir.x == 0.0 && dir.y == 0.0 {
        return;
    }
    dir = dir.normalize();

    let max = grid.get_bounds();
    let mut translation = camera_transform.translation + (dir * CAMERA_SPEED * time.delta_secs()).extend(0.0);
    translation = translation.clamp(Vec3::splat(0.0), max.extend(0.0));
    camera_transform.translation = translation;
}