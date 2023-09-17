mod ultimate_tic_tac_toe;

use std::time::Duration;
use crate::ultimate_tic_tac_toe::ai::*;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta::calc_action as alpha_beta_action;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta_2::calc_action as alpha_beta_2_action;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta_transpose::calc_action as alpha_beta_transpose_action;
use crate::ultimate_tic_tac_toe::strategy_mcts::calc_action as mcts_action;
use crate::ultimate_tic_tac_toe::strategy_random::calc_action as random_action;
use crate::ultimate_tic_tac_toe::strategy_negascout::calc_action as negascout_action;

fn main() {
    init();
    let mut n = 0;
    let mut w = 0;
    for _ in 0..50 {
        let player_1: fn(&State) -> Action = |state| alpha_beta_action(state, &Timer::new(&Duration::from_millis(50)), true);
        let player_2: fn(&State) -> Action = |state| mcts_action(state, &Timer::new(&Duration::from_millis(250)), false);
        n += 2;
        println!("{} {:3} tries {:5.1} %", match play(&player_1, &player_2) {
            Winner::First => { w += 2; "1p  (first)" },
            Winner::Second => "2p (second)",
            Winner::Draw => { w += 1; "draw       " },
        }, n / 2, 100. * w as f32 / n as f32);
        n += 2;
        println!("{} {:3} tries {:5.1} %", match play(&player_2, &player_1) {
            Winner::First => "2p  (first)",
            Winner::Second => { w += 2; "1p (second)" },
            Winner::Draw => { w += 1; "draw       " },
        }, n / 2, 100. * w as f32 / n as f32);
    }
}

fn play(player_1: &fn(&State) -> Action, player_2: &fn(&State) -> Action) -> Winner {
    let mut state = State::new();
    let mut turn = true;
    loop {
        let action = if turn {
            player_1(&state)
        } else {
            player_2(&state)
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
