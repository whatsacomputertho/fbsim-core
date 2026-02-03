//! WASM-compatible random number generator wrapper.
//!
//! Provides a seedable RNG that works in WASM environments using
//! `getrandom` with the `js` feature for entropy from `crypto.getRandomValues()`.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use wasm_bindgen::prelude::*;

/// A WASM-compatible random number generator.
///
/// This wrapper uses `SmallRng` internally, which is suitable for
/// game simulations where cryptographic security is not required.
#[wasm_bindgen]
pub struct WasmRng {
    inner: SmallRng,
}

#[wasm_bindgen]
impl WasmRng {
    /// Creates a new RNG with a random seed.
    ///
    /// Entropy is obtained from the JavaScript runtime via `crypto.getRandomValues()`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmRng {
        WasmRng {
            inner: SmallRng::from_entropy(),
        }
    }

    /// Creates a new RNG with a specific seed for reproducible simulations.
    ///
    /// Use this when you need deterministic results, such as for testing
    /// or replay functionality.
    #[wasm_bindgen(js_name = "fromSeed")]
    pub fn from_seed(seed: u64) -> WasmRng {
        WasmRng {
            inner: SmallRng::seed_from_u64(seed),
        }
    }

    /// Generates a random 32-bit unsigned integer.
    #[wasm_bindgen(js_name = "nextU32")]
    pub fn next_u32(&mut self) -> u32 {
        self.inner.gen()
    }

    /// Generates a random 64-bit unsigned integer.
    #[wasm_bindgen(js_name = "nextU64")]
    pub fn next_u64(&mut self) -> u64 {
        self.inner.gen()
    }

    /// Generates a random float between 0.0 and 1.0.
    #[wasm_bindgen(js_name = "nextFloat")]
    pub fn next_float(&mut self) -> f64 {
        self.inner.gen()
    }
}

impl Default for WasmRng {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmRng {
    /// Returns a mutable reference to the inner RNG for use with Rust APIs.
    pub fn as_rng(&mut self) -> &mut SmallRng {
        &mut self.inner
    }

    /// Returns a mutable reference to the inner RNG.
    /// Alias for `as_rng` to match the naming convention of other wrappers.
    pub fn inner_mut(&mut self) -> &mut SmallRng {
        &mut self.inner
    }
}
