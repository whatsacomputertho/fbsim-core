//! WASM wrappers for league season types.
//!
//! Provides JavaScript-friendly wrappers around `LeagueSeason`, exposing
//! construction, team and conference management, schedule generation,
//! simulation at every granularity (season/week/matchup/play), playoffs,
//! and standings queries via wasm-bindgen.

use wasm_bindgen::prelude::*;

use crate::league::season::{
    LeagueSeason, LeagueSeasonPlayoffOptions, LeagueSeasonScheduleOptions,
};
use crate::wasm::conference::WasmLeagueConference;
use crate::wasm::rng::WasmRng;
use crate::wasm::team::WasmFootballTeam;

/// A WASM-friendly wrapper around `LeagueSeason`.
#[wasm_bindgen(js_name = "LeagueSeason")]
pub struct WasmLeagueSeason {
    inner: LeagueSeason,
}

#[wasm_bindgen(js_class = "LeagueSeason")]
impl WasmLeagueSeason {
    // ---------------------------------------------------------------
    // Construction & Properties
    // ---------------------------------------------------------------

    /// Creates a new empty season (defaults to the current year).
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmLeagueSeason {
        WasmLeagueSeason {
            inner: LeagueSeason::new(),
        }
    }

    /// Gets the season year.
    #[wasm_bindgen(getter)]
    pub fn year(&self) -> usize {
        *self.inner.year()
    }

    /// Sets the season year.
    #[wasm_bindgen(setter)]
    pub fn set_year(&mut self, year: usize) {
        *self.inner.year_mut() = year;
    }

    /// Returns true if the season has started (any game played).
    #[wasm_bindgen(getter)]
    pub fn started(&self) -> bool {
        self.inner.started()
    }

    /// Returns true if the regular season is complete.
    #[wasm_bindgen(getter, js_name = "regularSeasonComplete")]
    pub fn regular_season_complete(&self) -> bool {
        self.inner.regular_season_complete()
    }

    /// Returns true if the entire season (including playoffs) is complete.
    #[wasm_bindgen(getter)]
    pub fn complete(&self) -> bool {
        self.inner.complete()
    }

    // ---------------------------------------------------------------
    // Team Management
    // ---------------------------------------------------------------

    /// Adds a team to the season.
    #[wasm_bindgen(js_name = "addTeam")]
    pub fn add_team(&mut self, id: usize, team: &WasmFootballTeam) -> Result<(), JsError> {
        self.inner
            .add_team(id, team.inner().clone())
            .map_err(|e| JsError::new(&e))
    }

    /// Checks if a team exists in the season.
    #[wasm_bindgen(js_name = "teamExists")]
    pub fn team_exists(&self, id: usize) -> bool {
        self.inner.team_exists(id)
    }

    /// Returns the teams as a JSON object (BTreeMap<usize, FootballTeam>).
    #[wasm_bindgen(getter)]
    pub fn teams(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.teams())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    // ---------------------------------------------------------------
    // Conference Management
    // ---------------------------------------------------------------

    /// Adds a conference to the season (takes ownership).
    #[wasm_bindgen(js_name = "addConference")]
    pub fn add_conference(
        &mut self,
        conference: WasmLeagueConference,
    ) -> Result<(), JsError> {
        self.inner
            .add_conference(conference.into_inner())
            .map_err(|e| JsError::new(&e))
    }

    /// Returns the conferences as a JSON array.
    #[wasm_bindgen(getter)]
    pub fn conferences(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.conferences())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    // ---------------------------------------------------------------
    // Schedule Generation
    // ---------------------------------------------------------------

    /// Generates the regular season schedule.
    ///
    /// `options` is a plain JS object matching `LeagueSeasonScheduleOptions`
    /// (e.g. `{ division_games: 2, permute: true }`).
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

    // ---------------------------------------------------------------
    // Regular Season Simulation
    // ---------------------------------------------------------------

