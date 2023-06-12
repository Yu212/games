use wasm_bindgen::prelude::wasm_bindgen;
use crate::log;
use crate::ultimate_tic_tac_toe::board::Cell::{Player, Ai, Empty};

#[wasm_bindgen]
pub struct Grid {
    #[wasm_bindgen(skip)]
    pub grid_small: [[Cell; 9]; 9],
    #[wasm_bindgen(skip)]
    pub grid_big: [Cell; 9],
    pub last_big: Option<usize>,
    pub is_player_turn: bool,
}

#[wasm_bindgen]
impl Grid {
    pub fn initial_grid() -> Self {
        let board = Grid {
            grid_small: [[Empty; 9]; 9],
            grid_big: [Empty; 9],
            last_big: None,
            is_player_turn: true
        };
        log!("init board {:?}", board.is_player_turn);
        board
    }

    pub fn play(mut self, action: &Action) -> Self {
        log!("play! {:?},{:?}", action.b, action.s);
        let cell = if self.is_player_turn { Player } else { Ai };
        self.grid_small[action.b][action.s] = cell;
        let winner = Grid::winner(&self.grid_small[action.b]);
        if winner != Empty {
            self.grid_big[action.b] = winner;
        }
        if self.grid_big[action.s] != Empty {
            self.last_big = None
        } else {
            self.last_big = Some(action.s);
        }
        // self.is_player_turn = !self.is_player_turn;
        self
    }

    pub fn is_valid_action(&self, b: usize, s: usize) -> bool {
        if !self.last_big.is_none() && self.last_big != Some(b) {
            return false
        }
        self.grid_big[b] == Empty && self.grid_small[b][s] == Empty
    }

    pub fn get_big_cell(&self, b: usize) -> Cell {
        self.grid_big[b].clone()
    }

    pub fn get_small_cell(&self, b: usize, s: usize) -> Cell {
        self.grid_small[b][s].clone()
    }

    pub fn ai_action(&self) -> Action {
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
        let action = Action { b: 1, s: 1 };
        action
    }

    fn winner(grid: &[Cell; 9]) -> Cell {
        for line in [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8], [0, 4, 8], [2, 4, 6]] {
            let [a, b, c] = [grid[line[0]], grid[line[1]], grid[line[2]]];
            if a != Empty && a == b && b == c {
                return a;
            }
        }
        Empty
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Action {
    pub b: usize,
    pub s: usize,
}

#[wasm_bindgen]
impl Action {
    pub fn action(b: usize, s: usize) -> Self {
        let action = Action { b, s };
        action
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Player, Ai, Empty
}
