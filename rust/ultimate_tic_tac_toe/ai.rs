use std::{fmt, io};
use std::fmt::Formatter;
use std::time::Duration;
use std::ops::Add;
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
pub struct Timer(f64);
#[cfg(target_arch = "wasm32")]
impl Timer {
    pub fn new(time_limit: &Duration) -> Self {
        Timer(crate::performance.now() + time_limit.as_secs_f64() * 1000.)
    }

    pub fn elapsed(&self) -> bool {
        crate::performance.now() > self.0
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Timer(Instant);
#[cfg(not(target_arch = "wasm32"))]
impl Timer {
    pub fn new(time_limit: &Duration) -> Self {
        Timer(Instant::now().add(*time_limit))
    }

    pub fn elapsed(&self) -> bool {
        self.0.elapsed() > Duration::ZERO
    }
}

static mut SCORE: [f32; 0x40000] = [0.; 0x40000];

pub unsafe fn init() {
    for me in (0..0x200 as u16).rev() {
        let mut op = !me & 0x1ff;
        loop {
            let mask = (me as usize) << 9 | op as usize;
            if is_win(me) {
                SCORE[mask] = 1.;
            } else if is_win(op) {
                SCORE[mask] = -1.;
            } else if !is_win(!me & 0x1ff) && !is_win(!op & 0x1ff) {
                SCORE[mask] = 0.;
            } else {
                let mut valid_me = Vec::new();
                let mut valid_op = Vec::new();
                for i in 0..9 {
                    if (me >> i | op >> i) & 1 == 1 {
                        continue;
                    }
                    valid_me.push(SCORE[(mask | 1 << i + 9) as usize]);
                    valid_op.push(SCORE[(mask | 1 << i) as usize]);
                }
                valid_me.sort_by(|a, b| b.partial_cmp(a).unwrap());
                valid_op.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut value_me = 0.;
                let mut value_op = 0.;
                let mut w = 1.;
                let mut w_sum = 0.;
                for (a, b) in valid_me.iter().zip(valid_op.iter()) {
                    value_me += a * w;
                    value_op += b * w;
                    w_sum += w;
                    w /= 4.;
                }
                SCORE[mask] = ((value_me + value_op) / w_sum / 2. as f32).clamp(-1., 1.);
            }
            if op == 0 {
                break;
            }
            op = (op - 1) & !me;
        }
    }
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    unsafe {
        init();
    }
    let mut state = State::new();
    let mut time_limit = Duration::from_millis(700);
    loop {
        let opponent_action = in_action();
        let timer = Timer::new(&time_limit);
        time_limit = Duration::from_millis(95);
        if let Some(action) = opponent_action {
            state = state.advanced(&action);
        } else {
            let action = Action { b: 4, s: 4, anywhere: false, score: 0. };
            state = state.advanced(&action);
            println!("{}", action);
            continue;
        }
        let mut action = None;
        for depth in 4..20 {
            eprintln!("{}", depth);
            if timer.elapsed() {
                break
            }
            let result = alpha_beta_action(&state, depth, &timer);
            if result.is_none() {
                break;
            }
            action = result;
        }
        println!("{}", action.as_ref().unwrap());
        state = state.advanced(&action.unwrap());
    }
}

fn in_action() -> Option<Action> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let y = parse_input!(inputs[0], i32);
    let x = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let valid_action_count = parse_input!(input_line, usize);
    for _ in 0..valid_action_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
    }
    if x == -1 {
        None
    } else {
        Some(Action {
            b: (y / 3 * 3 + x / 3) as u8,
            s: (y % 3 * 3 + x % 3) as u8,
            anywhere: false,
            score: 0.,
        })
    }
}

#[inline]
fn get_small(table: u128, b: u8) -> u16 {
    (table >> b * 9 & 0b111111111) as u16
}

#[inline]
fn is_win(table: u16) -> bool {
    table & 0b100100100 == 0b100100100 || table & 0b010010010 == 0b010010010 ||
    table & 0b001001001 == 0b001001001 || table & 0b111000000 == 0b111000000 ||
    table & 0b000111000 == 0b000111000 || table & 0b000000111 == 0b000000111 ||
    table & 0b100010001 == 0b100010001 || table & 0b001010100 == 0b001010100
}

pub fn alpha_beta_action(state: &State, depth: u8, timer: &Timer) -> Option<Action> {
    unsafe {
        let mut alpha = f32::MIN;
        let mut best: Option<Action> = None;
        for action in state.valid_actions() {
            let next = state.advanced(&action);
            let score = -alpha_beta(&next, f32::MIN, -alpha, depth, &timer);
            if timer.elapsed() {
                return None;
            }
            // crate::log!("{:?}, {}, {}", action, score, next.calc_score());
            if alpha < score {
                alpha = score;
                best = Some(action);
            }
        }
        // crate::log!("{:?}", best);
        best
    }
}

