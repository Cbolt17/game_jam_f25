use bevy::prelude::*;
use bevy::app::Plugin;

pub mod grid;
pub mod attraction;

use grid::AttractionGrid;
use crate::grid::attraction::*;
use crate::grid::grid::*;

const START_GRID_SIZE: UVec2 = UVec2::new(6, 6);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AttractionGrid::new(START_GRID_SIZE))
            .insert_resource(AvailableAttractions::new())
            .insert_resource(AttractionBlueprints::new())
            .add_systems(Startup, grid_start)
            .add_systems(Startup, test)
            .add_systems(Update, (
                get_available_attractions
            ))
            .add_observer(on_grid_resize)
        ;
    }
}

fn test(
    mut grid: ResMut<AttractionGrid>,
    blueprints: Res<AttractionBlueprints>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in 0..6 {
        grid.add(
            Vec2::splat(i as f32 * CELL_SIZE), 
            &AttractionType::Roulette, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }/*
    for i in 1..6 {
        grid.add(
            Vec2::new(i as f32 * CELL_SIZE, 0.0), 
            &AttractionType::BlackJack, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }
    for i in 1..6 {
        grid.add(
            Vec2::new(0.0, i as f32 * CELL_SIZE), 
            &AttractionType::Bar, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }*/
}