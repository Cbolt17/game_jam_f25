use bevy::prelude::*;

use crate::{grid::GridPlugin, ui::UiPlugin};

pub mod ui;
pub mod grid;

fn main() {
    App::new()
        .add_plugins((UiPlugin, GridPlugin))
    .run();
}