use crate::ast::datasource::Datasource;
use crate::ast::datasource_db::{DatasourceDb, Provider, Url};
use crate::ast::schema::SchemaAST;

pub mod ast;

pub fn example_schema() -> SchemaAST {
  SchemaAST {
    datasources: vec![Datasource::Db(DatasourceDb {
      provider: Provider::Postgres,
      url: Url::Env(String::from("DATABASE_URL")),
      shadow_database_url: None,
    })],
  }
}
