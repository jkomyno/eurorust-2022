// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct VecIntParams {
  pub id: Vec<u32>,
}
