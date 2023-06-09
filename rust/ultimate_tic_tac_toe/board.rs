use wasm_bindgen::prelude::wasm_bindgen;
use crate::log;
use crate::ultimate_tic_tac_toe::board::Cell::{Empty};

#[wasm_bindgen]
pub struct Grid {
    #[wasm_bindgen(skip)]
    pub grid: [[Cell; 9]; 9],
    pub is_player_turn: bool,
}

#[wasm_bindgen]
impl Grid {
    pub fn initial_grid() -> Self {
        let board = Grid { is_player_turn: true, grid: [[Empty; 9]; 9] };
        log!("init {:?}", board.is_player_turn);
        board
    }

    pub fn play(mut self, action: &Action) -> Self {
        log!("play! {:?},{:?}", action.x, action.y);
        self.is_player_turn = !self.is_player_turn;
        self
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.grid[x][y].clone()
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
        let action = Action { x: 123, y: 456 };
        log!("{}, {:?}", a, action);
        action
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Action {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Action {
    pub fn action(x: usize, y: usize) -> Self {
        let action = Action { x, y };
        action
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Player, Ai, Empty
}
