#![allow(clippy::derive_partial_eq_without_eq)]
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Clone is needed for parser macros in schema_parser::parser::parser.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum Provider {
  Postgres,
  CockroachDb,
  MySQL,
  MariaDb,
  SQLServer,
  SQLite,
  MongoDb,
}

impl From<&str> for Provider {
  fn from(provider: &str) -> Self {
    match provider {
      "postgres" => Self::Postgres,
      "cockroachdb" => Self::CockroachDb,
      "mysql" => Self::MySQL,
      "mariadb" => Self::MariaDb,
      "sqlserver" => Self::SQLServer,
      "sqlite" => Self::SQLite,
      "mongodb" => Self::MongoDb,
      _ => panic!("Unknown provider: {}", provider),
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum Url {
  Static(String),
  Env(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct DatasourceDb {
  pub provider: Provider,
  pub url: Url,
  pub shadow_database_url: Option<Url>,
}
