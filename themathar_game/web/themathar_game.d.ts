/* tslint:disable */
/* eslint-disable */

export function receive_identity(lobby_id: number, player_id: number): void;

export function receive_state(active_player_id: number, turn_number: number): void;

export function run_game(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly receive_identity: (a: number, b: number) => void;
    readonly receive_state: (a: number, b: number) => void;
    readonly main: (a: number, b: number) => number;
    readonly run_game: () => void;
    readonly __wasm_bindgen_func_elem_154076: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_158290: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_164238: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_164461: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_158304: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_164458: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_164459: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_154109: (a: number, b: number) => void;
    readonly __wasm_bindgen_func_elem_164457: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
