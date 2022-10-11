// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StringParams {
  id: String, // private!
}

#[wasm_bindgen]
impl StringParams {
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> String {
    self.id.clone()
  }

  #[wasm_bindgen(setter)]
  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }
}
