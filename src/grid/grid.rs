use bevy::prelude::*;

use crate::grid::attraction::{AttractionBlueprints, AttractionType, spawn_attraction};

pub const CELL_SIZE: f32 = 48.0;
pub const ATTRACTION_SIZE: f32 = 32.0;
pub const ATTRACTION_OFFSET: f32 = (CELL_SIZE-ATTRACTION_SIZE)/2.0;

pub fn to_cell(pos: Vec2) -> UVec2 {
    UVec2::new((pos.x / CELL_SIZE) as u32, (pos.y / CELL_SIZE) as u32)
}

#[derive(Resource)]
pub struct AttractionGrid {
    size: UVec2,
    cells: Vec<Vec<Option<Entity>>>,
}

#[derive(Event)]
pub struct GridResize {
    old: UVec2,
    new: UVec2,
}

impl GridResize {
    pub fn new(size: UVec2) -> Self {
        GridResize{old: UVec2::splat(0), new: size}
    }
    pub fn resize(old: UVec2, new: UVec2) -> Self {
        GridResize{old, new}
    }
}

pub fn grid_start(
    grid: Res<AttractionGrid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let x_max = grid.get_size().x / 2;
    let y_max = grid.get_size().y / 2;
    for x in 0..x_max {
        for y in 0..y_max {
            commands.spawn((
                Sprite {
                    image: asset_server.load("Tile2x2.png"),
                    ..default()
                },
                Transform::from_xyz(
                    (x as f32 + 0.5) * 2.0*CELL_SIZE, 
                    (y as f32 + 0.5) * 2.0*CELL_SIZE, 
                    -(y as f32+ 2.0) * 2.0*CELL_SIZE
                ),
            ));
        }
    }
}

pub fn on_grid_resize (
    resize: On<GridResize>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for y in 0..(resize.old.y / 2) {
        for x in (resize.old.x / 2)..(resize.new.x / 2) {
            commands.spawn((
                Sprite {
                    image: asset_server.load("Tile2x2.png"),
                    ..default()
                },
                Transform::from_xyz(
                    (x as f32 + 0.5) * 2.0*CELL_SIZE, 
                    (y as f32 + 0.5) * 2.0*CELL_SIZE, 
                    -(y as f32+ 100.0) * 100.0*CELL_SIZE
                ),
            ));
        }
    }
    for y in (resize.old.y / 2)..(resize.new.y / 2) {
        for x in 0..(resize.new.x / 2) {
            commands.spawn((
                Sprite {
                    image: asset_server.load("Tile2x2.png"),
                    ..default()
                },
                Transform::from_xyz(
                    (x as f32 + 0.5) * 2.0*CELL_SIZE, 
                    (y as f32 + 0.5) * 2.0*CELL_SIZE, 
                    -(y as f32+ 100.0) * 100.0*CELL_SIZE
                ),
            ));
        }
    }
}

impl AttractionGrid {
    pub fn new(size: UVec2) -> Self {
        AttractionGrid{size, cells: vec![vec![None; size.y as usize]; size.x as usize]}
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
    pub fn get_cell(pos: Vec2) -> IVec2 {
        IVec2::new(
            (pos.x / CELL_SIZE) as i32, 
            (pos.y / CELL_SIZE) as i32
        )
    }
    pub fn get_coords(pos: IVec2) -> Vec2 {
        Vec2::new(
            pos.x as f32 * CELL_SIZE, 
            pos.y as f32 * CELL_SIZE
        )
    }
    pub fn resize(&mut self, size: UVec2, commands: &mut Commands) {
        commands.trigger(GridResize::resize(self.size, size));
        for x in 0..self.size.x {
            self.cells[x as usize].resize(size.y as usize, None);
        }
        self.cells.resize(size.x as usize, vec![None; size.y as usize]);
        self.size = size;
    }
    pub fn get_size(&self) -> UVec2 {
        self.size
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