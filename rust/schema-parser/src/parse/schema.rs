use crate::{
  ast::{
    datasource::Datasource,
    schema::{SchemaAST, SchemaASTBlock},
  },
  parseutil::ws,
};
use nom::branch::alt;
use nom::IResult;
use nom::{combinator::map, multi::many0};

use super::parser::Parser;

impl SchemaASTBlock {
  /// Parses a schema block, e.g.,
  /// r#"datasource db {
  ///   provider = "postgres"
  ///   url      = "postgres://user:pass@host:port/dbname"
  /// }"#.
  fn parse(input: &str) -> IResult<&str, Self> {
    alt((map(Datasource::parse, SchemaASTBlock::Datasource),))(input)
  }
}

impl Parser for SchemaAST {
  /// Parses a schema, e.g.,
  /// r#"
  /// db {
  ///   provider = "postgres"
  ///   url      = "postgres://user:pass@host:port/dbname"
  /// }
  ///
  /// model User {
  ///   id   String @id @map("user_id")
  ///   name String @map("user_name")
  /// }
  /// "#.
  fn parse(input: &str) -> IResult<&str, Self> {
    ws(many0(SchemaASTBlock::parse))(input).map(|(rest, schema_ast_blocks)| {
      let mut datasources = vec![];
      for schema_ast_block in schema_ast_blocks {
        match schema_ast_block {
          SchemaASTBlock::Datasource(datasource) => datasources.push(datasource),
        }
      }
      (rest, SchemaAST { datasources })
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::ast::datasource_db::{DatasourceDb, Provider, Url};

  #[test]
  fn parse_empty_schema() {
    let schema = r#"

    "#;

    let (remaining_input, schema) = SchemaAST::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(schema, SchemaAST { datasources: vec![] });
  }

  #[test]
  fn parse_datasource_only_schema() {
    let schema = r#"
      datasource db {
        provider = "postgres"
        url = env("DATABASE_URL")
      }
    "#;

    let (remaining_input, schema) = SchemaAST::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(
      schema,
      SchemaAST {
        datasources: vec!(Datasource::Db(DatasourceDb {
          provider: Provider::Postgres,
          url: Url::Env(String::from("DATABASE_URL"),),
          shadow_database_url: None,
        },),),
      }
    );
  }

  #[test]
  fn parse_schema() {
    let schema = r#"
      datasource db {
        provider = "postgres"
        
        url = env("DATABASE_URL")
      }
    "#;

    let (remaining_input, schema) = SchemaAST::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(
      schema,
      SchemaAST {
        datasources: vec!(Datasource::Db(DatasourceDb {
          provider: Provider::Postgres,
          url: Url::Env(String::from("DATABASE_URL"),),
          shadow_database_url: None,
        },),),
      }
    );
  }
}
