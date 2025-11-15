use bevy::prelude::*;
use bevy::app::Plugin;

pub mod grid;
pub mod attraction;

use grid::AttractionGrid;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AttractionGrid::new(UVec2::new(25,25)))
            //.add_systems(Update, (add_health_bars, update_health_bars))
        ;
    }
}