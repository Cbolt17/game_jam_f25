use bevy::prelude::*;
use bevy::app::Plugin;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Update, (add_health_bars, update_health_bars))
        ;
    }
}