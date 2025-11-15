use bevy::prelude::*;
use bevy::app::Plugin;

use crate::camera::movement::*;

pub mod movement;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, camera_move)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // Zooms Camera In (Does not affect UI)
        Transform::from_scale(Vec3::new(0.4, 0.4, 1.0))
    ));
}