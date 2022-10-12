use validate::{diagnostics, validator};

use crate::ast::datasource::Datasource;
use crate::ast::datasource_db::{DatasourceDb, Provider, Url};
use crate::ast::schema::SchemaAST;
use crate::parse::parser::Parser;

pub mod ast;
pub mod parse;
mod parseutil;
pub mod validate;

// Given a schema file input, parse it and return a SchemaAST, or a parser error.
pub fn parse_schema(input: String) -> Result<SchemaAST, String> {
  let (_, schema_ast) = SchemaAST::parse(&input).map_err(|e| format!("{:?}", e))?;
  Ok(schema_ast)
}

// Given a parsed schema AST, validate it and return a potential list of validation errors.
// This function may panic in the following cases:
// - a datasource is defined with an environment variable URL
pub fn validate_ast(ast: &SchemaAST) -> Result<(), diagnostics::Diagnostics> {
  let mut diagnostics = diagnostics::Diagnostics::default();
  let supported_providers: Vec<Provider> = vec![Provider::SQLite, Provider::Postgres];
  validator::validate_configuration(&ast, &supported_providers, &mut diagnostics);

  diagnostics.to_result()
}

// Return an example schema AST to showcase serialization via Tsify in Wasm.
pub fn example_schema() -> SchemaAST {
  SchemaAST {
    datasources: vec![Datasource::Db(DatasourceDb {
      provider: Provider::Postgres,
      url: Url::Env(String::from("DATABASE_URL")),
      shadow_database_url: None,
    })],
  }
}
