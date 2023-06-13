use rand::seq::IteratorRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::log;
use crate::ultimate_tic_tac_toe::board::Turn::{Player, Ai};

#[wasm_bindgen]
pub struct Grid {
    #[wasm_bindgen(skip)]
    pub grid_small: [[Option<Turn>; 9]; 9],
    #[wasm_bindgen(skip)]
    pub grid_big: [Option<Turn>; 9],
    pub last_big: Option<usize>,
    pub is_player_turn: bool,
    pub winner: Option<Turn>,
}

#[wasm_bindgen]
impl Grid {
    pub fn initial_grid() -> Self {
        let board = Grid {
            grid_small: [[None; 9]; 9],
            grid_big: [None; 9],
            last_big: None,
            is_player_turn: true,
            winner: None
        };
        log!("init board {:?}", board.is_player_turn);
        board
    }

    pub fn play(mut self, action: &Cell) -> Self {
        log!("play! {:?},{:?}", action.b, action.s);
        let cell = if self.is_player_turn { Player } else { Ai };
        self.grid_small[action.b][action.s] = Some(cell);
        let winner = Grid::winner(&self.grid_small[action.b]);
        if winner.is_some() {
            self.grid_big[action.b] = winner;
        }
        let winner = Grid::winner(&self.grid_big);
        if winner.is_some() {
            self.winner = winner;
        }
        if self.grid_big[action.s].is_some() {
            self.last_big = None
        } else {
            self.last_big = Some(action.s);
        }
        self.is_player_turn = !self.is_player_turn;
        self
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

    pub fn ai_action(&self) -> Cell {
        let mut a = 0;
        for i in 0..10000 {
            for j in 0..100000 {
                a += i * j + 1 - j;
                if a < 0 {
                    a += 1;
                }
            }
        }
        if a == 0 {
            log!("{}", a);
        }
        let action = (0..81).map(|i| Cell { b: i%9, s: i/9 })
            .filter(|a| self.is_valid_action(a))
            .choose(&mut thread_rng());
        action.unwrap()
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
