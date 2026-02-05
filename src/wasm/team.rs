//! WASM wrappers for team types.
//!
//! Provides JavaScript-friendly wrappers around the team module types,
//! exposing constructors and methods via wasm-bindgen.

use wasm_bindgen::prelude::*;

use crate::team::coach::FootballTeamCoach;
use crate::team::defense::FootballTeamDefense;
use crate::team::offense::FootballTeamOffense;
use crate::team::FootballTeam;

/// A WASM-friendly wrapper around `FootballTeam`.
///
/// This wrapper provides JavaScript-accessible constructors and methods
/// for creating and manipulating football teams.
#[wasm_bindgen(js_name = "FootballTeam")]
pub struct WasmFootballTeam {
    inner: FootballTeam,
}

#[wasm_bindgen(js_class = "FootballTeam")]
impl WasmFootballTeam {
    /// Creates a new team with default values.
    ///
    /// The team will have the name "Null Island Defaults" and all
    /// attributes set to 50.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFootballTeam {
        WasmFootballTeam {
            inner: FootballTeam::new(),
        }
    }

    /// Creates a new team with the specified overall ratings.
    ///
    /// # Arguments
    /// * `name` - The team's full name (max 64 characters)
    /// * `short_name` - The team's abbreviation (max 4 characters)
    /// * `offense_overall` - Overall offensive rating (0-100)
    /// * `defense_overall` - Overall defensive rating (0-100)
    ///
    /// # Errors
    /// Returns an error if any rating is out of range or names are too long.
    #[wasm_bindgen(js_name = "fromOveralls")]
    pub fn from_overalls(
        name: &str,
        short_name: &str,
        offense_overall: u32,
        defense_overall: u32,
    ) -> Result<WasmFootballTeam, JsError> {
        FootballTeam::from_overalls(name, short_name, offense_overall, defense_overall)
            .map(|inner| WasmFootballTeam { inner })
            .map_err(|e| JsError::new(&e))
    }

    /// Gets the team's name.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Gets the team's short name / abbreviation.
    #[wasm_bindgen(getter, js_name = "shortName")]
    pub fn short_name(&self) -> String {
        self.inner.short_name().to_string()
    }

    /// Gets the team's offensive overall rating.
    #[wasm_bindgen(getter, js_name = "offenseOverall")]
    pub fn offense_overall(&self) -> u32 {
        use crate::game::score::ScoreSimulatable;
        self.inner.offense_overall()
    }

    /// Gets the team's defensive overall rating.
    #[wasm_bindgen(getter, js_name = "defenseOverall")]
    pub fn defense_overall(&self) -> u32 {
        use crate::game::score::ScoreSimulatable;
        self.inner.defense_overall()
    }

    /// Returns the team as a JSON-serializable object.
    ///
    /// This allows full access to all team properties from JavaScript.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Creates a team from a JSON object.
    ///
    /// # Arguments
    /// * `value` - A JavaScript object with team properties
    ///
    /// # Errors
    /// Returns an error if the object is invalid or has out-of-range values.
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: JsValue) -> Result<WasmFootballTeam, JsError> {
        let inner: FootballTeam =
            serde_wasm_bindgen::from_value(value).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WasmFootballTeam { inner })
    }
}

impl Default for WasmFootballTeam {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmFootballTeam {
    /// Returns a reference to the inner `FootballTeam`.
    ///
    /// This is used internally for simulation functions.
    pub fn inner(&self) -> &FootballTeam {
        &self.inner
    }

    /// Creates a wrapper from an existing `FootballTeam`.
    pub fn from_inner(inner: FootballTeam) -> Self {
        WasmFootballTeam { inner }
    }
}

/// A WASM-friendly wrapper around `FootballTeamCoach`.
#[wasm_bindgen(js_name = "FootballTeamCoach")]
pub struct WasmFootballTeamCoach {
    inner: FootballTeamCoach,
}

#[wasm_bindgen(js_class = "FootballTeamCoach")]
impl WasmFootballTeamCoach {
    /// Creates a new coach with default values (all attributes at 50).
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFootballTeamCoach {
        WasmFootballTeamCoach {
            inner: FootballTeamCoach::new(),
        }
    }

    /// Gets the coach's risk-taking tendency (0-100).
    /// Higher values mean the coach is more likely to go for it on 4th down, etc.
    #[wasm_bindgen(getter, js_name = "riskTaking")]
    pub fn risk_taking(&self) -> u32 {
        self.inner.risk_taking()
    }

    /// Gets the coach's run-pass tendency (0-100).
    /// Lower values favor running, higher values favor passing.
    #[wasm_bindgen(getter, js_name = "runPass")]
    pub fn run_pass(&self) -> u32 {
        self.inner.run_pass()
    }

    /// Gets the coach's up-tempo tendency (0-100).
    /// Higher values mean faster play calling.
    #[wasm_bindgen(getter, js_name = "upTempo")]
    pub fn up_tempo(&self) -> u32 {
        self.inner.up_tempo()
    }

