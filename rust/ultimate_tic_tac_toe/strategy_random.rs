use crate::ultimate_tic_tac_toe::ai::{SCORE, ZOBRIST, State, Action, SCORE_WIN, Timer};
use eprintln as log;

pub fn calc_action(state: &State, timer: &Timer, logging: bool) -> Action {
    state.random_action()
}
