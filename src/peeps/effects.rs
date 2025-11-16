use bevy::prelude::*;
use rand::Rng;

use crate::{grid::play_attraction::BetResult, peeps::peeps::Peep};

const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);

const TEXT_SPEED: f32 = 30.0;
const TEXT_LIFETIME: f32 = 0.75;

#[derive(Component)]
pub struct Despawner{
    pub speed: f32,
    pub timer: Timer,
    pub dir: Vec2
}

impl Despawner {
    pub fn new(speed: f32, time: f32) -> Self {
        let mut random = rand::thread_rng();
        Despawner{
            speed,
            timer: Timer::from_seconds(time, TimerMode::Once),
            dir:  Vec2::new(
                random.gen_range(-0.5..0.5),
                random.gen_range(0.5..1.0),
            )
        }
    }
}

pub fn bet_effect(
    bet_results: On<BetResult>,
    peep_query: Query<&Transform, With<Peep>>,
    mut commands: Commands,
) {
    if let Ok(transform) = peep_query.get(bet_results.entity) {
        let color = if bet_results.amt > 0 {RED} else {GREEN};
        let mut pos = transform.translation;
        pos.y += 16.0; // Peep size is 16
        let amt = bet_results.amt.abs().to_string();
        let text = commands.spawn((
            Text2d::new(amt.clone()),
            TextColor(color),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(pos)
                .with_scale(Vec3::splat(0.4)), // original size
            Despawner::new(TEXT_SPEED, TEXT_LIFETIME),
        )).id();
        // Add Black border
        let bg = commands.spawn((
            Text2d::new(amt.clone()),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(-2., 0.0, -0.001))
        )).id();
        commands.entity(text).add_child(bg);
        let bg = commands.spawn((
            Text2d::new(amt.clone()),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(2., 0.0, -0.001))
        )).id();
        commands.entity(text).add_child(bg);
        let bg = commands.spawn((
            Text2d::new(amt.clone()),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(0.0, 2., -0.001))
        )).id();
        commands.entity(text).add_child(bg);
        let bg = commands.spawn((
            Text2d::new(amt),
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_translation(Vec3::new(0.0, -2., -0.001))
        )).id();
        commands.entity(text).add_child(bg);
    }
}

pub fn move_bet_text(
    mut bet_text: Query<(&mut Transform, &Despawner)>,
    time: Res<Time>,
) {
    for (mut transform, text) in bet_text.iter_mut() {
        transform.translation = transform.translation + (text.dir * text.speed * time.delta_secs()).extend(0.1);
    }
}

pub fn despawn_bet_text(
    mut bet_text: Query<(Entity, &mut Despawner)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut text) in bet_text.iter_mut() {
        if text.timer.tick(time.delta()).is_finished() {
            let Ok(mut entity) = commands.get_entity(entity) else {
                continue;
            };
            entity.despawn();
        }
    }
}