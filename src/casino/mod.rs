use bevy::prelude::*;
use bevy::app::Plugin;

use crate::casino::income::casino_bet_result;
use crate::game::GameState;

const START_CASH: i64 = 200000;
const TIME_LIMIT: f32 = 30.0;

mod income;

pub struct CasinoPlugin;

#[derive(Resource)]
pub struct CasinoMoney(pub i64);

#[derive(Resource)]
pub struct Suspicion(pub f32);

#[derive(Resource)]
pub struct PeepCount(pub u32);

#[derive(Resource)]
pub struct TimeLimit(pub f32);

impl Plugin for CasinoPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TimeLimit(TIME_LIMIT)) // 30 min
            .insert_resource(CasinoMoney(START_CASH))
            .insert_resource(Suspicion(0.00))
            .insert_resource(PeepCount(0))
            .add_observer(casino_bet_result)
            .add_systems(OnExit(GameState::Started), reset)
            .add_systems(Update, time_tick)
        ;
    }
}

fn time_tick(
    mut time_limit: ResMut<TimeLimit>,
    time: Res<Time>,
) {
    time_limit.0 -= time.delta_secs() / 60.0;
    if time_limit.0 < 0.0 {
        time_limit.0 = 0.0;
    }
}

fn reset(
    mut casino: ResMut<CasinoMoney>,
    mut peep_count: ResMut<PeepCount>,
    mut suspicion: ResMut<Suspicion>,
    mut time_limit: ResMut<TimeLimit>
) {
    casino.0 = 20000;
    peep_count.0 = 0;
    suspicion.0 = 0.05;
    time_limit.0 = 30.0;
}