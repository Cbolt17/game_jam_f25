use bevy::prelude::*;
use rand::Rng;

use crate::peeps::play::GoTo;

const HALF_PI: f32 = 3.141592652589 / 2.0;

#[derive(Component)]
pub struct Drunk(pub u64);

#[derive(Component)]
pub struct PassOut;

#[derive(Component)]
pub struct DrunkTimer(pub Timer);

impl DrunkTimer {
    pub fn new() -> Self {
        DrunkTimer(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

pub fn add_drunk_timer(
    add: On<Add, Drunk>,
    mut commands: Commands,
) {
    commands.entity(add.entity).insert(DrunkTimer::new());
}

pub fn passout_chance(
    mut peep_query: Query<(Entity, &Drunk, &mut DrunkTimer), With<GoTo>>,
    mut commands: Commands,
    time: Res<Time>
) {
    let mut random = rand::thread_rng();
    for (entity, drunk, mut timer) in peep_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if random.gen_range(0..10) < drunk.0 {
                commands.entity(entity).insert(PassOut);
            }
        }
    }
}

pub fn peep_passout(
    passout: On<Add, PassOut>,
    mut query: Query<&mut Transform, With<Drunk>>,
) {
    if let Ok(mut transform) = query.get_mut(passout.entity) {
        let mut random = rand::thread_rng();
        let rads = if random.gen_bool(0.5) {-HALF_PI} else {HALF_PI};
        transform.rotate_z(rads);
    }
}