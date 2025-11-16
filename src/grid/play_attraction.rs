use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::grid::attraction::*;
use crate::peeps::peeps::*;
use crate::peeps::play::*;
use crate::peeps::profile::*;

#[derive(EntityEvent)]
pub struct BetResult {
    pub entity: Entity,
    pub amt: i64,
}

impl BetResult {
    pub fn new(entity: Entity, amt: i64) -> Self {
        BetResult{entity, amt}
    }
}

pub fn play_game(
    mut attraction_query: Query<(&Attraction, &AttractionType, &mut AttractionCooldown, &Players)>,
    peep_query: Query<(Entity, &MoneyProfile, &BetProfile, &RiskProfile), With<Peep>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut random = rand::thread_rng();
    for (attraction, a_type, mut cooldown, players) in attraction_query.iter_mut() {
        if cooldown.0.tick(time.delta()).just_finished() {
            match a_type {
                AttractionType::Roulette => {
                    for player in players.get_players().iter() {
                        if let Ok((peep_entity, money, betting, risk)) = peep_query.get(*player) {
                            let bounds = betting.bounds(money.0, attraction.min_bet(), attraction.max_bet());
                            // Try to bet
                            if bounds.x <= bounds.y {
                                let bet = betting.bet(*risk, money.0, bounds);
                                let bet = if random.gen_range(0.0_f32..1.0_f32) < attraction.win_rate() {
                                    -(bet as i64)
                                } else {
                                    bet as i64
                                };
                                commands.trigger(BetResult::new(peep_entity, bet));
                            }
                            else {
                                commands.entity(peep_entity).remove::<Playing>();
                            }
                        }
                    }
                },
                AttractionType::BlackJack => {
                    // Get random player
                    let rand = random.gen_range(0..players.len());
                    let (peep_entity, money, betting, risk) = 
                        peep_query.get(players.get_players()[rand]).unwrap();
                    let bounds = betting.bounds(money.0, attraction.min_bet(), attraction.max_bet());
                    // Try to bet
                    if bounds.x <= bounds.y {
                        let bet = betting.bet(*risk, money.0, bounds);
                        let bet = if random.gen_range(0.0_f32..1.0_f32) < attraction.win_rate() {
                            -(bet as i64)
                        } else {
                            bet as i64
                        };
                        commands.trigger(BetResult::new(peep_entity, bet));
                    }
                    else {
                        commands.entity(peep_entity).remove::<Playing>();
                    }
                },
                AttractionType::Bar => {
                    // Get random player
                    let rand = random.gen_range(0..players.len());
                    let (peep_entity, money, betting, risk) = 
                        peep_query.get(players.get_players()[rand]).unwrap();
                    let bounds = betting.bounds(money.0, attraction.min_bet(), attraction.max_bet());
                    // Try to bet
                    if bounds.x <= bounds.y {
                        let bet = betting.bet(*risk, money.0, bounds) as i64;
                        commands.trigger(BetResult::new(peep_entity, bet));
                    }
                    else {
                        commands.entity(peep_entity).remove::<Playing>();
                    }
                },
            }
        }
    }
}