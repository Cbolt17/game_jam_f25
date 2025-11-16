use bevy::prelude::*;
use bevy::app::Plugin;

use crate::game::GameState;
use crate::peeps::effects::*;
use crate::peeps::inspector::{InspectorSpawner, SpawnInspectorEvent, inspector_spawner, inspector_spawner_timer, inspector_taken, inspector_target, inspector_target_die};
use crate::peeps::peep_spawner::{PeepSpawner, SpawnPeepEvent, peep_spawner_timer};
use crate::peeps::peeps::*;
use crate::peeps::peep_spawner::peep_spawner;
use crate::peeps::drunk::*;
use crate::peeps::server::{SpawnServerEvent, server_target, server_spawner};

pub mod peeps;
pub mod play;
pub mod profile;
pub mod server;
pub mod effects;
pub mod peep_spawner;
pub mod drunk;
mod inspector;

pub struct PeepsPlugin;

impl Plugin for PeepsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PeepSheet>()
            .insert_resource(PeepSpawner::new(5.0, 5.0))
            .insert_resource(InspectorSpawner::new(150.0, 150.0))
            .add_systems(Update, (
                cheats,
                peep_goto,
                move_bet_text,
                despawn_bet_text,
                passout_chance,
                peep_target,
                server_target,
                inspector_target,
                peep_spawner_timer,
                inspector_spawner_timer,
                pass_out_die,

            ))
            .add_observer(bet_result)
            .add_observer(bet_effect)
            .add_observer(peep_reach_attraction)
            .add_observer(peep_reach_door)
            .add_observer(server_reach_door)
            .add_observer(server_reach_peep)
            .add_observer(add_drunk_timer)
            .add_observer(peep_passout)
            .add_observer(inspector_spawner)
            .add_observer(server_spawner)
            .add_observer(peep_spawner)
            .add_observer(inspector_reach_peep)
            .add_observer(make_peep_dead)
            .add_observer(inspector_taken)
            .add_observer(inspector_target_die)
            .add_systems(OnExit(GameState::Started), reset)
        ;
    }
}

fn reset(
    mut spawner: ResMut<PeepSpawner>,
    mut i_spawner: ResMut<InspectorSpawner>
) {
    spawner.reset();
    i_spawner.reset();
}

fn cheats(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        commands.trigger(SpawnServerEvent);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        commands.trigger(SpawnPeepEvent);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        commands.trigger(SpawnInspectorEvent);
    }
}