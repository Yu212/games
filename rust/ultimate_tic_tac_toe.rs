pub mod ai;
#[cfg(target_arch = "wasm32")]
mod game;
pub mod strategy_alpha_beta;
pub mod strategy_mcts;
pub mod strategy_random;
pub mod strategy_alpha_beta_2;
pub mod strategy_negascout;
pub mod strategy_alpha_beta_debug;
pub mod strategy_alpha_beta_transpose;
