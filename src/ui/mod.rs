use bevy::prelude::*;
use bevy::app::Plugin;

use crate::ui::header::{MoneyDisplay, create_header, update_capacity_text, update_money_text};

pub mod header;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (update_money_text, update_capacity_text))
            .insert_resource(MoneyDisplay{current: 0, change: 97})
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    create_header(commands);
}
