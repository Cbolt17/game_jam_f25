use bevy::prelude::*;

use crate::grid::grid::AttractionGrid;

#[derive(Component)]
pub struct Door;

pub fn set_door(
    grid: Res<AttractionGrid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let translation = grid.get_door();
    commands.spawn((
        Door,
        Transform::from_translation(translation.extend(8.0)),
        Sprite {
            image: asset_server.load("Door.png"),
            ..default()
        }
    ));
}