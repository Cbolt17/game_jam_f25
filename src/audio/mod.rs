use bevy::prelude::*;
use bevy::app::Plugin;

use crate::audio::musicplayer::{play_music};

pub mod musicplayer;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, play_music)
            //.add_systems(Update, update_music)
        ;
    }
}