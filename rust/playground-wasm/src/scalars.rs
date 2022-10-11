// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ScalarParams {
  pub Toggle: bool,
  pub Id: u32,
  pub Letter: char,
}
