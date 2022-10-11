use serde::Serialize;

// Showcase a few features of serde that serde-wasm-bindgen can
// serialize to json
// All type annotations get lost in the process.

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
  SQLite,
  Postgres,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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
///   }
/// }
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniSchema {
  pub providers: Vec<Provider>,
  pub shadow_database_url: Option<Url>,
  pub id: Option<u32>,
}
