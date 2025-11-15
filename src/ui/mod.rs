use bevy::prelude::*;
use bevy::app::Plugin;

//pub mod some_file_name;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Update, (add_health_bars, update_health_bars))
        ;
    }
}