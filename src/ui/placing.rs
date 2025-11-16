use bevy::{prelude::*, window::PrimaryWindow};

use crate::{casino::CasinoMoney, grid::{attraction::{AttractionBlueprints, AttractionType}, grid::AttractionGrid}};

#[derive(Resource)]
pub enum SelectedAttraction {
    None,
    Some(AttractionType)
}

pub fn place_attraction(
    mut selected_attraction: ResMut<SelectedAttraction>,
    mut grid: ResMut<AttractionGrid>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_single: Single<(&Camera, &GlobalTransform)>,
    blueprints: Res<AttractionBlueprints>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_ui: Query<&Interaction>,
    mut money: ResMut<CasinoMoney>
) {
    let (camera, transform) = camera_single.into_inner();
    if let SelectedAttraction::Some(attraction_type) = *selected_attraction {
        if buttons.just_pressed(MouseButton::Left) &&
        let Some(pos) = window.cursor_position() {
            if q_ui.iter().any(|i| *i != Interaction::None) {
                return;
            }
            let world_pos =  camera.viewport_to_world(transform, pos).map(|ray| ray.origin.truncate()).unwrap(); 
            if grid.at(world_pos) == None {
                grid.add(world_pos, &attraction_type, &blueprints, &mut commands, &asset_server);
                money.0 -= blueprints.get(attraction_type).cost;
            }
            *selected_attraction = SelectedAttraction::None;
        }
    }
}

