use bevy::prelude::*;

use crate::grid::attraction::{AttractionBlueprints, AttractionType, spawn_attraction};

pub const CELL_SIZE: f32 = 32.0;

pub fn to_cell(pos: Vec2) -> UVec2 {
    UVec2::new((pos.x / CELL_SIZE) as u32, (pos.y / CELL_SIZE) as u32)
}

#[derive(Resource)]
pub struct AttractionGrid {
    size: UVec2,
    cells: Vec<Vec<Option<Entity>>>,
}

impl AttractionGrid {
    pub fn new(size: UVec2) -> Self {
        AttractionGrid{size, cells: vec![vec![None; size.x as usize]; size.y as usize]}
    }
    pub fn at(&self, pos: Vec2) -> Option<Entity> {
        let pos = to_cell(pos);
        if pos.x >= self.size.x || pos.y >= self.size.y {
            None
        }
        else {
            self.cells[pos.x as usize][pos.y as usize]
        }
    }
    pub fn get_coords(pos: Vec2) -> Vec2 {
        Vec2::new(
            (pos.x / CELL_SIZE) as u32 as f32 * CELL_SIZE, 
            (pos.x / CELL_SIZE) as u32 as f32 * CELL_SIZE
        )
    }
    pub fn add(
        &mut self, 
        position: Vec2,
        attraction: &AttractionType,
        blueprints: &Res<AttractionBlueprints>,
        commands: &mut Commands, 
        asset_server: &Res<AssetServer>,
    ) -> bool {
        let pos = to_cell(position);
        if pos.x < self.size.x && pos.y < self.size.y && self.cells[pos.x as usize][pos.y as usize] == None {
            let entity = spawn_attraction(
                Vec2::new(pos.x as f32 * CELL_SIZE, pos.y as f32 * CELL_SIZE), 
                attraction, 
                blueprints, 
                commands, 
                asset_server
            );
            self.cells[pos.x as usize][pos.y as usize] = Some(entity);
            return true;
        }
        false
    }
    pub fn get_bounds(&self) -> Vec2 {
        Vec2::new(self.size.x as f32 * CELL_SIZE, self.size.y as f32 * CELL_SIZE)
    }
}