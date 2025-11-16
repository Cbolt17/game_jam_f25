use bevy::prelude::*;

use crate::{casino::{CasinoMoney, Suspicion, TimeLimit}, game::GameState};

const CASH_TO_WIN: i64 = 1000000;

#[derive(Event)]
pub struct YouWonEvent;

#[derive(Event)]
pub enum YouLoseEvent {
    BankRupt,
    OutOfTime,
    Arrested
}

pub fn end_check(
    money: Res<CasinoMoney>,
    sus: Res<Suspicion>,
    limit: Res<TimeLimit>,
    mut next_state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    mut commands: Commands,
) {
    if money.0 > CASH_TO_WIN {
        time.pause();
        *next_state = NextState::Pending(GameState::Paused);
        commands.trigger(YouWonEvent);
    }
    if money.0 < 0 {
        time.pause();
        *next_state = NextState::Pending(GameState::Paused);
        commands.trigger(YouLoseEvent::BankRupt);
    }
    if sus.0 >= 1.0 {
        time.pause();
        *next_state = NextState::Pending(GameState::Paused);
        commands.trigger(YouLoseEvent::Arrested);
    }
    if limit.0 <= 0.0 {
        time.pause();
        *next_state = NextState::Pending(GameState::Paused);
        commands.trigger(YouLoseEvent::OutOfTime);
    }
}