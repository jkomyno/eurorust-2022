/* datasource db */

export type Provider = "postgres" | "cockroachdb" | "mysql" | "mariadb" | "sqlserver" | "sqlite" | "mongodb";

export type Url
  = { _tag: "static", value: string }
  | { _tag: "env", value: string }

export interface DatasourceDb {
  provider: Provider
  url: Url
  shadowDatabaseUrl: Url | null
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

export interface Diagnostics {
  errors: DatamodelError[];
}
