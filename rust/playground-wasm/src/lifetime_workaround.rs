// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

struct RefT<'a> {
  pub description: &'a str,
}

#[wasm_bindgen]
pub fn lifetime_f1() -> Result<(), &str> {
  Ok(())
}

#[wasm_bindgen]
pub fn lifetime_f2() -> Result<(), &'static str> {
  Ok(())
}

#[wasm_bindgen]
pub struct RefTWrap {
  pub value: RefT<'static>,
}
