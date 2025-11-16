use bevy::prelude::*;
use bevy::app::Plugin;

use crate::casino::income::casino_bet_result;

mod income;

pub struct CasinoPlugin;

#[derive(Resource)]
pub struct CasinoMoney(pub i64);

#[derive(Resource)]
pub struct Suspicion(pub f32);

#[derive(Resource)]
pub struct PeepCount(pub u32);

impl Plugin for CasinoPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CasinoMoney(20000))
            .insert_resource(Suspicion(0.05))
            .insert_resource(PeepCount(0))
            .add_observer(casino_bet_result)
        ;
    }
}
