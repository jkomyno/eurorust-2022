/* tslint:disable */
/* eslint-disable */
/**
* @returns {MiniSchema}
*/
export function exampleSchema(): MiniSchema;
export type Provider = "sqlite" | "postgres";

export type Url = { static: string } | { env: string };

export type UrlTag = ({ _tag: "static" } & string) | ({ _tag: "env" } & string);

export type UrlTagContent = { _tag: "static"; value: string } | { _tag: "env"; value: string };

export interface MiniSchema {
    providers: Provider[];
    shadowDatabaseUrl: UrlTagContent | null;
    id: number | null;
}

