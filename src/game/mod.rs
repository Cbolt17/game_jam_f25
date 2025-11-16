use bevy::prelude::*;
use bevy::app::Plugin;

use crate::game::end::end_check;

mod end;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(OnExit(GameState::Started), clear_all)
            .add_systems(OnEnter(GameState::Paused), game_menu)
            .add_systems(Startup, game_menu)
            .add_systems(Update, start_game.run_if(in_state(GameState::Paused)))
            .add_systems(Update, end_check)
        ;
    }
}

fn game_menu(
    mut time: ResMut<Time<Virtual>>,
) {
    time.pause();
}

fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        time.unpause();
        *next_state =  //NextState::Pending(GameState::Started);
        match state.get() {
            GameState::Paused => NextState::Pending(GameState::Started),
            GameState::Started => NextState::Pending(GameState::Paused),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Paused,
    Started,
}

pub fn clear_all(
    query: Query<Entity, (With<Transform>, Without<Camera>)>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}