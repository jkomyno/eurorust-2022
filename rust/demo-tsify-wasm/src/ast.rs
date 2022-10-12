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

/// This gets typed as e.g.:
///
/// ```typescript
/// { static: string } |
/// { env: string }
/// ```
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Url {
  Static(String),
  Env(String),
}

/// This gets typed as:
///
/// ```typescript
/// ({ kind: 'static') & string) |
/// ({ kind: 'env') & string)
/// ```
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum UrlTag {
  Static(String),
  Env(String),
}

/// This gets typed as:
///
/// ```typescript
/// { kind: 'static', value: string } |
/// { kind: 'env', value: string }
/// ```
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum UrlTagContent {
  Static(String),
  Env(String),
}

/// This gets typed as:
///
/// ```typescript
/// {
///   provider: 'sqlite' | 'postgres',
///   shadow_database_url: { kind: 'static', value: string }
///                      | { kind: 'env', value: string }
///                      | null,
///   id: number | null,
/// }
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MiniSchema {
  pub providers: Vec<Provider>,
  pub shadow_database_url: Option<UrlTagContent>,
  pub id: Option<u32>,
}
