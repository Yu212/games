use crate::ultimate_tic_tac_toe::ai::*;
use eprintln as log;

pub fn calc_action(state: &State, timer: &Timer, logging: bool) -> Action {
    state.random_action()
}
