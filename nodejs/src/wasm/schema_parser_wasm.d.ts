/* tslint:disable */
/* eslint-disable */
/**
* @returns {SchemaAST}
*/
export function exampleSchema(): SchemaAST;
/**
* @param {string} input
* @returns {SchemaAST}
*/
export function parseSchema(input: string): SchemaAST;
/**
* @param {SchemaAST} ast
*/
export function validateAST(ast: SchemaAST): void;
export type SchemaASTBlock = { datasource: Datasource };

export interface SchemaAST {
    datasources: Datasource[];
}

export type Provider = "postgres" | "cockroachdb" | "mysql" | "mariadb" | "sqlserver" | "sqlite" | "mongodb";

export type Url = { _tag: "static"; value: string } | { _tag: "env"; value: string };

export interface DatasourceDb {
    provider: Provider;
    url: Url;
    shadowDatabaseUrl: Url | null;
}

export type DatamodelError = string;

export interface Diagnostics {
    errors: DatamodelError[];
}

export type Datasource = { _tag: "db"; value: DatasourceDb };

