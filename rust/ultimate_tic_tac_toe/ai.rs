use std::{fmt, io};
use std::cmp::Ordering;
use std::fmt::{Formatter};
use std::time::Duration;
use std::ops::Add;
use std::time::Instant;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

#[cfg(target_arch = "wasm32")]
pub(crate) use crate::log;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::wasm::Timer;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use eprintln as log;
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

    pub fn time(&self) -> Duration {
        self.0.elapsed()
    }
}

pub(crate) static mut SCORE: [f32; 0x40000] = [0.; 0x40000];
pub(crate) static mut ZOBRIST_O: [u64; 81] = [0; 81];
pub(crate) static mut ZOBRIST_X: [u64; 81] = [0; 81];
pub(crate) static mut ZOBRIST_RESET_O: [u64; 0x1200] = [0; 0x1200];
pub(crate) static mut ZOBRIST_RESET_X: [u64; 0x1200] = [0; 0x1200];
pub(crate) static mut ZOBRIST_BIG_O: [u64; 9] = [0; 9];
pub(crate) static mut ZOBRIST_BIG_X: [u64; 9] = [0; 9];
pub(crate) static mut ZOBRIST_BIG_DRAW: [u64; 9] = [0; 9];
static mut IS_WIN: [bool; 0x200] = [false; 0x200];
static mut POPCNT: [u16; 0x200] = [0; 0x200];

pub fn init() {
    let is_win = |table| table & 0b100100100 == 0b100100100 || table & 0b010010010 == 0b010010010 ||
        table & 0b001001001 == 0b001001001 || table & 0b111000000 == 0b111000000 ||
        table & 0b000111000 == 0b000111000 || table & 0b000000111 == 0b000000111 ||
        table & 0b100010001 == 0b100010001 || table & 0b001010100 == 0b001010100;
    unsafe {
        let mut rng = StdRng::seed_from_u64(0);
        for bs in 0..81 {
            ZOBRIST_O[bs] = rng.next_u64();
            ZOBRIST_X[bs] = rng.next_u64();
        }
        for b in 0..9 {
            ZOBRIST_BIG_O[b] = rng.next_u64();
            ZOBRIST_BIG_X[b] = rng.next_u64();
            ZOBRIST_BIG_DRAW[b] = rng.next_u64();
        }
        for o in (0..0x200).rev() {
            for b in 0..9 {
                for s in 0..9 {
                    if o >> s & 1 == 1 {
                        ZOBRIST_RESET_O[o * 9 + b] ^= ZOBRIST_O[b * 9 + s];
                        ZOBRIST_RESET_X[o * 9 + b] ^= ZOBRIST_X[b * 9 + s];
                    }
                }
            }
            POPCNT[o] = o.count_ones() as u16;
            let mut x = o ^ 0x1ff;
            loop {
                let mask = o << 9 | x;
                if is_win(o) {
                    IS_WIN[o] = true;
                    SCORE[mask] = 1.;
                } else if is_win(x) {
                    SCORE[mask] = -1.;
                } else if !is_win(o ^ 0x1ff) && !is_win(x ^ 0x1ff) {
                    SCORE[mask] = 0.;
                } else {
                    let mut valid_me = Vec::new();
                    let mut valid_op = Vec::new();
                    for i in 0..9 {
                        if (o | x) >> i & 1 == 1 {
                            continue;
                        }
                        valid_me.push(SCORE[mask | 1 << i + 9]);
                        valid_op.push(SCORE[mask | 1 << i]);
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
                if x == 0 {
                    break;
                }
                x = (x - 1) & !o;
            }
        }
    }
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    init();
    let mut state = State::new();
    let mut time_limit = Duration::from_millis(700);
    loop {
        let opponent_action = in_action();
        let timer = Timer::new(&time_limit);
        time_limit = Duration::from_millis(95);
        if let Some(action) = opponent_action {
            state = state.advanced(&action);
        }
        use crate::ultimate_tic_tac_toe::strategy_mcts::calc_action;
        let action = calc_action(&state, &timer, true);
        println!("{}", action);
        state = state.advanced(&action);
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
            b: (y / 3 * 3 + x / 3) as u16,
            s: (y % 3 * 3 + x % 3) as u16,
            anywhere: false,
            eval: 0.,
        })
    }
}

#[inline]
pub fn get_small(table: u128, b: u16) -> u16 {
    (table >> b * 9 & 0b111111111) as u16
}

