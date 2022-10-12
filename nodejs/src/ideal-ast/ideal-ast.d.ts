/* utils */

type Option<T>
  = { _tag: 'some', value: T }
  | { _tag: 'none' }

/* datasource db */

export type Provider = "postgres" | "cockroachdb" | "mysql" | "mariadb" | "sqlserver" | "sqlite" | "mongodb";

export type Url
  = { _tag: 'static', value: string } // in Rust => Static(String)
  | { _tag: 'env', value: string }    // in Rust => Env(String)

export type DatasourceDb = {
  provider: Provider
  url: Url
  shadowDatabaseUrl: Option<Url>
}

/* datasource */

export type Datasource
  = { _tag: 'db', value: DatasourceDb }

/* schema */

type SchemaAST = {
  datasources: Datasource[]
}

/* functions */

export function exampleSchema(): SchemaAST;
export function parseSchema(input: string): SchemaAST;
export function validateAST(ast: SchemaAST): void;

/* error */

type DatamodelError = string

export type Diagnostics = {
  errors: DatamodelError[]
}
