//! WASM bridge types for league conference and division management.
//!
//! These types provide JavaScript/TypeScript-compatible wrappers around the
//! core fbsim-core Rust types. They are intended exclusively for JS/TS
//! consumers via WebAssembly and are not part of the public Rust API.
//!
//! Feature-gated behind the `wasm` Cargo feature. Compiled to WebAssembly
//! via `wasm-pack`.

use wasm_bindgen::prelude::*;

use crate::league::season::conference::{LeagueConference, LeagueDivision};

/// A WASM-friendly wrapper around `LeagueDivision`.
#[wasm_bindgen(js_name = "LeagueDivision")]
pub struct WasmLeagueDivision {
    inner: LeagueDivision,
}

#[wasm_bindgen(js_class = "LeagueDivision")]
impl WasmLeagueDivision {
    /// Creates a new empty division.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmLeagueDivision {
        WasmLeagueDivision {
            inner: LeagueDivision::new(),
        }
    }

    /// Creates a new division with the given name.
    #[wasm_bindgen(js_name = "withName")]
    pub fn with_name(name: &str) -> WasmLeagueDivision {
        WasmLeagueDivision {
            inner: LeagueDivision::with_name(name),
        }
    }

    /// Gets the division name.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Sets the division name.
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        *self.inner.name_mut() = name.to_string();
    }

    /// Adds a team to the division by ID.
    #[wasm_bindgen(js_name = "addTeam")]
    pub fn add_team(&mut self, team_id: usize) -> Result<(), JsError> {
        self.inner
            .add_team(team_id)
            .map_err(|e| JsError::new(&e))
    }

    /// Returns the team IDs in this division.
    #[wasm_bindgen(getter)]
    pub fn teams(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.teams())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Checks if a team is in this division.
    #[wasm_bindgen(js_name = "containsTeam")]
    pub fn contains_team(&self, team_id: usize) -> bool {
        self.inner.contains_team(team_id)
    }

    /// Returns the number of teams in this division.
    #[wasm_bindgen(getter, js_name = "numTeams")]
    pub fn num_teams(&self) -> usize {
        self.inner.num_teams()
    }

    /// Returns the division as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmLeagueDivision {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmLeagueDivision {
    /// Returns a reference to the inner `LeagueDivision`.
    pub fn inner(&self) -> &LeagueDivision {
        &self.inner
    }

    /// Consumes the wrapper, returning the inner `LeagueDivision`.
    pub fn into_inner(self) -> LeagueDivision {
        self.inner
    }

    /// Creates a wrapper from an existing `LeagueDivision`.
    pub fn from_inner(inner: LeagueDivision) -> Self {
        WasmLeagueDivision { inner }
    }
}

/// A WASM-friendly wrapper around `LeagueConference`.
#[wasm_bindgen(js_name = "LeagueConference")]
pub struct WasmLeagueConference {
    inner: LeagueConference,
}

#[wasm_bindgen(js_class = "LeagueConference")]
impl WasmLeagueConference {
    /// Creates a new empty conference.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmLeagueConference {
        WasmLeagueConference {
            inner: LeagueConference::new(),
        }
    }

    /// Creates a new conference with the given name.
    #[wasm_bindgen(js_name = "withName")]
    pub fn with_name(name: &str) -> WasmLeagueConference {
        WasmLeagueConference {
            inner: LeagueConference::with_name(name),
        }
    }

    /// Gets the conference name.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Sets the conference name.
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        *self.inner.name_mut() = name.to_string();
    }

    /// Adds a division to the conference (takes ownership of the division).
    #[wasm_bindgen(js_name = "addDivision")]
    pub fn add_division(&mut self, division: WasmLeagueDivision) -> Result<(), JsError> {
        self.inner
            .add_division(division.into_inner())
            .map_err(|e| JsError::new(&e))
    }

    /// Returns the divisions as a JSON array.
    #[wasm_bindgen(getter)]
    pub fn divisions(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.divisions())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns all team IDs across all divisions.
    #[wasm_bindgen(js_name = "allTeams")]
    pub fn all_teams(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner.all_teams())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Checks if a team is in this conference.
    #[wasm_bindgen(js_name = "containsTeam")]
    pub fn contains_team(&self, team_id: usize) -> bool {
        self.inner.contains_team(team_id)
    }

    /// Returns the number of divisions in this conference.
    #[wasm_bindgen(getter, js_name = "numDivisions")]
    pub fn num_divisions(&self) -> usize {
        self.inner.num_divisions()
    }

    /// Returns the total number of teams in this conference.
    #[wasm_bindgen(getter, js_name = "numTeams")]
    pub fn num_teams(&self) -> usize {
        self.inner.num_teams()
    }

    /// Returns the conference as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmLeagueConference {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmLeagueConference {
    /// Returns a reference to the inner `LeagueConference`.
    pub fn inner(&self) -> &LeagueConference {
        &self.inner
    }

    /// Consumes the wrapper, returning the inner `LeagueConference`.
    pub fn into_inner(self) -> LeagueConference {
        self.inner
    }

    /// Creates a wrapper from an existing `LeagueConference`.
    pub fn from_inner(inner: LeagueConference) -> Self {
        WasmLeagueConference { inner }
    }
}
