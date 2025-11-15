use bevy::prelude::*;

use crate::{camera::CameraPlugin, casino::CasinoPlugin, grid::GridPlugin, peeps::PeepsPlugin, ui::UiPlugin};

pub mod ui;
pub mod grid;
pub mod peeps;
pub mod casino;
mod camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()), 
            UiPlugin, 
            GridPlugin, 
            PeepsPlugin,
            CasinoPlugin,
            CameraPlugin,
        ))
    .run();
}