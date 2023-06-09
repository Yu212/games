use wasm_bindgen::prelude::*;
use crate::log;

#[wasm_bindgen]
pub fn greet() {
    log!("Hello");
}
