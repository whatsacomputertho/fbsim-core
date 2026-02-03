//! WASM bindings for fbsim-core
//!
//! This module provides WebAssembly bindings for the fbsim-core library,
//! enabling use from JavaScript/TypeScript in both browser and Node.js environments.

mod game;
mod rng;
mod team;

pub use game::*;
pub use rng::*;
pub use team::*;

use wasm_bindgen::prelude::*;

/// Initialize the WASM module with better error handling.
/// This is called automatically when the module is loaded.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Returns the library version.
#[wasm_bindgen(js_name = "getVersion")]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
