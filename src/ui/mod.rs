use bevy::prelude::*;
use bevy::app::Plugin;

use crate::{game::GameState, ui::{buybar::{create_buybar, update_buttons, update_server_button}, header::{MoneyDisplay, create_header, update_capacity_text, update_money_text, update_sus_gauge, update_time_text}, placing::{SelectedAttraction, place_attraction}, title::create_title}};

pub mod utils;
pub mod header;
pub mod buybar;
pub mod placing;
pub mod title;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (create_header, create_buybar, create_title))
            .add_systems(Update, (
                update_money_text,
                update_capacity_text,
                update_sus_gauge,
                update_buttons,
                update_server_button,
                place_attraction,
                update_time_text,
            ))
            .insert_resource(MoneyDisplay{current: 0, change: 0, was_changed: false})
            .insert_resource(SelectedAttraction::None)
            .add_systems(OnExit(GameState::Started), reset)
        ;
    }
}

fn reset(
    mut select: ResMut<SelectedAttraction>,
) {
    *select = SelectedAttraction::None;
}