use bevy::prelude::*;

#[derive(Component)]
pub enum RiskProfile {
    Conservative,
    Normal,
    Risky
}

#[derive(Component)]
pub struct MoneyProfile {
    amt: i64
}

#[derive(Component)]
pub struct BetProfile {
    min_bet: i64,
    max_bet: i64
}