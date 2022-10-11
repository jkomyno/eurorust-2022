use ast::{MiniSchema, Provider, Url};
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub mod ast;

pub fn example_schema() -> MiniSchema {
  MiniSchema {
    providers: vec![Provider::SQLite, Provider::Postgres],
    shadow_database_url: Some(Url::Env(String::from("DATABASE_URL"))),
    id: None,
  }
}

// We can't return a `MiniSchema` directly, because the trait
// `ReturnWasmAbi` is not implemented for it. Hence we return
// a `JsValue` instead.
#[wasm_bindgen(js_name = exampleSchema)]
pub fn example_schema_wasm() -> Result<JsValue, String> {
  let ast = example_schema();
  let ast_as_js = to_js_value(&ast).unwrap();
  Ok(ast_as_js)
}

pub fn to_js_value<T>(value: &T) -> Result<wasm_bindgen::JsValue, serde_wasm_bindgen::Error>
where
  T: Serialize + ?Sized,
{
  serde_wasm_bindgen::to_value(&value)
}
