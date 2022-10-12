use ast::{MiniSchema, Provider, UrlTagContent};
use wasm_bindgen::prelude::*;

pub mod ast;

pub fn example_schema() -> MiniSchema {
  MiniSchema {
    providers: vec![Provider::SQLite, Provider::Postgres],
    shadow_database_url: Some(UrlTagContent::Env(String::from("DATABASE_URL"))),
    id: None,
  }
}

// We can return a `MiniSchema` directly, thanks to `tsify`'s magic
// which also allows to type `MiniSchema` in TypeScript.
#[wasm_bindgen(js_name = exampleSchema)]
pub fn example_schema_wasm() -> Result<MiniSchema, String> {
  let ast = example_schema();
  Ok(ast)
}
