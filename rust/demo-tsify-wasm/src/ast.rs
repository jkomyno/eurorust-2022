use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// Showcase a few features of tsify that can be serialized
// whilst preserving the type annotations.

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "lowercase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Provider {
  SQLite,
  Postgres,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Url {
  Static(String),
  Env(String),
}

/// This gets serialized as e.g.:
///
/// ```json
/// {
///   "providers": ["sqlite", "postgres"],
///   "shadowDatabaseUrl": {
///     "env": "DATABASE_URL"
///   },
///   "id": null
/// }
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MiniSchema {
  pub providers: Vec<Provider>,
  pub shadow_database_url: Option<Url>,
  pub id: Option<u32>,
}