    /// Returns the coach as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmFootballTeamCoach {
    fn default() -> Self {
        Self::new()
    }
}

/// A WASM-friendly wrapper around `FootballTeamOffense`.
#[wasm_bindgen(js_name = "FootballTeamOffense")]
pub struct WasmFootballTeamOffense {
    inner: FootballTeamOffense,
}

#[wasm_bindgen(js_class = "FootballTeamOffense")]
impl WasmFootballTeamOffense {
    /// Creates a new offense with default values (all attributes at 50).
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFootballTeamOffense {
        WasmFootballTeamOffense {
            inner: FootballTeamOffense::new(),
        }
    }

    /// Creates an offense with all attributes set to the given overall rating.
    #[wasm_bindgen(js_name = "fromOverall")]
    pub fn from_overall(overall: u32) -> Result<WasmFootballTeamOffense, JsError> {
        FootballTeamOffense::from_overall(overall)
            .map(|inner| WasmFootballTeamOffense { inner })
            .map_err(|e| JsError::new(&e))
    }

    /// Gets the overall offensive rating.
    #[wasm_bindgen(getter)]
    pub fn overall(&self) -> u32 {
        self.inner.overall()
    }

    /// Gets the passing rating.
    #[wasm_bindgen(getter)]
    pub fn passing(&self) -> u32 {
        self.inner.passing()
    }

    /// Gets the rushing rating.
    #[wasm_bindgen(getter)]
    pub fn rushing(&self) -> u32 {
        self.inner.rushing()
    }

    /// Gets the receiving rating.
    #[wasm_bindgen(getter)]
    pub fn receiving(&self) -> u32 {
        self.inner.receiving()
    }

    /// Gets the blocking rating.
    #[wasm_bindgen(getter)]
    pub fn blocking(&self) -> u32 {
        self.inner.blocking()
    }

    /// Gets the scrambling rating.
    #[wasm_bindgen(getter)]
    pub fn scrambling(&self) -> u32 {
        self.inner.scrambling()
    }

    /// Gets the turnovers rating (higher = fewer turnovers).
    #[wasm_bindgen(getter)]
    pub fn turnovers(&self) -> u32 {
        self.inner.turnovers()
    }

    /// Gets the field goals rating.
    #[wasm_bindgen(getter, js_name = "fieldGoals")]
    pub fn field_goals(&self) -> u32 {
        self.inner.field_goals()
    }

    /// Gets the punting rating.
    #[wasm_bindgen(getter)]
    pub fn punting(&self) -> u32 {
        self.inner.punting()
    }

    /// Gets the kickoffs rating.
    #[wasm_bindgen(getter)]
    pub fn kickoffs(&self) -> u32 {
        self.inner.kickoffs()
    }

    /// Gets the kick return defense rating.
    #[wasm_bindgen(getter, js_name = "kickReturnDefense")]
    pub fn kick_return_defense(&self) -> u32 {
        self.inner.kick_return_defense()
    }

    /// Returns the offense as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmFootballTeamOffense {
    fn default() -> Self {
        Self::new()
    }
}

/// A WASM-friendly wrapper around `FootballTeamDefense`.
#[wasm_bindgen(js_name = "FootballTeamDefense")]
pub struct WasmFootballTeamDefense {
    inner: FootballTeamDefense,
}

#[wasm_bindgen(js_class = "FootballTeamDefense")]
impl WasmFootballTeamDefense {
    /// Creates a new defense with default values (all attributes at 50).
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmFootballTeamDefense {
        WasmFootballTeamDefense {
            inner: FootballTeamDefense::new(),
        }
    }

    /// Creates a defense with all attributes set to the given overall rating.
    #[wasm_bindgen(js_name = "fromOverall")]
    pub fn from_overall(overall: u32) -> Result<WasmFootballTeamDefense, JsError> {
        FootballTeamDefense::from_overall(overall)
            .map(|inner| WasmFootballTeamDefense { inner })
            .map_err(|e| JsError::new(&e))
    }

    /// Gets the overall defensive rating.
    #[wasm_bindgen(getter)]
    pub fn overall(&self) -> u32 {
        self.inner.overall()
    }

    /// Gets the blitzing rating.
    #[wasm_bindgen(getter)]
    pub fn blitzing(&self) -> u32 {
        self.inner.blitzing()
    }

    /// Gets the rush defense rating.
    #[wasm_bindgen(getter, js_name = "rushDefense")]
    pub fn rush_defense(&self) -> u32 {
        self.inner.rush_defense()
    }

    /// Gets the pass defense rating.
    #[wasm_bindgen(getter, js_name = "passDefense")]
    pub fn pass_defense(&self) -> u32 {
        self.inner.pass_defense()
    }

    /// Gets the coverage rating.
    #[wasm_bindgen(getter)]
    pub fn coverage(&self) -> u32 {
        self.inner.coverage()
    }

    /// Gets the turnovers rating (higher = more turnovers forced).
    #[wasm_bindgen(getter)]
    pub fn turnovers(&self) -> u32 {
        self.inner.turnovers()
    }

    /// Gets the kick returning rating.
    #[wasm_bindgen(getter, js_name = "kickReturning")]
    pub fn kick_returning(&self) -> u32 {
        self.inner.kick_returning()
    }

    /// Returns the defense as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmFootballTeamDefense {
    fn default() -> Self {
        Self::new()
    }
}
