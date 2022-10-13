/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Provider {
  Postgres,
  MySQL,
  SQLite,
}
/**
*/
export class ScalarParams {
  free(): void;
/**
*/
  id: number;
/**
*/
  letter: string;
/**
*/
  toggle: boolean;
}
/**
*/
export class StringParams {
  free(): void;
/**
*/
  id: string;
}
/**
*/
export class VecIntParams {
  free(): void;
/**
*/
  id: Uint32Array;
}
