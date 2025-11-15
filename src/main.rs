use bevy::prelude::*;

use crate::{grid::GridPlugin, ui::UiPlugin, casino::CasinoPlugin};

pub mod ui;
pub mod grid;
pub mod casino;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, GridPlugin, CasinoPlugin))
    .run();
}