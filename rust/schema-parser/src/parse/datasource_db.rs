use crate::ast::datasource_db::{DatasourceDb, Provider, Url};
use crate::parseutil::{parse_string_quoted, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::delimited;
use nom::IResult;
use nom::{
  branch::permutation,
  character::complete::{line_ending, space0},
  combinator::{opt, value},
  sequence::tuple,
};

use super::parser::{parse_alt_enum, Parser};
use crate::parseutil::ws_inline;

impl Parser for Provider {
  /// Parse a datasource db provider, e.g., "postgres".
  /// It expects no whitespace prefix.
  fn parse(input: &str) -> IResult<&str, Self> {
    let delimiter = '"';
    delimited(
      char(delimiter),
      alt((
        parse_alt_enum!("postgres"),
        parse_alt_enum!("cockroachdb"),
        parse_alt_enum!("mysql"),
        parse_alt_enum!("mariadb"),
        parse_alt_enum!("sqlserver"),
        parse_alt_enum!("sqlite"),
        parse_alt_enum!("mongodb"),
      )),
      char(delimiter),
    )(input)
  }
}

impl Url {
  fn parse_static(input: &str) -> IResult<&str, Self> {
    parse_string_quoted('"')(input).map(|(rest, static_url)| (rest, Self::Static(static_url)))
  }

  fn parse_env(input: &str) -> IResult<&str, Self> {
    delimited(tag("env("), parse_string_quoted('"'), char(')'))(input)
      .map(|(rest, env_url)| (rest, Self::Env(env_url)))
  }
}

impl Parser for Url {
  /// Parse a datasource db URL, e.g.:
  /// - "postgres://user:pass@host:port/dbname" (Static)
  /// - env("POSTGRES_URL") (Env)
  /// It expects no whitespace prefix.
  fn parse(input: &str) -> IResult<&str, Self> {
    alt((Self::parse_static, Self::parse_env))(input)
  }
}

/// Parsers an attribute definition made of "$attr = $value",
/// returning the value.
/// It consumes whitespace around it, but leaves any newline.
macro_rules! parse_with_attribute_correct {
  // $t : Parser
  ($attr:expr,$t:ty) => {
    delimited(
      tuple((space0, tag($attr), ws_inline(char('=')))),
      <$t>::parse,
      tuple((space0, line_ending)),
    )
  };
}

impl DatasourceDb {
  // Parses datasource db's attributes, e.g.,
  // provider = "postgres"
  fn parse_attributes(input: &str) -> IResult<&str, Self> {
    permutation((
      ws(parse_with_attribute_correct!("provider", Provider)),
      ws(parse_with_attribute_correct!("url", Url)),
      ws(opt(parse_with_attribute_correct!("shadowDatabaseUrl", Url))),
    ))(input)
    .map(|(rest, (provider, url, shadow_database_url))| {
      (rest, Self { provider, url, shadow_database_url })
    })
  }
}

impl Parser for DatasourceDb {
  /// Parse a datasource db block, e.g.,
  /// db {
  ///   provider = "postgres"
  ///   url      = "postgres://user:pass@host:port/dbname"
  /// }
  fn parse(input: &str) -> IResult<&str, Self> {
    delimited(tuple((ws(tag(r#"db"#)), ws(tag("{")))), Self::parse_attributes, ws(tag("}")))(input)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! test_provider {
    ($s:expr) => {
      paste::item! {
        #[test]
        fn [< test_provider_ $s >] () {
          let schema = format!(r#""{}""#, &$s);
          let (remaining_input, provider) = Provider::parse(&schema).unwrap();
          assert!(remaining_input.is_empty());
          assert_eq!(provider, $s.into());
        }
      }
    };
  }

  macro_rules! test_url_env {
    ($s:expr,$t:ty) => {
      paste::item! {
        #[test]
        fn [< test_url_env_ $s >] () {
          let schema = r#"env("DATABASE_URL")"#;
          let (remaining_input, url) = $t::parse(schema).unwrap();
          assert!(remaining_input.is_empty());
          assert_eq!(url, $t::Env(String::from("DATABASE_URL")));
        }
      }
    };
  }

  macro_rules! test_url_static {
    ($s:expr,$t:ty) => {
      paste::item! {
        #[test]
        fn [< test_url_static_ $s >] () {
          let schema = r#""postgres://user:pass@host:port/dbname""#;
          let (remaining_input, url) = $t::parse(schema).unwrap();
          assert!(remaining_input.is_empty());
          assert_eq!(url, $t::Static  (String::from("postgres://user:pass@host:port/dbname")));
        }
      }
    };
  }

  test_provider!("postgres");
  test_provider!("cockroachdb");
  test_provider!("mysql");
  test_provider!("mariadb");
  test_provider!("sqlserver");
  test_provider!("sqlite");
  test_provider!("mongodb");

  test_url_env!("url", Url);
  test_url_env!("shadow_database_url", Url);

  test_url_static!("url", Url);
  test_url_static!("shadow_database_url", Url);

  #[test]
  fn test_datasource_db() {
    /* formatted */
    let schema = r#"
      db {
        provider = "postgres"

        url = env("DATABASE_URL")
      }
    "#;

    let (remaining_input, db) = DatasourceDb::parse(schema).unwrap();
    println!("remaining_input: {:?}", remaining_input);
    assert!(remaining_input.is_empty());

    assert_eq!(
      db,
      DatasourceDb {
        provider: Provider::Postgres,
        url: Url::Env(String::from("DATABASE_URL")),
        shadow_database_url: None,
      }
    );

    /* compact spacing */
    let schema = r#"db{provider = "postgres"
url = env("DATABASE_URL")
}"#;

    let (remaining_input, db) = DatasourceDb::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(
      db,
      DatasourceDb {
        provider: Provider::Postgres,
        url: Url::Env(String::from("DATABASE_URL")),
        shadow_database_url: None,
      }
    );

    /* with shadowDatabaseUrl */
    let schema = r#"
      db {
        provider = "postgres"
        url = env("DATABASE_URL")
        shadowDatabaseUrl = env("SHADOW_DATABASE_URL")
      }
    "#;

    let (remaining_input, db) = DatasourceDb::parse(schema).unwrap();
    assert!(remaining_input.is_empty());

    assert_eq!(
      db,
      DatasourceDb {
        provider: Provider::Postgres,
        url: Url::Env(String::from("DATABASE_URL")),
        shadow_database_url: Some(Url::Env(String::from("SHADOW_DATABASE_URL"))),
      }
    );
  }
}
