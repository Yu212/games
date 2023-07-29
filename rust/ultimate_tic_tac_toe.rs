pub mod ai;
#[cfg(target_arch = "wasm32")]
mod game;
pub(crate) mod strategy_alpha_beta;
pub(crate) mod strategy_mcts;
pub(crate) mod strategy_random;
pub(crate) mod strategy_alpha_beta_2;
pub(crate) mod strategy_negascout;
pub(crate) mod strategy_alpha_beta_debug;
pub(crate) mod strategy_alpha_beta_transpose;
