use bevy::prelude::*;

use crate::{camera::CameraPlugin, casino::CasinoPlugin, grid::GridPlugin, peeps::PeepsPlugin, ui::UiPlugin, audio::AudioPlugin};

pub mod ui;
pub mod grid;
pub mod peeps;
pub mod casino;
pub mod audio;
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
            AudioPlugin
        ))
    .run();
}