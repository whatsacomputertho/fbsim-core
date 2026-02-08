//! WASM bridge types for game simulation.
//!
//! These types provide JavaScript/TypeScript-compatible wrappers around the
//! core fbsim-core Rust types. They are intended exclusively for JS/TS
//! consumers via WebAssembly and are not part of the public Rust API.
//!
//! Feature-gated behind the `wasm` Cargo feature. Compiled to WebAssembly
//! via `wasm-pack`.

use wasm_bindgen::prelude::*;

use crate::game::context::GameContext;
use crate::game::play::{Game as CoreGame, GameSimulator};
use crate::wasm::play::{Drive, Play};
use crate::wasm::rng::WasmRng;
use crate::wasm::team::WasmFootballTeam;

/// A WASM-friendly wrapper around the core `Game` type.
///
/// This class holds the accumulated game log (drives and plays) and
/// provides query methods for accessing enriched play data, stats,
/// and serialization.
#[wasm_bindgen(js_name = "Game")]
pub struct WasmGame {
    inner: CoreGame,
}

#[wasm_bindgen(js_class = "Game")]
impl WasmGame {
    /// Creates a new empty game.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame {
            inner: CoreGame::new(),
        }
    }

    /// Returns true if the game is complete.
    #[wasm_bindgen(getter)]
    pub fn complete(&self) -> bool {
        self.inner.complete()
    }

    /// Returns the number of drives in the game so far.
    #[wasm_bindgen(getter, js_name = "driveCount")]
    pub fn drive_count(&self) -> usize {
        self.inner.drives().len()
    }

    /// Returns the total number of plays in the game so far.
    #[wasm_bindgen(getter, js_name = "playCount")]
    pub fn play_count(&self) -> usize {
        self.inner.drives().iter().map(|d| d.plays().len()).sum()
    }

    /// Returns a specific drive by index with enriched play data.
    ///
    /// # Arguments
    /// * `index` - The drive index (0-based)
    #[wasm_bindgen(js_name = "getDrive")]
    pub fn get_drive(&self, index: usize) -> Result<Drive, JsError> {
        self.inner
            .drives()
            .get(index)
            .map(Drive::from)
            .ok_or_else(|| JsError::new(&format!("Drive {} not found", index)))
    }

    /// Returns the latest play with enriched data, or null if no plays
    /// have been simulated.
    #[wasm_bindgen(js_name = "getLatestPlay")]
    pub fn get_latest_play(&self) -> Option<Play> {
        self.inner
            .drives()
            .last()
            .and_then(|d| d.plays().last())
            .map(Play::from)
    }

    /// Returns the home team's offensive stats as a JSON object.
    #[wasm_bindgen(js_name = "homeStats")]
    pub fn home_stats(&self) -> Result<JsValue, JsError> {
        let stats = self.inner.home_stats();
        serde_wasm_bindgen::to_value(&stats).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the away team's offensive stats as a JSON object.
    #[wasm_bindgen(js_name = "awayStats")]
    pub fn away_stats(&self) -> Result<JsValue, JsError> {
        let stats = self.inner.away_stats();
        serde_wasm_bindgen::to_value(&stats).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the game as a JSON-serializable object.
    ///
    /// The structure matches the core `Game` type used in league matchups.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmGame {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmGame {
    /// Returns a reference to the inner `Game`.
    pub fn inner(&self) -> &CoreGame {
        &self.inner
    }

    /// Returns a mutable reference to the inner `Game`.
    pub fn inner_mut(&mut self) -> &mut CoreGame {
        &mut self.inner
    }
}

/// A WASM-friendly wrapper around the core `GameSimulator` type.
///
/// Provides methods to simulate plays, drives, or full games.
/// Mirrors the Rust `GameSimulator` API, accepting teams, context,
/// and a mutable game reference as parameters.
#[wasm_bindgen(js_name = "GameSimulator")]
pub struct WasmGameSimulator {
    inner: GameSimulator,
}

#[wasm_bindgen(js_class = "GameSimulator")]
impl WasmGameSimulator {
    /// Creates a new game simulator.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGameSimulator {
        WasmGameSimulator {
            inner: GameSimulator::new(),
        }
    }

    /// Simulates the next play.
    ///
    /// Mutates the `game` in place and returns the updated `GameContext`.
    /// Call `game.getLatestPlay()` to inspect the play that was simulated.
    ///
    /// # Arguments
    /// * `home` - The home team
    /// * `away` - The away team
    /// * `context` - The current game context
    /// * `game` - The game log (mutated in place)
    /// * `rng` - The random number generator
    #[wasm_bindgen(js_name = "simPlay")]
    pub fn sim_play(
        &self,
        home: &WasmFootballTeam,
        away: &WasmFootballTeam,
        context: GameContext,
        game: &mut WasmGame,
        rng: &mut WasmRng,
    ) -> Result<GameContext, JsError> {
        self.inner
            .sim_play(
                home.inner(),
                away.inner(),
                context,
                &mut game.inner,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates until the current drive is complete.
    ///
    /// Mutates the `game` in place and returns the updated `GameContext`.
    ///
    /// # Arguments
    /// * `home` - The home team
    /// * `away` - The away team
    /// * `context` - The current game context
    /// * `game` - The game log (mutated in place)
    /// * `rng` - The random number generator
    #[wasm_bindgen(js_name = "simDrive")]
    pub fn sim_drive(
        &self,
        home: &WasmFootballTeam,
        away: &WasmFootballTeam,
        context: GameContext,
        game: &mut WasmGame,
        rng: &mut WasmRng,
    ) -> Result<GameContext, JsError> {
        self.inner
            .sim_drive(
                home.inner(),
                away.inner(),
                context,
                &mut game.inner,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates the rest of the game.
    ///
    /// Mutates the `game` in place and returns the updated `GameContext`.
    ///
    /// # Arguments
    /// * `home` - The home team
    /// * `away` - The away team
    /// * `context` - The current game context
    /// * `game` - The game log (mutated in place)
    /// * `rng` - The random number generator
    #[wasm_bindgen(js_name = "simGame")]
    pub fn sim_game(
        &self,
        home: &WasmFootballTeam,
        away: &WasmFootballTeam,
        context: GameContext,
        game: &mut WasmGame,
        rng: &mut WasmRng,
    ) -> Result<GameContext, JsError> {
        self.inner
            .sim_game(
                home.inner(),
                away.inner(),
                context,
                &mut game.inner,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))
    }
}

impl Default for WasmGameSimulator {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a new default `GameContext`.
///
/// This is a convenience function for JS/TS consumers. The returned
/// context represents the start of a game (kickoff, Q1, 15:00).
#[wasm_bindgen(js_name = "createGameContext")]
pub fn create_game_context() -> GameContext {
    GameContext::new()
}