#[inline]
pub fn is_win(table: u16) -> bool {
    return unsafe { IS_WIN[table as usize] };
}

#[inline]
pub fn popcount(table: u16) -> u16 {
    return unsafe { POPCNT[table as usize] };
}

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub b: u16,
    pub s: u16,
    pub anywhere: bool,
    pub eval: f32,
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
    pub last_big: u16,
    pub hash: u64,
    pub turn: bool,
}

pub const SCORE_WIN: f32 = 100.;
pub const SCORE_LOSE: f32 = -100.;

impl State {
    pub fn new() -> Self {
        State {
            small_win: 0,
            small_lose: 0,
            big_win: 0,
            big_lose: 0,
            big_draw: 0,
            last_big: 9,
            hash: 0,
            turn: false,
        }
    }

    pub fn advanced(&self, action: &Action) -> Self {
        let small_win = self.small_lose;
        let mut small_lose = self.small_win;
        let big_win = self.big_lose;
        let mut big_lose = self.big_win;
        let mut big_draw = self.big_draw;
        let hash = self.advanced_hash(action.b, action.s);
        let turn = !self.turn;
        small_lose |= 1_u128 << (action.b * 9 + action.s);
        if is_win(get_small(small_lose, action.b)) {
            big_lose |= 1 << action.b;
        } else if get_small(small_win | small_lose, action.b) == 0x1ff {
            big_draw |= 1 << action.b;
        }
        let free_big = (big_win | big_lose | big_draw) >> action.s & 1 == 1;
        let last_big = if free_big { 9 } else { action.s };
        State {
            small_win,
            small_lose,
            big_win,
            big_lose,
            big_draw,
            last_big,
            hash,
            turn
        }
    }

    pub fn advance_self(&mut self, action: &Action) {
        let small_win = self.small_lose;
        let mut small_lose = self.small_win;
        let big_win = self.big_lose;
        let mut big_lose = self.big_win;
        let mut big_draw = self.big_draw;
        let hash = self.advanced_hash(action.b, action.s);
        let turn = !self.turn;
        small_lose |= 1_u128 << (action.b * 9 + action.s);
        if is_win(get_small(small_lose, action.b)) {
            big_lose |= 1 << action.b;
        } else if get_small(small_win | small_lose, action.b) == 0x1ff {
            big_draw |= 1 << action.b;
        }
        let free_big = (big_win | big_lose | big_draw) >> action.s & 1 == 1;
        let last_big = if free_big { 9 } else { action.s };
        self.small_win = small_win;
        self.small_lose = small_lose;
        self.big_win = big_win;
        self.big_lose = big_lose;
        self.big_draw = big_draw;
        self.last_big = last_big;
        self.hash = hash;
        self.turn = turn;
    }

    pub fn advanced_hash(&self, b: u16, s: u16) -> u64 {
        unsafe {
            let small_w = get_small(self.small_win, b);
            let small_l = get_small(self.small_lose, b);
            let wb = (small_w * 9 + b) as usize;
            let lb = (small_l * 9 + b) as usize;
            self.hash ^ if is_win(small_w | 1 << s) {
                if self.turn {
                    ZOBRIST_RESET_O[wb] ^ ZOBRIST_RESET_X[lb] ^ ZOBRIST_BIG_O[b as usize]
                } else {
                    ZOBRIST_RESET_X[wb] ^ ZOBRIST_RESET_O[lb] ^ ZOBRIST_BIG_X[b as usize]
                }
            } else if small_w | small_l | 1 << s == 0x1ff {
                if self.turn {
                    ZOBRIST_RESET_O[wb] ^ ZOBRIST_RESET_X[lb] ^ ZOBRIST_BIG_DRAW[b as usize]
                } else {
                    ZOBRIST_RESET_X[wb] ^ ZOBRIST_RESET_O[lb] ^ ZOBRIST_BIG_DRAW[b as usize]
                }
            } else {
                if self.turn {
                    ZOBRIST_O[(b * 9 + s) as usize]
                } else {
                    ZOBRIST_X[(b * 9 + s) as usize]
                }
            }
        }
    }

    pub fn action_of(&self, b: u16, s: u16) -> Action {
        let win = get_small(self.small_win, b);
        let lose = get_small(self.small_lose, b);
        let anywhere = (self.big_win | self.big_lose | self.big_draw) >> s & 1 == 1 || b == s && is_win(win | 1 << s) || win | lose == 0x1ff;
        return Action {
            b,
            s,
            anywhere,
            eval: 0.,
        }
    }

