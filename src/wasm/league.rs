//! WASM wrappers for the top-level League type.
//!
//! Provides JavaScript-friendly wrappers around `League`, exposing team
//! management, season lifecycle, schedule generation, simulation at every
//! granularity, and cross-season query methods via wasm-bindgen.

use wasm_bindgen::prelude::*;

use crate::league::season::LeagueSeasonScheduleOptions;
use crate::league::League;
use crate::wasm::conference::WasmLeagueConference;
use crate::wasm::rng::WasmRng;
use crate::wasm::team::WasmFootballTeam;

/// A WASM-friendly wrapper around `League`.
#[wasm_bindgen(js_name = "League")]
pub struct WasmLeague {
    inner: League,
}

#[wasm_bindgen(js_class = "League")]
impl WasmLeague {
    // ---------------------------------------------------------------
    // Construction & Serialization
    // ---------------------------------------------------------------

    /// Creates a new empty league.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmLeague {
        WasmLeague {
            inner: League::new(),
        }
    }

    /// Serializes the league to a JSON-compatible JS value.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Deserializes a league from a JSON-compatible JS value.
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: JsValue) -> Result<WasmLeague, JsError> {
        let inner: League =
            serde_wasm_bindgen::from_value(value).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WasmLeague { inner })
    }

    // ---------------------------------------------------------------
    // Team Management
    // ---------------------------------------------------------------

    /// Adds a new team to the league (auto-assigns an ID).
    #[wasm_bindgen(js_name = "addTeam")]
    pub fn add_team(&mut self) {
        self.inner.add_team();
    }

    /// Returns the league teams as a JSON object (BTreeMap<usize, LeagueTeam>).
    #[wasm_bindgen(getter)]
    pub fn teams(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.teams())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a specific team as a JSON object, or `undefined` if not found.
    #[wasm_bindgen(js_name = "getTeam")]
    pub fn get_team(&self, id: usize) -> Result<JsValue, JsError> {
        match self.inner.team(id) {
            Some(team) => serde_wasm_bindgen::to_value(team)
                .map_err(|e| JsError::new(&e.to_string())),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    // ---------------------------------------------------------------
    // Season Management
    // ---------------------------------------------------------------

    /// Creates a new season (archives the current one if complete).
    #[wasm_bindgen(js_name = "addSeason")]
    pub fn add_season(&mut self) -> Result<(), JsError> {
        self.inner.add_season().map_err(|e| JsError::new(&e))
    }

    /// Returns the current season as a JSON object, or `undefined` if none.
    #[wasm_bindgen(getter, js_name = "currentSeason")]
    pub fn current_season(&self) -> Result<JsValue, JsError> {
        match self.inner.current_season() {
            Some(season) => serde_wasm_bindgen::to_value(season)
                .map_err(|e| JsError::new(&e.to_string())),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Returns the past seasons as a JSON array.
    #[wasm_bindgen(getter)]
    pub fn seasons(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.seasons())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a specific season by year as JSON, or `undefined` if not found.
    #[wasm_bindgen(js_name = "getSeason")]
    pub fn get_season(&self, year: usize) -> Result<JsValue, JsError> {
        match self.inner.season(year) {
            Some(season) => serde_wasm_bindgen::to_value(season)
                .map_err(|e| JsError::new(&e.to_string())),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Returns a specific week as JSON, or `undefined` if not found.
    #[wasm_bindgen(js_name = "getWeek")]
    pub fn get_week(&self, year: usize, week: usize) -> Result<JsValue, JsError> {
        match self.inner.week(year, week) {
            Some(w) => serde_wasm_bindgen::to_value(w)
                .map_err(|e| JsError::new(&e.to_string())),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Returns a specific matchup as JSON, or `undefined` if not found.
    #[wasm_bindgen(js_name = "getMatchup")]
    pub fn get_matchup(
        &self,
        year: usize,
        week: usize,
        matchup: usize,
    ) -> Result<JsValue, JsError> {
        match self.inner.matchup(year, week, matchup) {
            Some(m) => serde_wasm_bindgen::to_value(m)
                .map_err(|e| JsError::new(&e.to_string())),
            None => Ok(JsValue::UNDEFINED),
        }
    }

    // ---------------------------------------------------------------
    // Season Team & Conference Management
    // ---------------------------------------------------------------

    /// Adds a team roster to the current season.
    #[wasm_bindgen(js_name = "addSeasonTeam")]
    pub fn add_season_team(
        &mut self,
        id: usize,
        team: &WasmFootballTeam,
    ) -> Result<(), JsError> {
        self.inner
            .add_season_team(id, team.inner().clone())
            .map_err(|e| JsError::new(&e))
    }

    /// Adds a conference to the current season (takes ownership).
    #[wasm_bindgen(js_name = "addConference")]
    pub fn add_conference(
        &mut self,
        conference: WasmLeagueConference,
    ) -> Result<(), JsError> {
        let season = self
            .inner
            .current_season_mut()
            .as_mut()
            .ok_or_else(|| JsError::new("No current season"))?;
        season
            .add_conference(conference.into_inner())
            .map_err(|e| JsError::new(&e))
    }

    // ---------------------------------------------------------------
    // Schedule & Simulation
    // ---------------------------------------------------------------

    /// Generates the schedule for the current season.
    #[wasm_bindgen(js_name = "generateSchedule")]
    pub fn generate_schedule(
        &mut self,
        options: LeagueSeasonScheduleOptions,
        rng: &mut WasmRng,
    ) -> Result<(), JsError> {
        self.inner
            .generate_schedule(options, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates the entire current season.
    #[wasm_bindgen(js_name = "sim")]
    pub fn sim(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim(rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a single week of the current season.
    #[wasm_bindgen(js_name = "simWeek")]
    pub fn sim_week(&mut self, week: usize, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim_week(week, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a single matchup of the current season. Returns game log as JSON.
    #[wasm_bindgen(js_name = "simMatchup")]
    pub fn sim_matchup(
        &mut self,
        week: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let game = self
            .inner
            .sim_matchup(week, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Simulates a single play of a matchup. Returns game log as JSON if the
    /// game finished on this play, or `undefined` if still in progress.
    #[wasm_bindgen(js_name = "simPlay")]
    pub fn sim_play(
        &mut self,
        week: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let result = self
            .inner
            .sim_play(week, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        match result {
            Some(game) => {
                serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    // ---------------------------------------------------------------
    // Cross-Season Queries
    // ---------------------------------------------------------------

    /// Returns all matchups for a team across all seasons as JSON.
    #[wasm_bindgen(js_name = "teamMatchups")]
    pub fn team_matchups(&self, id: usize) -> Result<JsValue, JsError> {
        let matchups = self
            .inner
            .team_matchups(id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&matchups).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns all matchups for a team in a specific season as JSON.
    #[wasm_bindgen(js_name = "teamSeasonMatchups")]
    pub fn team_season_matchups(
        &self,
        id: usize,
        year: usize,
    ) -> Result<JsValue, JsError> {
        let matchups = self
            .inner
            .team_season_matchups(id, year)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&matchups).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a team's all-time playoff record as JSON.
    #[wasm_bindgen(js_name = "teamPlayoffRecord")]
    pub fn team_playoff_record(&self, id: usize) -> Result<JsValue, JsError> {
        let record = self
            .inner
            .team_playoff_record(id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&record).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a team's total championship appearances.
    #[wasm_bindgen(js_name = "teamChampionshipAppearances")]
    pub fn team_championship_appearances(&self, id: usize) -> Result<usize, JsError> {
        self.inner
            .team_championship_appearances(id)
            .map_err(|e| JsError::new(&e))
    }

    /// Returns a team's total championship wins.
    #[wasm_bindgen(js_name = "teamChampionshipWins")]
    pub fn team_championship_wins(&self, id: usize) -> Result<usize, JsError> {
        self.inner
            .team_championship_wins(id)
            .map_err(|e| JsError::new(&e))
    }
}

impl Default for WasmLeague {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmLeague {
    /// Returns a reference to the inner `League`.
    pub fn inner(&self) -> &League {
        &self.inner
    }

    /// Returns a mutable reference to the inner `League`.
    pub fn inner_mut(&mut self) -> &mut League {
        &mut self.inner
    }

    /// Consumes the wrapper, returning the inner `League`.
    pub fn into_inner(self) -> League {
        self.inner
    }

    /// Creates a wrapper from an existing `League`.
    pub fn from_inner(inner: League) -> Self {
        WasmLeague { inner }
    }
}
