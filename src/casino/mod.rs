use bevy::prelude::*;
use bevy::app::Plugin;

pub struct CasinoPlugin;

#[derive(Resource)]
pub struct CasinoMoney(pub i64);

#[derive(Resource)]
pub struct Suspicion(pub f32);

#[derive(Resource)]
pub struct CasinoCapacity {
    pub current: u32,
    pub max: u32
}

impl Plugin for CasinoPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CasinoMoney(20000))
            .insert_resource(Suspicion(0.05))
            .insert_resource(CasinoCapacity{current: 0, max: 25})
        ;
    }
}