    pub fn random_action(&self) -> Action {
        return if self.last_big != 9 {
            let small_win = get_small(self.small_win, self.last_big);
            let small_lose = get_small(self.small_lose, self.last_big);
            let mut remain = (small_win | small_lose) ^ 0x1ff;
            let num_actions = popcount(remain) as usize;
            let mut i = rng::next() % num_actions;
            for _ in 0..i {
                remain ^= remain & !(remain - 1);
            }
            let s = popcount((remain & !(remain - 1)) - 1);
            self.action_of(self.last_big, s)
        } else {
            let remain_big = (self.big_win | self.big_lose | self.big_draw) ^ 0x1ff;
            let remain_big_mask = (((remain_big as u128 & 0xff) * 0x101010101010101_u128 | (remain_big as u128) << 64) & 0x1008040201008040201_u128) * 0x1ff;
            let mut remain = !self.small_win & !self.small_lose & remain_big_mask;
            let num_actions = remain.count_ones() as usize;
            let mut i = rng::next() % num_actions;
            for _ in 0..i {
                remain ^= remain & !(remain - 1);
            }
            let bs = remain.trailing_zeros() as u16;
            self.action_of(bs / 9, bs % 9)
        }
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        if self.last_big != 9 {
            let small_win = get_small(self.small_win, self.last_big);
            let small_lose = get_small(self.small_lose, self.last_big);
            let mut remain = (small_win | small_lose) ^ 0x1ff;
            let num_actions = popcount(remain) as usize;
            let mut actions = Vec::with_capacity(num_actions);
            while remain > 0 {
                let bit = remain & !(remain - 1);
                let s = popcount(bit - 1);
                actions.push(self.action_of(self.last_big, s));
                remain ^= bit;
            }
            actions
        } else {
            let remain_big = (self.big_win | self.big_lose | self.big_draw) ^ 0x1ff;
            let remain_big_mask = (((remain_big as u128 & 0xff) * 0x101010101010101_u128 | (remain_big as u128) << 64) & 0x1008040201008040201_u128) * 0x1ff;
            let remain = !self.small_win & !self.small_lose & remain_big_mask;
            let num_actions = remain.count_ones() as usize;
            let mut actions = Vec::with_capacity(num_actions);
            for b in 0..9 {
                if remain_big >> b & 1 == 0 {
                    continue;
                }
                let mut remain_small = (remain >> b * 9 & 0x1ff) as u16;
                while remain_small > 0 {
                    let bit = remain_small & !(remain_small - 1);
                    let s = popcount(bit - 1);
                    actions.push(self.action_of(b, s));
                    remain_small ^= bit;
                }
            }
            actions
        }
    }

    pub fn valid_actions_with_move_ordering(&self) -> Vec<Action> {
        let mut actions = self.valid_actions();
        for mut action in &mut actions {
            let win = get_small(self.small_win, action.b);
            let lose = get_small(self.small_lose, action.b);
            action.eval = unsafe { SCORE[(win as usize) << 9 | (lose as usize) | 1 << (action.s + 9)] } + if action.anywhere { -1. } else { 0. };
        }
        actions.sort_unstable_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
        if actions.len() >= 3 {
            actions.swap(0, 1);
        }
        actions
    }

    pub fn finished(&self) -> bool {
        if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            return true;
        }
        return is_win(self.big_win) || is_win(self.big_lose);
    }

    pub fn winner(&self) -> Option<MatchResult> {
        if is_win(self.big_win) {
            Some(MatchResult::Win)
        } else if is_win(self.big_lose) {
            Some(MatchResult::Lose)
        } else if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            match popcount(self.big_win).cmp(&popcount(self.big_lose)) {
                Ordering::Greater => Some(MatchResult::Win),
                Ordering::Less => Some(MatchResult::Lose),
                Ordering::Equal => Some(MatchResult::Draw),
            }
        } else {
            None
        }
    }
}

