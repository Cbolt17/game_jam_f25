use bevy::prelude::*;

use crate::{camera::CameraPlugin, casino::CasinoPlugin, game::GamePlugin, grid::GridPlugin, peeps::PeepsPlugin, ui::UiPlugin, audio::AudioPlugin};

pub mod ui;
pub mod grid;
pub mod peeps;
pub mod casino;
pub mod audio;
mod camera;
mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            GamePlugin, 
            UiPlugin, 
            GridPlugin, 
            PeepsPlugin,
            CasinoPlugin,
            CameraPlugin,
            AudioPlugin
        ))
    .run();
}