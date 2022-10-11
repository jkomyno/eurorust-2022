#![allow(clippy::match_single_binding)]
use crate::ast::{datasource::Datasource, datasource_db::DatasourceDb};
use crate::parseutil::ws;
use nom::branch::alt;
use nom::IResult;
use nom::{bytes::complete::tag, sequence::preceded};

use super::parser::Parser;

impl Parser for Datasource {
  /// Parses a datasource block, e.g.,
  /// r#"db {
  ///   provider = "postgres"
  ///   url      = "postgres://user:pass@host:port/dbname"
  /// }"#.
  fn parse(input: &str) -> IResult<&str, Self> {
    preceded(ws(tag(r#"datasource"#)), alt((DatasourceDb::parse,)))(input).map(
      |(rest, datasource_block)| {
        let block = match datasource_block {
          block @ DatasourceDb { .. } => Self::Db(block),
        };
        (rest, block)
      },
    )
  }
}

#[cfg(test)]
mod test {
  use crate::ast::datasource_db::{DatasourceDb, Provider, Url};

  use super::*;

  #[test]
  fn parse_datasource_db() {
    /* formatted */
    let schema = r#"
      datasource db {
        provider = "postgres"
        url = env("DATABASE_URL")
      }
    "#;

    let (remaining_input, db) = Datasource::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(
      db,
      Datasource::Db(DatasourceDb {
        provider: Provider::Postgres,
        url: Url::Env(String::from("DATABASE_URL"),),
        shadow_database_url: None,
      },)
    )
  }
}
