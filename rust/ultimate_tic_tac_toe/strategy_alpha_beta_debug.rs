use std::time::Duration;
use crate::ultimate_tic_tac_toe::ai::*;

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
        let time = Timer::new(&Duration::ZERO);
        let mut alpha = f32::MIN;
        let mut best: Option<Action> = None;
        for action in state.valid_actions() {
            let next = state.advanced(&action);
            let (score, hash) = alpha_beta(&next, f32::MIN, 0, -f32::MIN, 0, 0, depth, &timer);
            if -score == SCORE_WIN {
                return Some(action);
            }
            if timer.elapsed() {
                return None;
            }
            if logging {
                log!("{}-{}, {}, {:.8}, {:.8}, {:.8}, {:?}, {}", action.b, action.s, action.anywhere, action.score, score, calc_score(&next), time.time(), hash);
            }
            if alpha < -score {
                alpha = -score;
                best = Some(action);
            }
        }
        if logging {
            log!("action: {}", best.as_ref().map_or("None".to_string(), |a| format!("{}-{}", a.b, a.s)));
        }
        best
    }
}

pub fn alpha_beta(state: &State, mut alpha: f32, mut alpha_hash: u64, beta: f32, beta_hash: u64, depth: u8, max_depth: u8, timer: &Timer) -> (f32, u64) {
    if depth == max_depth || state.finished() {
        return (calc_score(state), state.hash);
    }
    if depth <= 3 && timer.elapsed() {
        return (0., 0);
    }
    for action in state.valid_actions() {
        let next = state.advanced(&action);
        let (score, hash) = alpha_beta(&next, -beta, beta_hash, -alpha, alpha_hash, depth + 1, max_depth, timer);
        if alpha < -score {
            alpha = -score;
            alpha_hash = hash;
        }
        if alpha >= beta {
            return (alpha, alpha_hash);
        }
    }
    (alpha, alpha_hash)
}

pub fn calc_score(state: &State) -> f32 {
    if is_win(state.big_win) {
        return SCORE_WIN;
    }
    if is_win(state.big_lose) {
        return SCORE_LOSE;
    }
    if state.big_win | state.big_lose | state.big_draw == 0x1ff {
        let count_win = popcount(state.big_win);
        let count_lose = popcount(state.big_lose);
        return if count_win > count_lose {
            SCORE_WIN
        } else if count_win < count_lose {
            SCORE_LOSE
        } else {
            0.
        }
    }
    let mut score = 0.;
    let mut draw_mask = 0_u16;
    let mut small_scores = [0.; 9];
    score += unsafe { SCORE[(state.big_win as usize) << 9 | state.big_lose as usize] };
    for i in 0..9 {
        let win = get_small(state.small_win, i as u8);
        let lose = get_small(state.small_lose, i as u8);
        if !is_win(win ^ 0x1ff) && !is_win(lose ^ 0x1ff) {
            draw_mask |= 1 << i;
        } else {
            small_scores[i] = unsafe { SCORE[(win as usize) << 9 | lose as usize] };
            score += small_scores[i] * [3., 2., 3., 2., 4., 2., 3., 2., 3.][i];
            small_scores[i] = (small_scores[i] + 1.) / 2.;
        }
    }
    #[inline]
    fn calc(a: f32, b: f32, c: f32) -> f32 {
        let p1 = a * b * c;
        let p2 = (1. - a) * (1. - b) * (1. - c);
        return if p1 > p2 { p1 } else { -p2 }
    }
    let line_scores = [
        if draw_mask & 0b000000111 == 0 { calc(small_scores[0], small_scores[1], small_scores[2]) } else { 0. },
        if draw_mask & 0b000111000 == 0 { calc(small_scores[3], small_scores[4], small_scores[5]) } else { 0. },
        if draw_mask & 0b111000000 == 0 { calc(small_scores[6], small_scores[7], small_scores[8]) } else { 0. },
        if draw_mask & 0b001001001 == 0 { calc(small_scores[0], small_scores[3], small_scores[6]) } else { 0. },
        if draw_mask & 0b010010010 == 0 { calc(small_scores[1], small_scores[4], small_scores[7]) } else { 0. },
        if draw_mask & 0b100100100 == 0 { calc(small_scores[2], small_scores[5], small_scores[8]) } else { 0. },
        if draw_mask & 0b100010001 == 0 { calc(small_scores[0], small_scores[4], small_scores[8]) } else { 0. },
        if draw_mask & 0b001010100 == 0 { calc(small_scores[2], small_scores[4], small_scores[6]) } else { 0. }
    ];
    score += line_scores[0] + line_scores[1] + line_scores[2] + line_scores[3] +
        line_scores[4] + line_scores[5] + line_scores[6] + line_scores[7];
    let min = line_scores[0].min(line_scores[1]).min(line_scores[2]).min(line_scores[3])
        .min(line_scores[4]).min(line_scores[5]).min(line_scores[6]).min(line_scores[7]);
    let max = line_scores[0].max(line_scores[1]).max(line_scores[2]).max(line_scores[3])
        .max(line_scores[4]).max(line_scores[5]).max(line_scores[6]).max(line_scores[7]);
    if min < 0. {
        score += min * 7.;
    }
    if max > 0. {
        score += max * 7.;
    }
    score
}
