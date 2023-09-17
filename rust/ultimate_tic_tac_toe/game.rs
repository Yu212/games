use std::ops::Add;
use std::time::{Duration, Instant};
use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Float32Array;
use crate::log;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta::{calc_action, calc_deep_eval};
use crate::ultimate_tic_tac_toe::ai::*;
use crate::wasm::{log, Timer};
use crate::ultimate_tic_tac_toe::game::Turn::{Player, Ai};

#[wasm_bindgen]
pub unsafe fn init_ai() {
    log!("init ai");
    init();
}

#[wasm_bindgen]
pub struct Grid {
    #[wasm_bindgen(skip)]
    pub grid_small: [[Option<Turn>; 9]; 9],
    #[wasm_bindgen(skip)]
    pub grid_big: [Option<Turn>; 9],
    pub last_big: Option<usize>,
    pub is_player_turn: bool,
    pub is_first_player: bool,
    pub winner: Option<Turn>,
}

#[wasm_bindgen]
impl Grid {
    pub fn initial_grid(first_turn: Turn) -> Self {
        log!("initi: {}", first_turn == Player);
        Grid {
            grid_small: [[None; 9]; 9],
            grid_big: [None; 9],
            last_big: None,
            is_player_turn: first_turn == Player,
            is_first_player: first_turn == Player,
            winner: None,
        }
    }

    pub fn advance(&self, action: &Cell) -> Self {
        log!("move! {:?},{:?},{}", action.b, action.s, self.is_player_turn);
        let mut grid_small = self.grid_small.clone();
        let mut grid_big = self.grid_big.clone();
        let cell = if self.is_player_turn { Player } else { Ai };
        grid_small[action.b][action.s] = Some(cell);
        grid_big[action.b] = Grid::winner(&grid_small[action.b]);
        let winner = Grid::winner(&grid_big);
        let last_big = if grid_big[action.s].is_some() || grid_small[action.s].iter().all(|c| c.is_some()) {
            None
        } else {
            Some(action.s)
        };
        Grid {
            grid_small,
            grid_big,
            last_big,
            is_player_turn: !self.is_player_turn,
            is_first_player: self.is_first_player,
            winner,
        }
    }

    pub fn is_valid_action(&self, a: &Cell) -> bool {
        if !self.last_big.is_none() && self.last_big != Some(a.b) {
            return false
        }
        self.grid_big[a.b].is_none() && self.grid_small[a.b][a.s].is_none()
    }

    pub fn get_big_cell(&self, b: usize) -> Option<Turn> {
        self.grid_big[b].clone()
    }

    pub fn get_small_cell(&self, b: usize, s: usize) -> Option<Turn> {
        self.grid_small[b][s].clone()
    }

    pub fn calc_all_evals(&self) -> Float32Array {
        let state = self.to_state();
        let timer = Timer::new(&Duration::from_secs(1));
        for depth in 3.. {
            let mut evals = [0.; 81];
            for action in state.valid_actions_with_move_ordering() {
                let state = state.advanced(&action);
                evals[(action.b * 9 + action.s) as usize] = calc_deep_eval(&state, depth);
            }
            if timer.elapsed() {
                return Float32Array::from(evals.as_ref());
            }
        }
        unreachable!();
    }

    pub fn ai_action(&self) -> Cell {
        let state = self.to_state();
        let timer = Timer::new(&Duration::from_millis(1000));
        let action = calc_action(&state, &timer, false);
        Cell { b: action.b as usize, s: action.s as usize }
    }

    fn to_state(&self) -> State {
        let mut small_win = 0;
        let mut small_lose = 0;
        let mut big_win = 0;
        let mut big_lose = 0;
        let mut big_draw = 0;
        for b in 0..9 {
            for s in 0..9 {
                match self.grid_small[b][s] {
                    Some(Ai) => {
                        small_win |= 1_u128 << (b * 9 + s);
                    }
                    Some(Player) => {
                        small_lose |= 1_u128 << (b * 9 + s);
                    }
                    None => {}
                }
            }
            if self.grid_small[b].iter().all(|c| c.is_some()) {
                big_draw |= 1 << b;
            }
            match self.grid_big[b] {
                Some(Ai) => {
                    big_win |= 1 << b;
                }
                Some(Player) => {
                    big_lose |= 1 << b;
                }
                None => {}
            }
        }
        let last_big = if self.last_big.is_some() { self.last_big.unwrap() as u16 } else { 9 };
        let mut state = if self.is_player_turn {
            State {
                small_win: small_lose,
                small_lose: small_win,
                big_win: big_lose,
                big_lose: big_win,
                big_draw,
                last_big,
                hash: 0,
                turn: !self.is_first_player,
            }
        } else {
            State {
                small_win,
                small_lose,
                big_win,
                big_lose,
                big_draw,
                last_big,
                hash: 0,
                turn: self.is_first_player,
            }
        };
        state.hash = hashing(&state);
        state
    }

    fn winner(grid: &[Option<Turn>; 9]) -> Option<Turn> {
        for line in [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]] {
            let [a, b, c] = [grid[line[0]], grid[line[1]], grid[line[2]]];
            if a.is_some() && a == b && b == c {
                return a;
            }
        }
        None
    }
}

pub fn hashing(state: &State) -> u64 {
    let mut hash = 0;
    let (zobrist_win, zobrist_lose, zobrist_big_win, zobrist_big_lose, zobrist_big_draw) = unsafe {
        if state.turn {
            (ZOBRIST_O, ZOBRIST_X, ZOBRIST_BIG_O, ZOBRIST_BIG_X, ZOBRIST_BIG_DRAW)
        } else {
            (ZOBRIST_X, ZOBRIST_O, ZOBRIST_BIG_X, ZOBRIST_BIG_O, ZOBRIST_BIG_DRAW)
        }
    };
    for b in 0..9 {
        if state.big_win >> b & 1 == 1 {
            hash ^= zobrist_big_win[b];
        } else if state.big_lose >> b & 1 == 1 {
            hash ^= zobrist_big_lose[b];
        } else if state.big_draw >> b & 1 == 1 {
            hash ^= zobrist_big_draw[b];
        } else {
            for s in 0..9 {
                if state.small_win >> (b * 9 + s) & 1 == 1 {
                    hash ^= zobrist_win[b * 9 + s];
                } else if state.small_lose >> (b * 9 + s) & 1 == 1 {
                    hash ^= zobrist_lose[b * 9 + s];
                }
            }
        }
    }
    hash
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Cell {
    pub b: usize,
    pub s: usize,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(constructor)]
    pub fn new(b: usize, s: usize) -> Self {
        Cell { b, s }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    Player, Ai
}
