use bevy::prelude::*;
use bevy::app::Plugin;

pub struct CasinoPlugin;

#[derive(Resource)]
pub struct CasinoMoney(pub i64);

impl Plugin for CasinoPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CasinoMoney(20000))
        ;
    }
}
