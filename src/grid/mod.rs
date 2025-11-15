use bevy::prelude::*;
use bevy::app::Plugin;

pub mod grid;
pub mod attraction;

use grid::AttractionGrid;
use crate::grid::attraction::*;
use crate::grid::grid::CELL_SIZE;

const START_GRID_SIZE: UVec2 = UVec2::new(5, 5);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AttractionGrid::new(START_GRID_SIZE))
            .insert_resource(AttractionBlueprints::new())
            .add_systems(Startup, test)
        ;
    }
}

fn test(
    mut grid: ResMut<AttractionGrid>,
    blueprints: Res<AttractionBlueprints>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in 0..5 {
        grid.add(
            Vec2::splat(i as f32 * CELL_SIZE), 
            &AttractionType::Roulette, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }
    for i in 1..5 {
        grid.add(
            Vec2::new(i as f32 * CELL_SIZE, 0.0), 
            &AttractionType::BlackJack, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }
    for i in 1..5 {
        grid.add(
            Vec2::new(0.0, i as f32 * CELL_SIZE), 
            &AttractionType::Bar, 
            &blueprints, 
            &mut commands, 
            &asset_server
        );
    }
}