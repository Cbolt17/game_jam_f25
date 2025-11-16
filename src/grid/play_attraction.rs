use bevy::prelude::*;
use rand::Rng;

use crate::grid::attraction::*;
use crate::peeps::drunk::Drunk;
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
    mut attraction_query: Query<(&mut Attraction, &AttractionType, &mut AttractionCooldown, &Players)>,
    mut peep_query: Query<(Entity, &MoneyProfile, &BetProfile, &RiskProfile, &mut NoPlayRecord, &mut Record, Option<&mut Drunk>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut random = rand::thread_rng();
    for (mut attraction, a_type, mut cooldown, players) in attraction_query.iter_mut() {
        if players.len() == 0 {
            continue;
        }
        if cooldown.0.tick(time.delta()).just_finished() {
            match a_type {
                AttractionType::Roulette => {
                    for player in players.get_players().iter() {
                        let (peep_entity, money, betting, risk, mut play_record, mut record, drunk) = 
                            peep_query.get_mut(*player).unwrap();
                        let bounds = betting.bounds(money.0, attraction.min_bet(), attraction.max_bet());
                        // Try to bet
                        if bounds.x <= bounds.y {
                            play_record.reset();
                            let bet = betting.bet(*risk, bounds);
                            let mut outcome = attraction.win_rate();
                            if let Some(drunk) = drunk {
                                outcome += drunk.0 as f32 * 5.0;
                            }
                            outcome = outcome.clamp(0.01, 0.99);
                            // From peep perspective (negative is lost)
                            let bet = if random.gen_bool(outcome as f64) {
                                -(bet as i64)
                            } else {
                                bet as i64
                            };
                            commands.trigger(BetResult::new(peep_entity, bet));
                            record.update(bet);
                        }
                        else {
                            play_record.0 += 1;
                            commands.entity(peep_entity).remove::<Playing>();
                            attraction.remove_player();
                        }
                    }
                },
                AttractionType::BlackJack => {
                    // Get random player
                    let rand = random.gen_range(0..players.len());
                    let (peep_entity, money, betting, risk, mut play_record, mut record, drunk) = 
                        peep_query.get_mut(players.get_players()[rand]).unwrap();
                    let bounds = betting.bounds(money.0, attraction.min_bet(), attraction.max_bet());
                    // Try to bet
                    if bounds.x <= bounds.y {
                        play_record.reset();
                        let bet = betting.bet(*risk, bounds);
                        let mut outcome = attraction.win_rate();
                        if let Some(drunk) = drunk {
                            outcome += drunk.0 as f32 * 5.0;
                        }
                        outcome = outcome.clamp(0.01, 0.99);
                        // From peep perspective (negative is lost)
                        let bet = if random.gen_bool(outcome as f64) {
                            -(bet as i64)
                        } else {
                            bet as i64
                        };
                        commands.trigger(BetResult::new(peep_entity, bet));
                        record.update(bet);
                    }
                    else {
                        play_record.0 += 1;
                        commands.entity(peep_entity).remove::<Playing>();
                        attraction.remove_player();
                    }
                },
                AttractionType::Bar => {
                    // Get random player
                    let rand = random.gen_range(0..players.len());
                    let (peep_entity, _, _, _, mut play_record, _, drunk) = 
                        peep_query.get_mut(players.get_players()[rand]).unwrap();
                    play_record.reset();
                    if let Some(mut drunk) = drunk {
                        drunk.0 += 1;
                    }
                    else {
                        commands.entity(peep_entity).insert(Drunk(1));
                    }
                    commands.entity(peep_entity).remove::<Playing>();
                    attraction.remove_player();
                },
            }
        }
    }
}