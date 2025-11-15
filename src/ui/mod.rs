use bevy::prelude::*;
use bevy::app::Plugin;

use crate::ui::header::{MoneyDisplay, create_header, update_capacity_text, update_money_text, update_sus_gauge};

pub mod header;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_header)
            .add_systems(Update, (update_money_text, update_capacity_text, update_sus_gauge))
            .insert_resource(MoneyDisplay{current: 0, change: 97})
        ;
    }
}
