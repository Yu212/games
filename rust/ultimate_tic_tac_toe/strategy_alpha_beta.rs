use crate::ultimate_tic_tac_toe::ai::{SCORE, ZOBRIST, State, Action, SCORE_WIN, Timer, log};

pub fn calc_action(state: &State, timer: &Timer, logging: bool) -> Action {
    if state.small_lose | state.small_win == 0 {
        return Action { b: 4, s: 4, anywhere: false, score: 0. };
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
    unsafe {
        let mut alpha = f32::MIN;
        let mut best: Option<Action> = None;
        for action in state.valid_actions() {
            let next = state.advanced(&action);
            let score = -alpha_beta(&next, f32::MIN, -alpha, depth, &timer);
            if score == SCORE_WIN {
                return Some(action);
            }
            if timer.elapsed() {
                return None;
            }
            if logging {
                log!("{}-{}, {}, {}", action.b, action.s, score, next.calc_score());
            }
            if alpha < score {
                alpha = score;
                best = Some(action);
            }
        }
        if logging {
            log!("action: {}", best.as_ref().map_or("None".to_string(), |a| format!("{}-{}", a.b, a.s)));
        }
        best
    }
}

pub fn alpha_beta(state: &State, mut alpha: f32, beta: f32, depth: u8, timer: &Timer) -> f32 {
    if timer.elapsed() {
        return 0.;
    }
    if depth == 0 || state.finished() {
        return state.calc_score();
    }
    for action in state.valid_actions() {
        let next = state.advanced(&action);
        let score = -alpha_beta(&next, -beta, -alpha, depth - 1, timer);
        if timer.elapsed() {
            break
        }
        if alpha < score {
            alpha = score;
        }
        if alpha >= beta {
            return alpha;
        }
    }
    alpha
}
