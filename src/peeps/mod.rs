use bevy::prelude::*;
use bevy::app::Plugin;

use crate::peeps::bet_effects::*;
use crate::peeps::peep_spawner::PeepSpawner;
use crate::peeps::peeps::*;
use crate::peeps::peep_spawner::peep_spawner;
use crate::peeps::drunk::*;
use crate::peeps::server::{SpawnServerEvent, server_do_stuff, server_spawner};

pub mod peeps;
pub mod play;
pub mod profile;
pub mod server;
pub mod bet_effects;
pub mod peep_spawner;
pub mod drunk;

pub struct PeepsPlugin;

impl Plugin for PeepsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PeepSheet>()
            .insert_resource(PeepSpawner::new(5.0, 5.0))
            .add_systems(Update, (
                test2,
                peep_target,
                peep_goto,
                move_bet_text,
                despawn_bet_text,
                peep_spawner,
                passout_chance,
                server_do_stuff,
            ))
            .add_observer(bet_result)
            .add_observer(bet_effect)
            .add_observer(peep_reach_attraction)
            .add_observer(peep_reach_door)
            .add_observer(server_reach_door)
            .add_observer(server_reach_peep)
            .add_observer(add_drunk_timer)
            .add_observer(peep_passout)
            .add_observer(server_spawner)
        ;
    }
}

fn test2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut peep_query: Query<&mut Sprite, With<Peep>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        commands.trigger(SpawnServerEvent);
    }
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