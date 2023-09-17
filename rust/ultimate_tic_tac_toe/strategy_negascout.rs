use std::time::Duration;
use crate::ultimate_tic_tac_toe::ai::*;

pub(crate) static mut TRANSPOSE_UPPER_HASH: [u64; 0x100000] = [0; 0x100000];
pub(crate) static mut TRANSPOSE_LOWER_HASH: [u64; 0x100000] = [0; 0x100000];
pub(crate) static mut TRANSPOSE_UPPER: [f32; 0x100000] = [0.; 0x100000];
pub(crate) static mut TRANSPOSE_LOWER: [f32; 0x100000] = [0.; 0x100000];
const TRANSPOSE_HASH_MASK: u64 = 0xfffff;

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
        let result = negascout_action(&state, depth, &timer, logging);
        if result.is_none() {
            break;
        }
        action = result;
    }
    action.unwrap()
}

pub fn negascout_action(state: &State, depth: u8, timer: &Timer, logging: bool) -> Option<Action> {
    unsafe {
        TRANSPOSE_UPPER_HASH.iter_mut().for_each(|x| *x = 0);
        TRANSPOSE_LOWER_HASH.iter_mut().for_each(|x| *x = 0);
    }
    unsafe {
        let time = Timer::new(&Duration::ZERO);
        let mut alpha = f32::MIN;
        let mut best: Option<Action> = None;
        for action in state.valid_actions_with_move_ordering() {
            let next = state.advanced(&action);
            let eval = -negascout(&next, f32::MIN, -alpha, 0, depth, &timer);
            if eval == SCORE_WIN {
                return Some(action);
            }
            if timer.elapsed() {
                return None;
            }
            if logging {
                log!("{}-{}, {}, {}, {:?}", action.b, action.s, eval, calc_eval(&next), time.time());
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
}

pub fn negascout(state: &State, mut alpha: f32, mut beta: f32, depth: u8, max_depth: u8, timer: &Timer) -> f32 {
    if depth == max_depth || state.finished() {
        return calc_eval(state);
    }
    if depth <= 3 && timer.elapsed() {
        return 0.;
    }
    let mut upper = f32::MAX;
    let mut lower = f32::MIN;
    let masked_hash = (state.hash & TRANSPOSE_HASH_MASK) as usize;
    unsafe {
        if TRANSPOSE_UPPER_HASH[masked_hash] == state.hash {
            upper = TRANSPOSE_UPPER[masked_hash];
        }
        if TRANSPOSE_LOWER_HASH[masked_hash] == state.hash {
            lower = TRANSPOSE_LOWER[masked_hash];
        }
    }
    if lower == upper {
        return upper;
    }
    alpha = alpha.max(lower);
    beta = beta.max(upper);
    let actions = state.valid_actions_with_move_ordering();
    let first = state.advanced(&actions[0]);
    let eval = -negascout(&first, -beta, -alpha, depth + 1, max_depth, timer);
    let mut max_eval = eval;
    if beta <= eval {
        return eval;
    }
    if alpha < eval {
        alpha = eval;
    }
    for action in actions.iter().skip(1) {
        let next = state.advanced(&action);
        let mut eval = -negascout(&next, -alpha - f32::EPSILON, -alpha, depth + 1, max_depth, timer);
        if beta <= eval {
            return eval;
        }
        if alpha < eval {
            alpha = eval;
            eval = -negascout(&next, -beta, -alpha, depth + 1, max_depth, timer);
            if beta <= eval {
                return eval;
            }
            if alpha < eval {
                alpha = eval;
            }
        }
        max_eval = max_eval.max(eval);
    }
    if max_eval < alpha {
        unsafe {
            TRANSPOSE_UPPER_HASH[masked_hash] = state.hash;
            TRANSPOSE_UPPER[masked_hash] = max_eval;
        }
    } else {
        unsafe {
            TRANSPOSE_UPPER_HASH[masked_hash] = state.hash;
            TRANSPOSE_UPPER[masked_hash] = max_eval;
            TRANSPOSE_LOWER_HASH[masked_hash] = state.hash;
            TRANSPOSE_LOWER[masked_hash] = max_eval;
        }
    }
    max_eval
}
