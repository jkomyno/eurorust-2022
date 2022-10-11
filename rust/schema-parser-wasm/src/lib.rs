use schema_parser::ast::schema::SchemaAST;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = exampleSchema)]
pub fn example_schema_wasm() -> Result<SchemaAST, JsError> {
  let schema = schema_parser::example_schema();
  Ok(schema)
}

#[wasm_bindgen(js_name = parseSchema)]
pub fn parse_schema_wasm(input: String) -> Result<SchemaAST, JsError> {
  schema_parser::parse_schema(input).map_err(|err| to_js_error(&err))
}

fn to_js_error<T>(err: &T) -> JsError
where
  T: Serialize + ?Sized,
{
  // Serialization can panic if T's implementation of Serialize decides to fail,
  // or if T contains a map with non-string keys. (That's not our case here.)
  JsError::new(&serde_json::to_string(&err).unwrap())
}
