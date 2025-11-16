use bevy::prelude::*;
use rand::Rng;

use crate::{grid::door::Door, peeps::{peeps::{Peep, PeepSheet}, profile::*}};

#[derive(Resource)]
pub struct PeepSpawner {
    rate: f32,
    countdown: f32,
}

impl PeepSpawner {
    pub fn new(rate: f32, countdown: f32) -> Self {
        PeepSpawner{rate, countdown}
    }
    pub fn cycle_back(&mut self) {
        self.countdown += self.rate;
    }
}

pub fn spawn_peep(
    position: Vec2,
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    peep_sheet: &Res<PeepSheet>,
) {
    let mut random = rand::thread_rng();
    let money_profile = MoneyProfile(random.gen_range(100..500));
    let risk = match random.gen_range(0..3) {
        0=>{RiskProfile::Conservative}, 
        1=>{RiskProfile::Normal}, 
        _=>{RiskProfile::Risky}
    };
    let bets = BetProfile::new(5, random.gen_range(5..2000));
    commands.spawn((
        Peep,
        NoPlayRecord(0),
        Record::new(),
        Sprite {
            image: asset_server.load("PeepSheet.png"),
            texture_atlas: Some(TextureAtlas {
                layout: peep_sheet.0.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        money_profile,
        risk,
        bets
    ));
}

pub fn peep_spawner(
    mut spawner: ResMut<PeepSpawner>,
    peep_sheet: Res<PeepSheet>,
    door: Single<&Transform, With<Door>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawner.countdown -= time.delta_secs();
    while spawner.countdown < 0.0 {
        spawner.cycle_back();
        spawn_peep(door.translation.xy(), &mut commands, &asset_server, &peep_sheet);
    }
}