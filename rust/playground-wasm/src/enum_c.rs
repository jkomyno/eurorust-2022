// Libraries needed:
// - wasm-bindgen = { features = ["serde-serialize"] }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Provider {
  Postgres,
  MySQL,
  SQLite,
}
