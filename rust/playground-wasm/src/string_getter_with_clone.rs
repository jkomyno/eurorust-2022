// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Serialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct StringParams {
  pub id: String,
}
