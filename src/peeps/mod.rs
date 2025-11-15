use bevy::prelude::*;
use bevy::app::Plugin;

use crate::peeps::peeps::*;

mod peeps;
mod play;
mod profile;
pub mod bell_hop;

pub struct PeepsPlugin;

impl Plugin for PeepsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PeepSheet>()
            .add_systems(Startup, test)
            .add_systems(Update, test2)
        ;
    }
}

fn test(
    peep_sheet: Res<PeepSheet>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_peep(Vec2::new(0.0, 0.0), &mut commands, &asset_server, &peep_sheet);
}

fn test2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut peep_query: Query<&mut Sprite, With<Peep>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        for mut sprite in peep_query.iter_mut() {
            make_peep_happy(&mut sprite);
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        for mut sprite in peep_query.iter_mut() {
            make_peep_angy(&mut sprite);
        }
    }
}