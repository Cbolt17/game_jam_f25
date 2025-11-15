use bevy::prelude::*;

#[derive(Resource)]
pub struct MoneyBellHopAmount(pub i64);

#[derive(Component)]
pub struct MoneyBellHop {
    cooldown: Timer,
}