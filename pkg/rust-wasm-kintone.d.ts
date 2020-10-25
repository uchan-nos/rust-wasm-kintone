/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @returns {string}
*/
export function greet(name: string): string;
/**
* @param {Uint8Array} input_img
* @param {any} filters_checkbox
* @param {any} filter_params_js
* @returns {Uint8Array}
*/
export function convimg(input_img: Uint8Array, filters_checkbox: any, filter_params_js: any): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number, c: number) => void;
  readonly convimg: (a: number, b: number, c: number) => number;
  readonly main: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        