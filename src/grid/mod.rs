use bevy::prelude::*;
use bevy::app::Plugin;

pub mod grid;
pub mod attraction;
pub mod play_attraction;
pub mod door;

use grid::AttractionGrid;
use crate::game::GameState;
use crate::grid::attraction::*;
use crate::grid::door::set_door;
use crate::grid::grid::*;
use crate::grid::play_attraction::play_game;

const START_GRID_SIZE: UVec2 = UVec2::new(16, 10);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AttractionGrid::new(START_GRID_SIZE))
            .insert_resource(AvailableAttractions::new())
            .insert_resource(AttractionBlueprints::new())
            .add_systems(OnEnter(GameState::Started), (
                grid_start,
                set_door,
            ))
            .add_systems(Update, (
                get_available_attractions,
                play_game
            ))
            .add_observer(on_grid_resize)
            .add_systems(OnExit(GameState::Started), reset)
        ;
    }
}

fn reset(
    mut grid: ResMut<AttractionGrid>,
    mut avail: ResMut<AvailableAttractions>
) {
    grid.reset();
    avail.0.clear();
}