pub fn calc_eval(state: &State) -> f32 {
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
    let mut eval = 0.;
    let mut draw_mask = 0_u16;
    let mut small_evals = [0.; 9];
    eval += unsafe { SCORE[(state.big_win as usize) << 9 | state.big_lose as usize] };
    for i in 0..9 {
        let win = get_small(state.small_win, i as u16);
        let lose = get_small(state.small_lose, i as u16);
        if !is_win(win ^ 0x1ff) && !is_win(lose ^ 0x1ff) {
            draw_mask |= 1 << i;
        } else {
            small_evals[i] = unsafe { SCORE[(win as usize) << 9 | lose as usize] };
            eval += small_evals[i] * [3., 2., 3., 2., 4., 2., 3., 2., 3.][i];
            small_evals[i] = (small_evals[i] + 1.) / 2.;
        }
    }
    #[inline]
    fn calc(a: f32, b: f32, c: f32) -> f32 {
        let p1 = a * b * c;
        let p2 = (1. - a) * (1. - b) * (1. - c);
        return if p1 > p2 { p1 } else { -p2 }
    }
    let line_evals = [
        if draw_mask & 0b000000111 == 0 { calc(small_evals[0], small_evals[1], small_evals[2]) } else { 0. },
        if draw_mask & 0b000111000 == 0 { calc(small_evals[3], small_evals[4], small_evals[5]) } else { 0. },
        if draw_mask & 0b111000000 == 0 { calc(small_evals[6], small_evals[7], small_evals[8]) } else { 0. },
        if draw_mask & 0b001001001 == 0 { calc(small_evals[0], small_evals[3], small_evals[6]) } else { 0. },
        if draw_mask & 0b010010010 == 0 { calc(small_evals[1], small_evals[4], small_evals[7]) } else { 0. },
        if draw_mask & 0b100100100 == 0 { calc(small_evals[2], small_evals[5], small_evals[8]) } else { 0. },
        if draw_mask & 0b100010001 == 0 { calc(small_evals[0], small_evals[4], small_evals[8]) } else { 0. },
        if draw_mask & 0b001010100 == 0 { calc(small_evals[2], small_evals[4], small_evals[6]) } else { 0. }
    ];
    eval += 7. * (line_evals[0] + line_evals[1] + line_evals[2] + line_evals[3] +
        line_evals[4] + line_evals[5] + line_evals[6] + line_evals[7]);
    let min = line_evals[0].min(line_evals[1]).min(line_evals[2]).min(line_evals[3])
        .min(line_evals[4]).min(line_evals[5]).min(line_evals[6]).min(line_evals[7]);
    let max = line_evals[0].max(line_evals[1]).max(line_evals[2]).max(line_evals[3])
        .max(line_evals[4]).max(line_evals[5]).max(line_evals[6]).max(line_evals[7]);
    if min < 0. {
        eval += min * 7.;
    }
    if max > 0. {
        eval += max * 7.;
    }
    eval
}

mod rng {
    pub fn next() -> usize {
        static mut SEED: u64 = 88172645463325252;
        unsafe {
            SEED ^= SEED << 7;
            SEED ^= SEED >> 9;
            SEED as usize
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut grid = [['.'; 11]; 11];
        for i in 0..11 {
            grid[3][i] = '-';
            grid[7][i] = '-';
            grid[i][3] = '|';
            grid[i][7] = '|';
        }
        grid[3][3] = '+';
        grid[3][7] = '+';
        grid[7][3] = '+';
        grid[7][7] = '+';
        for y in 0..9 {
            for x in 0..9 {
                let b = y / 3 * 3 + x / 3;
                let s = y % 3 * 3 + x % 3;
                if (self.big_lose >> b & 1) == 1 {
                    grid[y+y/3][x+x/3] = if s == 4 {
                        if self.turn { 'O' } else { 'X' }
                    } else {
                        '*'
                    }
                } else if (self.big_win >> b & 1) == 1 {
                    grid[y+y/3][x+x/3] = if s == 4 {
                        if self.turn { 'X' } else { 'O' }
                    } else {
                        '*'
                    }
                } else if (self.small_lose >> (b * 9 + s) & 1) == 1 {
                    grid[y+y/3][x+x/3] = if self.turn { 'O' } else { 'X' }
                } else if (self.small_win >> (b * 9 + s) & 1) == 1 {
                    grid[y+y/3][x+x/3] = if self.turn { 'X' } else { 'O' }
                }
            }
        }
        for row in grid {
            for ch in row {
                if let Err(e) = write!(f, "{}", ch) {
                    return Err(e);
                }
            }
            if let Err(e) = writeln!(f) {
                return Err(e);
            }
        }
        Ok(())
    }
}

pub enum MatchResult {
    Win, Lose, Draw
}
