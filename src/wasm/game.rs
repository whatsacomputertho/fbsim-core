//! WASM wrappers for game simulation.
//!
//! Provides JavaScript-friendly wrappers for simulating football games
//! play-by-play, drive-by-drive, or all at once.

use wasm_bindgen::prelude::*;

use crate::game::context::GameContext;
use crate::game::play::{Game as CoreGame, GameSimulator};
use crate::wasm::play::{Drive, Game, Play};
use crate::wasm::rng::WasmRng;
use crate::wasm::team::WasmFootballTeam;

/// A WASM-friendly wrapper for game simulation state.
///
/// This struct holds all the state needed to simulate a football game,
/// including both teams, the game context, and the accumulated game log.
/// It provides methods to simulate plays incrementally or all at once.
#[wasm_bindgen(js_name = "GameSession")]
pub struct WasmGameSession {
    home_team: crate::team::FootballTeam,
    away_team: crate::team::FootballTeam,
    context: GameContext,
    game: CoreGame,
    simulator: GameSimulator,
}

#[wasm_bindgen(js_class = "GameSession")]
impl WasmGameSession {
    /// Creates a new game session with the given teams.
    ///
    /// The game will start with a default context (kickoff at start of game).
    ///
    /// # Arguments
    /// * `home_team` - The home team
    /// * `away_team` - The away team
    #[wasm_bindgen(constructor)]
    pub fn new(home_team: &WasmFootballTeam, away_team: &WasmFootballTeam) -> WasmGameSession {
        WasmGameSession {
            home_team: home_team.inner().clone(),
            away_team: away_team.inner().clone(),
            context: GameContext::new(),
            game: CoreGame::new(),
            simulator: GameSimulator::new(),
        }
    }

    /// Creates a new game session with custom team names.
    ///
    /// # Arguments
    /// * `home_team` - The home team
    /// * `away_team` - The away team
    /// * `home_short` - Short name for the home team (max 4 characters)
    /// * `away_short` - Short name for the away team (max 4 characters)
    #[wasm_bindgen(js_name = "withNames")]
    pub fn with_names(
        home_team: &WasmFootballTeam,
        away_team: &WasmFootballTeam,
        home_short: &str,
        away_short: &str,
    ) -> Result<WasmGameSession, JsError> {
        use crate::game::context::GameContextBuilder;

        let context = GameContextBuilder::new()
            .home_team_short(home_short)
            .away_team_short(away_short)
            .build()
            .map_err(|e| JsError::new(&e))?;

        Ok(WasmGameSession {
            home_team: home_team.inner().clone(),
            away_team: away_team.inner().clone(),
            context,
            game: CoreGame::new(),
            simulator: GameSimulator::new(),
        })
    }

    /// Returns true if the game is over.
    #[wasm_bindgen(getter, js_name = "isGameOver")]
    pub fn is_game_over(&self) -> bool {
        self.context.game_over()
    }

    /// Returns the current quarter (1-4, or higher for overtime).
    #[wasm_bindgen(getter)]
    pub fn quarter(&self) -> u32 {
        self.context.quarter()
    }

    /// Returns the seconds remaining in the current half.
    #[wasm_bindgen(getter, js_name = "halfSeconds")]
    pub fn half_seconds(&self) -> u32 {
        self.context.half_seconds()
    }

    /// Returns the current down (1-4, or 0 for kickoff/extra point).
    #[wasm_bindgen(getter)]
    pub fn down(&self) -> u32 {
        self.context.down()
    }

    /// Returns the yards to go for a first down.
    #[wasm_bindgen(getter)]
    pub fn distance(&self) -> u32 {
        self.context.distance()
    }

    /// Returns the current yard line (0-100).
    #[wasm_bindgen(getter, js_name = "yardLine")]
    pub fn yard_line(&self) -> u32 {
        self.context.yard_line()
    }

    /// Returns the home team's score.
    #[wasm_bindgen(getter, js_name = "homeScore")]
    pub fn home_score(&self) -> u32 {
        self.context.home_score()
    }

    /// Returns the away team's score.
    #[wasm_bindgen(getter, js_name = "awayScore")]
    pub fn away_score(&self) -> u32 {
        self.context.away_score()
    }

    /// Returns true if the home team has possession.
    #[wasm_bindgen(getter, js_name = "homePossession")]
    pub fn home_possession(&self) -> bool {
        self.context.home_possession()
    }

    /// Returns the home team's remaining timeouts.
    #[wasm_bindgen(getter, js_name = "homeTimeouts")]
    pub fn home_timeouts(&self) -> u32 {
        self.context.home_timeouts()
    }

    /// Returns the away team's remaining timeouts.
    #[wasm_bindgen(getter, js_name = "awayTimeouts")]
    pub fn away_timeouts(&self) -> u32 {
        self.context.away_timeouts()
    }

    /// Returns true if the next play is a kickoff.
    #[wasm_bindgen(getter, js_name = "nextPlayKickoff")]
    pub fn next_play_kickoff(&self) -> bool {
        self.context.next_play_kickoff()
    }

    /// Returns true if the next play is an extra point attempt.
    #[wasm_bindgen(getter, js_name = "nextPlayExtraPoint")]
    pub fn next_play_extra_point(&self) -> bool {
        self.context.next_play_extra_point()
    }

    /// Returns the number of drives in the game so far.
    #[wasm_bindgen(getter, js_name = "driveCount")]
    pub fn drive_count(&self) -> usize {
        self.game.drives().len()
    }

    /// Returns the total number of plays in the game so far.
    #[wasm_bindgen(getter, js_name = "playCount")]
    pub fn play_count(&self) -> usize {
        self.game.drives().iter().map(|d| d.plays().len()).sum()
    }

