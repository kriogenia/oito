/* tslint:disable */
/* eslint-disable */
/**
*/
export class OitoWasm {
  free(): void;
/**
*/
  constructor();
/**
*/
  tick(): void;
/**
*/
  frame_tick(): void;
/**
* @param {Uint8Array} data
*/
  load(data: Uint8Array): void;
/**
* Emmulates the pressing of the desired key
* @param {KeyboardEvent} key
*/
  key_press(key: KeyboardEvent): void;
/**
* Emmulates the release of the desired key
* @param {KeyboardEvent} key
*/
  key_release(key: KeyboardEvent): void;
/**
* @param {number} scale
*/
  draw(scale: number): void;
/**
* @returns {boolean}
*/
  sound(): boolean;
/**
*/
  reset(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_oitowasm_free: (a: number) => void;
  readonly oitowasm_new: () => number;
  readonly oitowasm_tick: (a: number) => void;
  readonly oitowasm_frame_tick: (a: number) => void;
  readonly oitowasm_load: (a: number, b: number) => void;
  readonly oitowasm_key_press: (a: number, b: number) => void;
  readonly oitowasm_key_release: (a: number, b: number) => void;
  readonly oitowasm_draw: (a: number, b: number) => void;
  readonly oitowasm_sound: (a: number) => number;
  readonly oitowasm_reset: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
