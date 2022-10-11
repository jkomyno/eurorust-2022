use wasm_bindgen::prelude::*;

// requires manual implementation of wasm_bindgen::JsCast
pub struct Foo {
  pub id: u32,
}

#[wasm_bindgen]
pub struct VecStructParams {
  pub id: Vec<Foo>,
}