pub unsafe fn alpha_beta(state: &State, mut alpha: f32, beta: f32, depth: u8, timer: &Timer) -> f32 {
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

#[derive(Debug)]
pub struct Action {
    pub b: u8,
    pub s: u8,
    pub anywhere: bool,
    pub score: f32,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.b / 3 * 3 + self.s / 3, self.b % 3 * 3 + self.s % 3)
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub small_win: u128,
    pub small_lose: u128,
    pub big_win: u16,
    pub big_lose: u16,
    pub big_draw: u16,
    pub last_big: i8,
}

impl State {
    fn new() -> Self {
        State {
            small_win: 0,
            small_lose: 0,
            big_win: 0,
            big_lose: 0,
            big_draw: 0,
            last_big: -1,
        }
    }

    pub fn advanced(&self, action: &Action) -> Self {
        let small_win = self.small_lose;
        let mut small_lose = self.small_win;
        let big_win = self.big_lose;
        let mut big_lose = self.big_win;
        let mut big_draw = self.big_draw;
        small_lose |= 1_u128 << (action.b * 9 + action.s);
        if is_win(get_small(small_lose, action.b)) {
            big_lose |= 1 << action.b;
        } else if get_small(small_win | small_lose, action.b) == 0x1ff {
            big_draw |= 1 << action.b;
        }
        let free_big = (big_win | big_lose | big_draw) >> action.s & 1 == 1;
        let last_big = if free_big { -1 } else { action.s as i8 };
        State {
            small_win,
            small_lose,
            big_win,
            big_lose,
            big_draw,
            last_big,
        }
    }

    pub unsafe fn valid_actions(&self) -> Vec<Action> {
        if self.last_big != -1 {
            let small_win = get_small(self.small_win, self.last_big as u8);
            let small_lose = get_small(self.small_lose, self.last_big as u8);
            let remain = !small_win & !small_lose & 0x1ff;
            let mut actions = Vec::with_capacity(9);
            for s in 0..9 {
                if remain >> s & 1 == 1 {
                    let win = get_small(self.small_win, self.last_big as u8);
                    let lose = get_small(self.small_lose, self.last_big as u8);
                    let mut anywhere = self.big_win | self.big_lose | self.big_draw >> s & 1 == 1;
                    if !anywhere && self.last_big == s && is_win(win | 1 << s) || win | lose == 0x1ff {
                        anywhere = true;
                    }
                    actions.push(Action {
                        b: self.last_big as u8,
                        s: s as u8,
                        anywhere,
                        score: SCORE[(win as usize) << 9 | lose as usize | 1 << (s + 9)]
                    })
                }
            }
            actions.sort_unstable_by(|a, b| a.anywhere.cmp(&b.anywhere).then(b.score.partial_cmp(&a.score).unwrap()));
            actions
        } else {
            let remain_big = !self.big_win & !self.big_lose & !self.big_draw & 0x1ff;
            let remain_small = !self.small_win & !self.small_lose & 0x1ffffffffffffffffffff;
            let mut actions = Vec::with_capacity(81);
            for b in 0..9 {
                if remain_big >> b & 1 == 0 {
                    continue;
                }
                for s in 0..9 {
                    let win = get_small(self.small_win, b);
                    let lose = get_small(self.small_lose, b);
                    let mut anywhere = self.big_win | self.big_lose | self.big_draw >> s & 1 == 1;
                    if !anywhere && b == s && is_win(win | 1 << s) || win | lose == 0x1ff {
                        anywhere = true;
                    }
                    if remain_small >> (b * 9 + s) & 1 == 1 {
                        actions.push(Action {
                            b,
                            s,
                            anywhere,
                            score: SCORE[(win as usize) << 9 | lose as usize | 1 << (s + 9)]
                        })
                    }
                }
            }
            actions.sort_unstable_by(|a, b| a.anywhere.cmp(&b.anywhere).then(b.score.partial_cmp(&a.score).unwrap()));
            actions
        }
    }

    pub unsafe fn calc_score(&self) -> f32 {
        if is_win(self.big_win) {
            return 100.;
        }
        if is_win(self.big_lose) {
            return -100.;
        }
        if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            let count_win = self.big_win.count_ones();
            let count_lose = self.big_lose.count_ones();
            return if count_win > count_lose {
                100.
            } else if count_win < count_lose {
                -100.
            } else {
                0.
            }
        }
        let mut score = 0.;
        let mut draw_mask = 0_u16;
        let mut small_scores = [0.; 9];
        score += SCORE[(self.big_win as usize) << 9 | self.big_lose as usize];
        for i in 0..9 {
            let win = get_small(self.small_win, i as u8);
            let lose = get_small(self.small_lose, i as u8);
            if !is_win(!win & 0x1ff) && !is_win(!lose & 0x1ff) {
                draw_mask |= 1 << i;
            } else {
                small_scores[i] = SCORE[(win as usize) << 9 | lose as usize];
                score += small_scores[i] * [3., 2., 3., 2., 4., 2., 3., 2., 3.][i];
                small_scores[i] = (small_scores[i] + 1.) / 2.;
            }
        }
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
        score += line_scores[0] + line_scores[1] + line_scores[2] +line_scores[3] +
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

    fn finished(&self) -> bool {
        if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            return true;
        }
        return is_win(self.big_win) || is_win(self.big_lose);
    }
}
