use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Foo {
  pub id: u32,
}

#[wasm_bindgen]
pub struct VecStructParams {
  pub id: Vec<Foo>,
}
