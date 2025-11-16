use std::collections::VecDeque;

use bevy::{math::U64Vec2, prelude::*};
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub enum RiskProfile {
    Conservative,
    Normal,
    Risky
}

pub enum RecordOpinion {
    NewGame,
    Leave,
    Stay,
}

#[derive(Component)]
pub struct Record(VecDeque<i64>);

impl Record {
    pub fn new() -> Self {
        Record(VecDeque::new())
    }
    pub fn update(&mut self, amt: i64) {
        self.0.push_back(amt);
        if self.0.len() > 10 {
            self.0.pop_front();
        }
    }
    pub fn opinion(&self) -> RecordOpinion {
        let mut rate = 0.0;
        for val in self.0.iter() {
            rate += if *val >= 0 {1.0} else {-1.0};
        }
        let len = self.0.len();
        rate /= len as f32;
        if rate > 0.7 && len > 5 {
            RecordOpinion::Leave
        }
        else if rate < 0.7 && len > 7 {
            RecordOpinion::Leave
        }
        else if rate < 0.7 && len > 3 {
            RecordOpinion::NewGame
        }
        else {
            RecordOpinion::Stay
        }
    }
}

#[derive(Component)]
pub struct NoPlayRecord(pub u64);

impl NoPlayRecord {
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

#[derive(EntityEvent)]
pub struct FoundFullEvent {
    pub entity: Entity
}

pub fn found_full_observer(
    full: On<FoundFullEvent>,
    mut peeps_query: Query<&mut NoPlayRecord>,
) {
    if let Ok(mut record) = peeps_query.get_mut(full.entity) {
        record.0 += 1;
    }
}

#[derive(Component)]
pub struct MoneyProfile(pub u64);

#[derive(Component)]
pub struct BetProfile {
    min_bet: u64,
    max_bet: u64
}

impl BetProfile {
    pub fn new(min_bet: u64, max_bet: u64) -> Self {
        BetProfile { min_bet, max_bet }
    }
    pub fn bounds(&self, money: u64, min_bet: u64, max_bet: u64) -> U64Vec2 {
        U64Vec2::new(
            self.min_bet.max(min_bet),
            self.max_bet.min(max_bet).min(money)
        )
    }
    pub fn bet(&self, risk: RiskProfile, bounds: U64Vec2) -> u64 {
        let mut random = rand::thread_rng();
        let weight = match risk {
            RiskProfile::Conservative => random.gen_range(0.0_f32..1.0_f32).min(random.gen_range(0.0_f32..1.0_f32)),
            RiskProfile::Normal =>       random.gen_range(0.0_f32..1.0_f32),
            RiskProfile::Risky =>        random.gen_range(0.0_f32..1.0_f32).max(random.gen_range(0.0_f32..1.0_f32)),
        };
        (bounds.x as f32).lerp(bounds.y as f32, weight) as u64
    }
}