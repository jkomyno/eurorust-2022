// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ScalarParams {
  pub toggle: bool,
  pub id: u32,
  pub letter: char,
}
