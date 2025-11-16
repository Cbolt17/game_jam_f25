use bevy::prelude::*;
use bevy::app::Plugin;

use crate::camera::movement::*;
use crate::game::GameState;
use crate::grid::grid::{AttractionGrid, CELL_SIZE};

pub mod movement;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, camera_move)
            .add_systems(OnEnter(GameState::Started), center)
        ;
    }
}

fn center(
    grid: Res<AttractionGrid>,
    mut cam: Single<&mut Transform, With<Camera>>,
) {
    let size = grid.get_size();
    let x = (size.x / 2) as f32 * CELL_SIZE;
    let y =  3.0 * CELL_SIZE - 16.0;
    cam.translation = Vec3::new(x, y, 0.0);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // Zooms Camera In (Does not affect UI)
        Transform::from_scale(Vec3::new(0.4, 0.4, 1.0))
    ));
}