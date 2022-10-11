// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RefT<'a> {
  pub Description: &'a str,
}
