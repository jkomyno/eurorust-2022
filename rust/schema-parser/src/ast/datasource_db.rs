use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// Clone is needed for parser macros in schema_parser::parser::parser.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
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

impl fmt::Display for Provider {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let provider_as_str = serde_json::to_string(self).unwrap();
    write!(f, "{}", provider_as_str)
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
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
