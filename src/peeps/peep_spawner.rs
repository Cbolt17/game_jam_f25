use bevy::prelude::*;
use rand::Rng;

use crate::{grid::door::Door, peeps::{peeps::{Peep, PeepSheet}, profile::*}};

pub const ACCEL_INC: f32 = 20.0;
pub const ACCEL_RATE: f32 = 0.95;

#[derive(Resource)]
pub struct PeepSpawner {
    accel: Timer,
    rate: f32,
    countdown: f32,
}

#[derive(Event)]
pub struct SpawnPeepEvent;

impl PeepSpawner {
    pub fn new(rate: f32, countdown: f32) -> Self {
        PeepSpawner{accel: Timer::from_seconds(ACCEL_INC, TimerMode::Repeating), rate, countdown}
    }
    pub fn cycle_back(&mut self) {
        self.countdown += self.rate;
    }
    pub fn reset(&mut self) {
        self.countdown = self.rate;
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

pub fn peep_spawner_timer(
    mut spawner: ResMut<PeepSpawner>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if spawner.accel.tick(time.delta()).is_finished() {
        spawner.rate *= ACCEL_RATE;
    }
    spawner.countdown -= time.delta_secs();
    while spawner.countdown < 0.0 {
        spawner.cycle_back();
        commands.trigger(SpawnPeepEvent);
    }
}

pub fn peep_spawner(
    _event: On<SpawnPeepEvent>,
    door: Single<&Transform, With<Door>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    peep_sheet: Res<PeepSheet>,
) {
    spawn_peep(door.translation.xy(), &mut commands, &asset_server, &peep_sheet);
}