    /// Simulates the next play and returns the enriched play result.
    ///
    /// The returned `Play` includes computed values from the `PlayResult` trait
    /// (such as `net_yards`, `turnover`, `offense_score`) and a human-readable
    /// `description` string.
    ///
    /// # Arguments
    /// * `rng` - The random number generator to use
    ///
    /// # Returns
    /// The enriched play that was simulated, or an error if the game is already over.
    #[wasm_bindgen(js_name = "simPlay")]
    pub fn sim_play(&mut self, rng: &mut WasmRng) -> Result<Play, JsError> {
        if self.context.game_over() {
            return Err(JsError::new("Game is already over"));
        }

        let prev_play_count = self.play_count();
        self.context = self
            .simulator
            .sim_play(
                &self.home_team,
                &self.away_team,
                self.context.clone(),
                &mut self.game,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))?;

        // Find and return the newly added play
        let drives = self.game.drives();
        for drive in drives.iter().rev() {
            if let Some(play) = drive.plays().last() {
                if self.play_count() > prev_play_count {
                    return Ok(Play::from(play));
                }
            }
        }

        Err(JsError::new("No play was simulated"))
    }

    /// Simulates until the current drive is complete.
    ///
    /// The returned `Drive` includes enriched `Play` entries with computed
    /// values, total yards, and a human-readable display string.
    ///
    /// # Arguments
    /// * `rng` - The random number generator to use
    ///
    /// # Returns
    /// The enriched completed drive, or an error if the game is already over.
    #[wasm_bindgen(js_name = "simDrive")]
    pub fn sim_drive(&mut self, rng: &mut WasmRng) -> Result<Drive, JsError> {
        if self.context.game_over() {
            return Err(JsError::new("Game is already over"));
        }

        self.context = self
            .simulator
            .sim_drive(
                &self.home_team,
                &self.away_team,
                self.context.clone(),
                &mut self.game,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))?;

        // Return the most recently completed drive
        self.game
            .drives()
            .last()
            .map(Drive::from)
            .ok_or_else(|| JsError::new("No drive was simulated"))
    }

    /// Simulates the rest of the game.
    ///
    /// # Arguments
    /// * `rng` - The random number generator to use
    ///
    /// # Returns
    /// The final game context after all plays have been simulated.
    #[wasm_bindgen(js_name = "simGame")]
    pub fn sim_game(&mut self, rng: &mut WasmRng) -> Result<(), JsError> {
        if self.context.game_over() {
            return Err(JsError::new("Game is already over"));
        }

        self.context = self
            .simulator
            .sim_game(
                &self.home_team,
                &self.away_team,
                self.context.clone(),
                &mut self.game,
                rng.inner_mut(),
            )
            .map_err(|e| JsError::new(&e))?;

        Ok(())
    }

    /// Returns the current game context as a JSON object.
    #[wasm_bindgen(js_name = "getContext")]
    pub fn get_context(&self) -> Result<GameContext, JsError> {
        Ok(self.context.clone())
    }

    /// Returns the full game log with enriched play data.
    #[wasm_bindgen(js_name = "getGame")]
    pub fn get_game(&self) -> Result<Game, JsError> {
        Ok(Game::from(&self.game))
    }

    /// Returns a specific drive by index with enriched play data.
    ///
    /// # Arguments
    /// * `index` - The drive index (0-based)
    #[wasm_bindgen(js_name = "getDrive")]
    pub fn get_drive(&self, index: usize) -> Result<Drive, JsError> {
        self.game
            .drives()
            .get(index)
            .map(Drive::from)
            .ok_or_else(|| JsError::new(&format!("Drive {} not found", index)))
    }

    /// Returns the latest play with enriched data, or null if no plays have been simulated.
    #[wasm_bindgen(js_name = "getLatestPlay")]
    pub fn get_latest_play(&self) -> Option<Play> {
        self.game
            .drives()
            .last()
            .and_then(|d| d.plays().last())
            .map(Play::from)
    }

    /// Returns the home team's offensive stats as a JSON object.
    #[wasm_bindgen(js_name = "getHomeStats")]
    pub fn get_home_stats(&self) -> Result<JsValue, JsError> {
        let stats = self.game.home_stats();
        serde_wasm_bindgen::to_value(&stats).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns the away team's offensive stats as a JSON object.
    #[wasm_bindgen(js_name = "getAwayStats")]
    pub fn get_away_stats(&self) -> Result<JsValue, JsError> {
        let stats = self.game.away_stats();
        serde_wasm_bindgen::to_value(&stats).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Returns a human-readable description of the current game state.
    #[wasm_bindgen(js_name = "getStatusText")]
    pub fn get_status_text(&self) -> String {
        format!("{}", self.context)
    }
}

/// Simulates a complete game and returns the result.
///
/// This is a convenience function for simulating a game without
/// needing to create a session object.
///
/// # Arguments
/// * `home_team` - The home team
/// * `away_team` - The away team
/// * `rng` - The random number generator to use
///
/// # Returns
/// A tuple of (game log, final context) as a JSON object.
#[wasm_bindgen(js_name = "simulateGame")]
pub fn simulate_game(
    home_team: &WasmFootballTeam,
    away_team: &WasmFootballTeam,
    rng: &mut WasmRng,
) -> Result<JsValue, JsError> {
    let context = GameContext::new();
    let simulator = GameSimulator::new();

    let (game, final_context) = simulator
        .sim(home_team.inner(), away_team.inner(), context, rng.inner_mut())
        .map_err(|e| JsError::new(&e))?;

    // Return both game and context as a JS object
    let result = serde_wasm_bindgen::to_value(&serde_json::json!({
        "game": game,
        "context": final_context,
        "homeScore": final_context.home_score(),
        "awayScore": final_context.away_score(),
    }))
    .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(result)
}
