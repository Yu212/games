use std::time::Duration;
use itertools::Itertools;
use crate::ultimate_tic_tac_toe::ai::*;

pub fn calc_action(state: &State, timer: &Timer, logging: bool) -> Action {
    if state.small_lose | state.small_win == 0 {
        return Action { b: 4, s: 4, anywhere: false, eval: 0. };
    }
    let mut action = None;
    for depth in 3..20 {
        if logging {
            log!("{}", depth);
        }
        if timer.elapsed() {
            break
        }
        let result = alpha_beta_action(&state, depth, &timer, logging);
        if result.is_none() {
            break;
        }
        action = result;
    }
    action.unwrap()
}

pub fn alpha_beta_action(state: &State, depth: u8, timer: &Timer, logging: bool) -> Option<Action> {
    let time = Timer::new(&Duration::ZERO);
    let mut alpha = f32::MIN;
    let mut best: Option<Action> = None;
    for action in state.valid_actions_with_move_ordering() {
        let next = state.advanced(&action);
        let eval = -alpha_beta(&next, f32::MIN, -alpha, 0, depth, &timer);
        if eval == SCORE_WIN {
            return Some(action);
        }
        if timer.elapsed() {
            return None;
        }
        if logging {
            log!("{}-{}, {}, {:.8}, {:.8}, {:.8}, {:?}", action.b, action.s, action.anywhere, action.eval, eval, calc_eval(&next), time.time());
        }
        if alpha < eval {
            alpha = eval;
            best = Some(action);
        }
    }
    if logging {
        log!("action: {}", best.as_ref().map_or("None".to_string(), |a| format!("{}-{}", a.b, a.s)));
    }
    best
}

pub fn alpha_beta(state: &State, mut alpha: f32, beta: f32, depth: u8, max_depth: u8, timer: &Timer) -> f32 {
    if depth == max_depth || state.finished() {
        return calc_eval(state);
    }
    if depth <= 3 && timer.elapsed() {
        return 0.;
    }
    let mut max_eval = f32::MIN;
    for action in state.valid_actions_with_move_ordering() {
        let next = state.advanced(&action);
        let eval = -alpha_beta(&next, -beta, -alpha, depth + 1, max_depth, timer);
        if eval >= beta {
            return eval;
        }
        alpha = alpha.max(eval);
        max_eval = max_eval.max(eval);
    }
    max_eval
}

#[cfg(target_arch = "wasm32")]
pub fn calc_deep_eval(state: &State, depth: u8) -> f32 {
    -alpha_beta(state, f32::MIN, f32::MAX, 0, depth, &Timer::infinity())
}
