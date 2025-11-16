use std::collections::VecDeque;

use bevy::{math::U64Vec2, prelude::*};
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub enum RiskProfile {
    Conservative,
    Normal,
    Risky
}
#[derive(Component)]
pub struct Record(VecDeque<i64>);

impl Record {
    pub fn update(&mut self, amt: i64) {
        self.0.push_back(amt);
        if self.0.len() > 10 {
            self.0.pop_front();
        }
    }
    pub fn quit(&self, quit_rate: f32) -> bool {
        let mut rate = 0.0;
        for val in self.0.iter() {
            rate += if *val >= 0 {1.0} else {-1.0};
        }
        rate /= self.0.len() as f32;
        quit_rate >= rate
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
    pub fn bet(&self, risk: RiskProfile, money: u64, bounds: U64Vec2) -> u64 {
        let mut random = rand::thread_rng();
        let weight = match risk {
            RiskProfile::Conservative => random.gen_range(0.0_f32..1.0_f32).min(random.gen_range(0.0_f32..1.0_f32)),
            RiskProfile::Normal =>       random.gen_range(0.0_f32..1.0_f32),
            RiskProfile::Risky =>        random.gen_range(0.0_f32..1.0_f32).max(random.gen_range(0.0_f32..1.0_f32)),
        };
        (bounds.x as f32).lerp(bounds.y as f32, weight) as u64
    }
}