    /// Simulates the entire season (regular season + playoffs if generated).
    #[wasm_bindgen(js_name = "sim")]
    pub fn sim(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim(rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates all remaining regular season weeks.
    #[wasm_bindgen(js_name = "simRegularSeason")]
    pub fn sim_regular_season(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim_regular_season(rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates all matchups in a single week.
    #[wasm_bindgen(js_name = "simWeek")]
    pub fn sim_week(&mut self, week: usize, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim_week(week, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a single matchup to completion. Returns the game log as JSON.
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

    /// Simulates a single play of a matchup. Returns the game log as JSON
    /// if the game finished on this play, or `undefined` if still in progress.
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
    // Playoff Generation & Simulation
    // ---------------------------------------------------------------

    /// Generates the playoff bracket.
    ///
    /// `options` is a plain JS object matching `LeagueSeasonPlayoffOptions`.
    #[wasm_bindgen(js_name = "generatePlayoffs")]
    pub fn generate_playoffs(
        &mut self,
        options: LeagueSeasonPlayoffOptions,
        rng: &mut WasmRng,
    ) -> Result<(), JsError> {
        self.inner
            .generate_playoffs(options, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Generates the next playoff round (used for multi-round brackets).
    #[wasm_bindgen(js_name = "generateNextPlayoffRound")]
    pub fn generate_next_playoff_round(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .generate_next_playoff_round(rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates all remaining playoffs.
    #[wasm_bindgen(js_name = "simPlayoffs")]
    pub fn sim_playoffs(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        self.inner
            .sim_playoffs(rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a full playoff round across all conference brackets.
    #[wasm_bindgen(js_name = "simPlayoffRound")]
    pub fn sim_playoff_round(
        &mut self,
        round: usize,
        rng: &mut WasmRng,
    ) -> Result<(), JsError> {
        self.inner
            .sim_playoff_round(round, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a full round in a single conference bracket.
    #[wasm_bindgen(js_name = "simPlayoffConferenceRound")]
    pub fn sim_playoff_conference_round(
        &mut self,
        conference: usize,
        round: usize,
        rng: &mut WasmRng,
    ) -> Result<(), JsError> {
        self.inner
            .sim_playoff_conference_round(conference, round, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a single playoff matchup to completion. Returns game log as JSON.
    #[wasm_bindgen(js_name = "simPlayoffMatchup")]
    pub fn sim_playoff_matchup(
        &mut self,
        conference: usize,
        round: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let game = self
            .inner
            .sim_playoff_matchup(conference, round, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Simulates a single play of a playoff matchup. Returns game log as JSON
    /// if the game finished, or `undefined` if still in progress.
    #[wasm_bindgen(js_name = "simPlayoffPlay")]
    pub fn sim_playoff_play(
        &mut self,
        conference: usize,
        round: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let result = self
            .inner
            .sim_playoff_play(conference, round, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        match result {
            Some(game) => {
                serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    /// Simulates a full round of the winners bracket.
    #[wasm_bindgen(js_name = "simWinnersBracketRound")]
    pub fn sim_winners_bracket_round(
        &mut self,
        round: usize,
        rng: &mut WasmRng,
    ) -> Result<(), JsError> {
        self.inner
            .sim_winners_bracket_round(round, rng.inner_mut())
            .map_err(|e| JsError::new(&e))
    }

    /// Simulates a single winners bracket matchup to completion. Returns game log as JSON.
    #[wasm_bindgen(js_name = "simWinnersBracketMatchup")]
    pub fn sim_winners_bracket_matchup(
        &mut self,
        round: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let game = self
            .inner
            .sim_winners_bracket_matchup(round, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Simulates a single play of a winners bracket matchup. Returns game log
    /// as JSON if the game finished, or `undefined` if still in progress.
    #[wasm_bindgen(js_name = "simWinnersBracketPlay")]
    pub fn sim_winners_bracket_play(
        &mut self,
        round: usize,
        matchup: usize,
        rng: &mut WasmRng,
    ) -> Result<JsValue, JsError> {
        let result = self
            .inner
            .sim_winners_bracket_play(round, matchup, rng.inner_mut())
            .map_err(|e| JsError::new(&e))?;
        match result {
            Some(game) => {
                serde_wasm_bindgen::to_value(&game).map_err(|e| JsError::new(&e.to_string()))
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    // ---------------------------------------------------------------
    // Query Methods
    // ---------------------------------------------------------------

    /// Returns the season weeks as a JSON array.
    #[wasm_bindgen(getter)]
    pub fn weeks(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.weeks())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the playoffs as a JSON object.
    #[wasm_bindgen(getter)]
    pub fn playoffs(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self.inner.playoffs())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the overall standings as a JSON array of [teamId, record] pairs.
    #[wasm_bindgen(getter)]
    pub fn standings(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner.standings())
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the standings for a specific division as a JSON array.
    #[wasm_bindgen(js_name = "divisionStandings")]
    pub fn division_standings(
        &self,
        conf_index: usize,
        div_id: usize,
    ) -> Result<JsValue, JsError> {
        let standings = self
            .inner
            .division_standings(conf_index, div_id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&standings).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the standings for a specific conference as a JSON array.
    #[wasm_bindgen(js_name = "conferenceStandings")]
    pub fn conference_standings(&self, conf_index: usize) -> Result<JsValue, JsError> {
        let standings = self
            .inner
            .conference_standings(conf_index)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&standings).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a team's division record as a JSON object.
    #[wasm_bindgen(js_name = "divisionRecord")]
    pub fn division_record(&self, team_id: usize) -> Result<JsValue, JsError> {
        let record = self
            .inner
            .division_record(team_id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&record).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a team's conference record as a JSON object.
    #[wasm_bindgen(js_name = "conferenceRecord")]
    pub fn conference_record(&self, team_id: usize) -> Result<JsValue, JsError> {
        let record = self
            .inner
            .conference_record(team_id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&record).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns all matchups involving a team as a JSON object.
    #[wasm_bindgen(js_name = "teamMatchups")]
    pub fn team_matchups(&self, id: usize) -> Result<JsValue, JsError> {
        let matchups = self
            .inner
            .team_matchups(id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&matchups).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns true if a team participated in the playoffs.
    #[wasm_bindgen(js_name = "teamInPlayoffs")]
    pub fn team_in_playoffs(&self, team_id: usize) -> Result<bool, JsError> {
        self.inner
            .team_in_playoffs(team_id)
            .map_err(|e| JsError::new(&e))
    }

    /// Returns a team's playoff record as a JSON object.
    #[wasm_bindgen(js_name = "playoffRecord")]
    pub fn playoff_record(&self, team_id: usize) -> Result<JsValue, JsError> {
        let record = self
            .inner
            .playoff_record(team_id)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&record).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the playoff picture as a JSON object.
    #[wasm_bindgen(js_name = "playoffPicture")]
    pub fn playoff_picture(&self, num_teams: usize) -> Result<JsValue, JsError> {
        let picture = self
            .inner
            .playoff_picture(num_teams)
            .map_err(|e| JsError::new(&e))?;
        serde_wasm_bindgen::to_value(&picture).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns true if a team made it to the championship.
    #[wasm_bindgen(js_name = "teamInChampionship")]
    pub fn team_in_championship(&self, team_id: usize) -> Result<bool, JsError> {
        self.inner
            .team_in_championship(team_id)
            .map_err(|e| JsError::new(&e))
    }

    /// Returns true if a team won the championship.
    #[wasm_bindgen(js_name = "teamWonChampionship")]
    pub fn team_won_championship(&self, team_id: usize) -> Result<bool, JsError> {
        self.inner
            .team_won_championship(team_id)
            .map_err(|e| JsError::new(&e))
    }

    /// Returns the season as a JSON-serializable object.
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

impl Default for WasmLeagueSeason {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmLeagueSeason {
    /// Returns a reference to the inner `LeagueSeason`.
    pub fn inner(&self) -> &LeagueSeason {
        &self.inner
    }

    /// Returns a mutable reference to the inner `LeagueSeason`.
    pub fn inner_mut(&mut self) -> &mut LeagueSeason {
        &mut self.inner
    }

    /// Consumes the wrapper, returning the inner `LeagueSeason`.
    pub fn into_inner(self) -> LeagueSeason {
        self.inner
    }

    /// Creates a wrapper from an existing `LeagueSeason`.
    pub fn from_inner(inner: LeagueSeason) -> Self {
        WasmLeagueSeason { inner }
    }
}
