//! WASM bridge types for fbsim-core.
//!
//! These types provide JavaScript/TypeScript-compatible wrappers around the
//! core fbsim-core Rust types. They are intended exclusively for JS/TS
//! consumers via WebAssembly and are not part of the public Rust API.
//!
//! Feature-gated behind the `wasm` Cargo feature. Compiled to WebAssembly
//! via `wasm-pack`.

mod conference;
mod game;
mod league;
mod play;
mod rng;
mod season;
mod team;

pub use conference::*;
pub use game::*;
pub use league::*;
pub use play::*;
pub use rng::*;
pub use season::*;
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
