use crate::ast::datasource::Datasource;
use crate::ast::datasource_db::{DatasourceDb, Provider, Url};
use crate::ast::schema::SchemaAST;
use crate::parser::parser::Parser;

pub mod ast;
pub mod parser;
mod parseutil;

// Given a schema file input, parse it and return a SchemaAST, or a parser error.
pub fn parse_schema(input: String) -> Result<SchemaAST, String> {
  let (_, schema_ast) = SchemaAST::parse(&input).map_err(|e| format!("{:?}", e))?;
  Ok(schema_ast)
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
