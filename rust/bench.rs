mod ultimate_tic_tac_toe;

use std::time::Duration;
use crate::ultimate_tic_tac_toe::ai::{init, State, Timer, Action, MatchResult};
use crate::ultimate_tic_tac_toe::strategy_alpha_beta::calc_action as alpha_beta_action;
use crate::ultimate_tic_tac_toe::strategy_mcts::calc_action as mcts_action;
use crate::ultimate_tic_tac_toe::strategy_random::calc_action as random_action;

fn main() {
    init();
    loop {
        println!("{:?}", play());
    }
}

fn play() -> Winner {
    let mut state = State::new();
    let mut turn = true;
    loop {
        let mut time_limit = Duration::from_millis(100);
        let timer = Timer::new(&time_limit);
        let action = if turn {
            mcts_action(&state, &timer, false)
        } else {
            alpha_beta_action(&state, &timer, false)
        };
        turn = !turn;
        state = state.advanced(&action);
        return match state.winner() {
            Some(MatchResult::Win) if turn => Winner::First,
            Some(MatchResult::Lose) if turn => Winner::Second,
            Some(MatchResult::Win) => Winner::Second,
            Some(MatchResult::Lose) => Winner::First,
            Some(MatchResult::Draw) => Winner::Draw,
            _ => continue,
        }
    }
}

#[derive(Debug)]
pub enum Winner {
    First, Second, Draw
}
