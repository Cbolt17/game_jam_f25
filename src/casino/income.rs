use bevy::prelude::*;

use crate::{casino::CasinoMoney, grid::play_attraction::BetResult};

pub fn casino_bet_result(
    bet_results: On<BetResult>,
    mut casino: ResMut<CasinoMoney>,
) {
    casino.0 -= bet_results.amt;
}