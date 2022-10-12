use std::collections::BTreeMap;

use crate::ast::{
  datasource::Datasource,
  datasource_db::{Provider, Url},
  schema::SchemaAST,
};

use super::diagnostics::Diagnostics;

pub fn validate_configuration(
  ast: &SchemaAST,
  supported_providers: &Vec<Provider>,
  diagnostics: &mut Diagnostics,
) {
  for datasource in &ast.datasources {
    match datasource {
      Datasource::Db(datasource) => {
        if !supported_providers.contains(&datasource.provider) {
          let supported_providers_as_str = supported_providers
            .iter()
            .map(|provider| format!("'{}'", provider))
            .collect::<Vec<String>>()
            .join(", ");
          diagnostics.push_error(format!(
            "The provider {} is not yet supported. Supported providers are: {}",
            datasource.provider, supported_providers_as_str
          ));
        }

        if let Err(err) = validate_url(&datasource.provider, &datasource.url) {
          diagnostics.push_error(err);
        }

        if let Some(url) = &datasource.shadow_database_url {
          if let Err(err) = validate_url(&datasource.provider, &url) {
            diagnostics.push_error(err);
          }
        }
      }
    }
  }

  if ast.datasources.len() > 1 {
    diagnostics
      .push_error("You defined more than one datasource. This is not supported yet.".into());
  }
}

fn validate_url(provider: &Provider, url: &Url) -> Result<(), String> {
  match url {
    Url::Env(_) => panic!("Environment variables are not yet supported for database URLs."),
    Url::Static(url) => {
      let provider_to_protocol = BTreeMap::from([
        (Provider::Postgres, "postgres://"),
        (Provider::CockroachDb, "postgres://"),
        (Provider::MySQL, "mysql://"),
        (Provider::MariaDb, "mysql://"),
        (Provider::SQLServer, "sqlserver://"),
        (Provider::SQLite, "file:"),
        (Provider::MongoDb, "mongo://"),
      ]);

      let expected_protocol = provider_to_protocol.get(&provider).expect("Unknown provider");
      if !url.starts_with(expected_protocol) {
        Err(format!("{provider} URLs must start with {expected_protocol}, received {url}"))
      } else {
        Ok(())
      }
    }
  }
}
