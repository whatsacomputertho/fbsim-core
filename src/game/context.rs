#![doc = include_str!("../../docs/game/context.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};

use crate::game::play::context::PlayContext;
use crate::game::play::result::{ScoreResult, PlayResult};

/// # `GameContextRaw` struct
///
/// A `GameContextRaw` is a `GameContext` before its properties have been
/// validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GameContextRaw {
    home_team_short: String,
    away_team_short: String,
    quarter: u32,
    half_seconds: u32,
    down: u32,
    distance: u32,
    yard_line: u32,
    home_score: u32,
    away_score: u32,
    home_timeouts: u32,
    away_timeouts: u32,
    home_positive_direction: bool,
    home_opening_kickoff: bool,
    home_possession: bool,
    last_play_turnover: bool,
    last_play_incomplete: bool,
    last_play_out_of_bounds: bool,
    last_play_timeout: bool,
    last_play_kickoff: bool,
    last_play_punt: bool,
    next_play_extra_point: bool,
    next_play_kickoff: bool,
    end_of_half: bool,
    game_over: bool
}

impl GameContextRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure each team acronym is no longer than 4 characters
        if self.home_team_short.len() > 4 {
            return Err(
                format!(
                    "Home team short name is longer than 4 characters: {}",
                    self.home_team_short
                )
            )
        }
        if self.away_team_short.len() > 4 {
            return Err(
                format!(
                    "Away team short name is longer than 4 characters: {}",
                    self.away_team_short
                )
            )
        }

        // Ensure half seconds is no greater than 1800 (15 mins)
        if self.half_seconds > 1800 {
            return Err(
                format!(
                    "Half seconds is not in range [0, 1800]: {}",
                    self.half_seconds
                )
            )
        }

        // Ensure half seconds is not less than 900 if quarter is odd and less than 4
        if self.half_seconds < 900 && self.quarter % 2 == 1 && self.quarter < 4 {
            return Err(
                format!(
                    "Half seconds is not in range [900, 1800] for quarter {}: {}",
                    self.quarter,
                    self.half_seconds
                )
            )
        }

        // Ensure half seconds is not greater than 900 if quarter is even or greater than 4
        if self.half_seconds > 900 && (self.quarter.is_multiple_of(2) || self.quarter > 4) {
            return Err(
                format!(
                    "Half seconds is not in range [0, 900] for quarter {}: {}",
                    self.quarter,
                    self.half_seconds
                )
            )
        }

        // Ensure down is no greater than 4
        if self.down > 4 {
            return Err(
                format!(
                    "Down is not in range [0, 4]: {}",
                    self.down
                )
            )
        }

        // Ensure yard line is no greater than 100
        if self.yard_line > 100 {
            return Err(
                format!(
                    "Yard line is not in range [0, 100]: {}",
                    self.yard_line
                )
            )
        }

        // Ensure distance is no greater than the remaining yards
        let remaining_yards: u32 = if self.home_possession ^ self.home_positive_direction {
            self.yard_line
        } else {
            100_u32 - self.yard_line
        };
        if self.distance > remaining_yards {
            return Err(
                format!(
                    "Distance was greater than yards remaining to touchdown: {} > {}",
                    self.distance,
                    remaining_yards
                )
            )
        }

        // Ensure home and away timeouts are no greater than 3
        if self.home_timeouts > 3 {
            return Err(
                format!(
                    "Home timeouts is not in range [0, 3]: {}",
                    self.home_timeouts
                )
            )
        }
        if self.away_timeouts > 3 {
            return Err(
                format!(
                    "Away timeouts is not in range [0, 3]: {}",
                    self.away_timeouts
                )
            )
        }

        // Ensure no invalid last play scenarios
        if self.last_play_incomplete && self.last_play_out_of_bounds {
            return Err(
                String::from("Invalid combination of last play scenarios: Incomplete & out of bounds")
            )
        }
        if self.last_play_kickoff && self.last_play_timeout {
            return Err(
                String::from("Invalid combination of last play scenarios: Kickoff & timeout")
            )
        }
        if self.last_play_punt && self.last_play_timeout {
            return Err(
                String::from("Invalid combination of last play scenarios: Punt & timeout")
            )
        }
        if self.last_play_punt && self.last_play_kickoff {
            return Err(
                String::from("Invalid combination of last play scenarios: Punt & kickoff")
            )
        }

        // Ensure no invalid next play scenarios
        if self.next_play_extra_point && self.next_play_kickoff {
            return Err(
                String::from("Invalid combination of next play scenarios: Kickoff & extra point")
            )
        }

        // Ensure half is not over if quarter is odd and less than 4
        if self.end_of_half && (self.quarter == 1 || (self.quarter == 3 && self.half_seconds < 1800)) {
            return Err(
                format!(
                    "Cannot end half during quarter: {}",
                    self.quarter
                )
            )
        }

        // Ensure half is not over if there is still time left
        if self.end_of_half && self.half_seconds != 1800 && self.half_seconds != 600 && self.half_seconds > 0 {
            return Err(
                format!(
                    "End of half but nonzero half seconds: {}",
                    self.half_seconds
                )
            )
        }

        // Ensure game is not over if quarter is less than 4
        if self.game_over && self.quarter < 4 {
            return Err(
                format!(
                    "Cannot end game during quarter: {}",
                    self.quarter
                )
            )
        }

        // Ensure game is not over if there is still time left
        if self.game_over && self.half_seconds > 0 {
            return Err(
                format!(
                    "End of game but nonzero half seconds: {}",
                    self.half_seconds
                )
            )
        }
        Ok(())
    }
}

/// # `GameContext` struct
///
/// A `GameContext` represents a game scenario
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct GameContext {
    home_team_short: String,
    away_team_short: String,
    quarter: u32,
    half_seconds: u32,
    down: u32,
    distance: u32,
    yard_line: u32,
    home_score: u32,
    away_score: u32,
    home_timeouts: u32,
    away_timeouts: u32,
    home_positive_direction: bool,
    home_opening_kickoff: bool,
    home_possession: bool,
    last_play_turnover: bool,
    last_play_incomplete: bool,
    last_play_out_of_bounds: bool,
    last_play_timeout: bool,
    last_play_kickoff: bool,
    last_play_punt: bool,
    next_play_extra_point: bool,
    next_play_kickoff: bool,
    end_of_half: bool,
    game_over: bool
}

impl Default for GameContext {
    /// Default constructor for the GameContext class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::default();
    /// ```
    fn default() -> Self {
        GameContext {
            home_team_short: String::from("HOME"),
            away_team_short: String::from("AWAY"),
            quarter: 1,
            half_seconds: 1800,
            down: 0,
            distance: 10,
            yard_line: 35,
            home_score: 0,
            away_score: 0,
            home_timeouts: 3,
            away_timeouts: 3,
            home_positive_direction: true,
            home_opening_kickoff: true,
            home_possession: true,
            last_play_turnover: false,
            last_play_incomplete: false,
            last_play_out_of_bounds: false,
            last_play_timeout: false,
            last_play_kickoff: false,
            last_play_punt: false,
            next_play_extra_point: false,
            next_play_kickoff: true,
            end_of_half: false,
            game_over: false
        }
    }
}

impl TryFrom<GameContextRaw> for GameContext {
    type Error = String;

    fn try_from(item: GameContextRaw) -> Result<Self, Self::Error> {
        // Validate the raw game context
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            GameContext{
                home_team_short: item.home_team_short,
                away_team_short: item.away_team_short,
                quarter: item.quarter,
                half_seconds: item.half_seconds,
                down: item.down,
                distance: item.distance,
                yard_line: item.yard_line,
                home_score: item.home_score,
                away_score: item.away_score,
                home_timeouts: item.home_timeouts,
                away_timeouts: item.away_timeouts,
                home_positive_direction: item.home_positive_direction,
                home_opening_kickoff: item.home_opening_kickoff,
                home_possession: item.home_possession,
                last_play_turnover: item.last_play_turnover,
                last_play_incomplete: item.last_play_incomplete,
                last_play_out_of_bounds: item.last_play_out_of_bounds,
                last_play_timeout: item.last_play_timeout,
                last_play_kickoff: item.last_play_kickoff,
                last_play_punt: item.last_play_punt,
                next_play_extra_point: item.next_play_extra_point,
                next_play_kickoff: item.next_play_kickoff,
                end_of_half: item.end_of_half,
                game_over: item.game_over
            }
        )
    }
}

impl<'de> Deserialize<'de> for GameContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = GameContextRaw::deserialize(deserializer)?;
        GameContext::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl GameContext {
    /// Constructor for the GameContext class where properties are defaulted
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// ```
    pub fn new() -> GameContext {
        GameContext::default()
    }

    /// Borrow the GameContext home team short property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_team_short = my_context.home_team_short();
    /// assert!(home_team_short == "HOME");
    /// ```
    pub fn home_team_short(&self) -> &str {
        &self.home_team_short
    }

    /// Borrow the GameContext away team short property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let away_team_short = my_context.away_team_short();
    /// assert!(away_team_short == "AWAY");
    /// ```
    pub fn away_team_short(&self) -> &str {
        &self.away_team_short
    }

    /// Borrow the GameContext quarter property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let quarter = my_context.quarter();
    /// assert!(quarter == 1);
    /// ```
    pub fn quarter(&self) -> u32 {
        self.quarter
    }

    /// Borrow the GameContext half_seconds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let half_seconds = my_context.half_seconds();
    /// assert!(half_seconds == 1800);
    /// ```
    pub fn half_seconds(&self) -> u32 {
        self.half_seconds
    }

    /// Borrow the GameContext down property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let down = my_context.down();
    /// assert!(down == 0);
    /// ```
    pub fn down(&self) -> u32 {
        self.down
    }

    /// Borrow the GameContext distance property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let distance = my_context.distance();
    /// assert!(distance == 10);
    /// ```
    pub fn distance(&self) -> u32 {
        self.distance
    }

    /// Borrow the GameContext yard_line property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let yard_line = my_context.yard_line();
    /// assert!(yard_line == 35);
    /// ```
    pub fn yard_line(&self) -> u32 {
        self.yard_line
    }

    /// Borrow the GameContext home_score property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_score = my_context.home_score();
    /// assert!(home_score == 0);
    /// ```
    pub fn home_score(&self) -> u32 {
        self.home_score
    }

    /// Borrow the GameContext away_score property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let away_score = my_context.away_score();
    /// assert!(away_score == 0);
    /// ```
    pub fn away_score(&self) -> u32 {
        self.away_score
    }

    /// Borrow the GameContext home_timeouts property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_timeouts = my_context.home_timeouts();
    /// assert!(home_timeouts == 3);
    /// ```
    pub fn home_timeouts(&self) -> u32 {
        self.home_timeouts
    }

    /// Borrow the GameContext away_timeouts property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let away_timeouts = my_context.away_timeouts();
    /// assert!(away_timeouts == 3);
    /// ```
    pub fn away_timeouts(&self) -> u32 {
        self.away_timeouts
    }

    /// Borrow the GameContext home_possession property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_possession = my_context.home_possession();
    /// assert!(home_possession);
    /// ```
    pub fn home_possession(&self) -> bool {
        self.home_possession
    }

    /// Borrow the GameContext home_positive_direction property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_positive_direction = my_context.home_positive_direction();
    /// assert!(home_positive_direction);
    /// ```
    pub fn home_positive_direction(&self) -> bool {
        self.home_positive_direction
    }

    /// Borrow the GameContext home_opening_kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let home_opening_kickoff = my_context.home_opening_kickoff();
    /// assert!(home_opening_kickoff);
    /// ```
    pub fn home_opening_kickoff(&self) -> bool {
        self.home_opening_kickoff
    }

    /// Borrow the GameContext last_play_turnover property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_turnover = my_context.last_play_turnover();
    /// assert!(!last_play_turnover);
    /// ```
    pub fn last_play_turnover(&self) -> bool {
        self.last_play_turnover
    }

    /// Borrow the GameContext last_play_incomplete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_incomplete = my_context.last_play_incomplete();
    /// assert!(!last_play_incomplete);
    /// ```
    pub fn last_play_incomplete(&self) -> bool {
        self.last_play_incomplete
    }

    /// Borrow the GameContext last_play_out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_out_of_bounds = my_context.last_play_out_of_bounds();
    /// assert!(!last_play_out_of_bounds);
    /// ```
    pub fn last_play_out_of_bounds(&self) -> bool {
        self.last_play_out_of_bounds
    }

    /// Borrow the GameContext last_play_kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_kickoff = my_context.last_play_kickoff();
    /// assert!(!last_play_kickoff);
    /// ```
    pub fn last_play_kickoff(&self) -> bool {
        self.last_play_kickoff
    }

    /// Borrow the GameContext last_play_punt property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_punt = my_context.last_play_punt();
    /// assert!(!last_play_punt);
    /// ```
    pub fn last_play_punt(&self) -> bool {
        self.last_play_punt
    }

    /// Borrow the GameContext last_play_timeout property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let last_play_timeout = my_context.last_play_timeout();
    /// assert!(!last_play_timeout);
    /// ```
    pub fn last_play_timeout(&self) -> bool {
        self.last_play_timeout
    }

    /// Borrow the GameContext next_play_kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let next_play_kickoff = my_context.next_play_kickoff();
    /// assert!(next_play_kickoff);
    /// ```
    pub fn next_play_kickoff(&self) -> bool {
        self.next_play_kickoff
    }

    /// Borrow the GameContext next_play_extra_point property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let next_play_extra_point = my_context.next_play_extra_point();
    /// assert!(!next_play_extra_point);
    /// ```
    pub fn next_play_extra_point(&self) -> bool {
        self.next_play_extra_point
    }

    /// Borrow the GameContext end_of_half property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let end_of_half = my_context.end_of_half();
    /// assert!(!end_of_half);
    /// ```
    pub fn end_of_half(&self) -> bool {
        self.end_of_half
    }

    /// Borrow the GameContext game_over property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let game_over = my_context.game_over();
    /// assert!(!game_over);
    /// ```
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Get the number of timeouts the defense has left
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let defense_timeouts = my_context.defense_timeouts();
    /// assert!(defense_timeouts == 3);
    /// ```
    pub fn defense_timeouts(&self) -> u32 {
        if self.home_possession {
            self.away_timeouts
        } else {
            self.home_timeouts
        }
    }

    /// Get the number of timeouts the offense has left
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let offense_timeouts = my_context.offense_timeouts();
    /// assert!(offense_timeouts == 3);
    /// ```
    pub fn offense_timeouts(&self) -> u32 {
        if self.home_possession {
            self.home_timeouts
        } else {
            self.away_timeouts
        }
    }

    /// Determine whether the clock is running
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let clock_running = my_context.clock_running();
    /// assert!(!clock_running);
    /// ```
    pub fn clock_running(&self) -> bool {
        !(
            self.last_play_incomplete || self.last_play_timeout || self.next_play_extra_point ||
            self.next_play_kickoff || self.last_play_kickoff || self.last_play_punt || self.last_play_turnover ||
            (
                self.last_play_out_of_bounds && (
                    (self.quarter == 2 && self.half_seconds < 120) ||
                    (self.quarter >= 4 && self.half_seconds < 300)
                )
            )
        )
    }

    /// Get the yards remaining until the defense's goal line
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let yards_to_touchdown = my_context.yards_to_touchdown();
    /// assert!(yards_to_touchdown == 65_i32);
    /// ```
    pub fn yards_to_touchdown(&self) -> i32 {
        if self.home_possession ^ self.home_positive_direction {
            self.yard_line as i32
        } else {
            100_i32 - self.yard_line as i32
        }
    }

    /// Get the yards remaining until the offense's goal line
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let yards_to_safety = my_context.yards_to_safety();
    /// assert!(yards_to_safety == -35_i32);
    /// ```
    pub fn yards_to_safety(&self) -> i32 {
        let safety_yards = if self.home_possession ^ self.home_positive_direction {
            100_i32 - self.yard_line as i32
        } else {
            self.yard_line as i32
        };
        -safety_yards
    }

    /// Get the updated home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let home_score = my_context.next_home_score(ScoreResult::Touchdown, ScoreResult::None);
    /// assert!(home_score == 6);
    /// ```
    pub fn next_home_score(&self, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        if self.home_possession {
            self.home_score + offense_score.points()
        } else {
            self.home_score + defense_score.points()
        }
    }

    /// Get the updated away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let away_score = my_context.next_away_score(ScoreResult::None, ScoreResult::Safety);
    /// assert!(away_score == 2);
    /// ```
    pub fn next_away_score(&self, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        if self.home_possession {
            self.away_score + defense_score.points()
        } else {
            self.away_score + offense_score.points()
        }
    }

    /// Determine whether the score is tied given the scoring results from the last play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let score_tied = my_context.next_score_tied(ScoreResult::None, ScoreResult::None);
    /// assert!(score_tied);
    /// ```
    pub fn next_score_tied(&self, offense_score: ScoreResult, defense_score: ScoreResult) -> bool {
        let next_home_score = self.next_home_score(offense_score, defense_score);
        let next_away_score = self.next_away_score(offense_score, defense_score);
        next_home_score == next_away_score
    }

    /// Get the updated half seconds
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let half_seconds = my_context.next_half_seconds(10, false, false, ScoreResult::None, ScoreResult::None);
    /// assert!(half_seconds == 1790);
    /// ```
    pub fn next_half_seconds(&self, play_duration: u32, end_of_half: bool, post_play_end_of_half: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        let next_clock = u32::try_from(self.half_seconds as i32 - play_duration as i32).unwrap_or_default();

        // If end of quarter, max out at 900 seconds
        if (self.quarter == 1 || self.quarter == 3) && self.half_seconds > 900 && next_clock <= 900 {
            return 900;
        }

        // If end of half, return 0 seconds
        if end_of_half {
            return 0;
        }

        // If start of second half, return to 1800 seconds
        if (post_play_end_of_half && self.quarter < 4) || (self.end_of_half && self.quarter == 2) {
            return 1800;
        }

        // Check if end of game
        if self.quarter >= 4 && next_clock == 0 {
            if !self.next_score_tied(offense_score, defense_score) {
                // If end of game, max out at 0 seconds
                return 0;
            } else {
                // If overtime, return to 600 seconds
                return 600;
            }
        }
        next_clock
    }

    /// Get the updated end of half property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let end_of_half = my_context.next_end_of_half(10, ScoreResult::None, ScoreResult::None);
    /// assert!(!end_of_half);
    /// ```
    pub fn next_end_of_half(&self, play_duration: u32, offense_score: ScoreResult, defense_score: ScoreResult) -> bool {
        let next_clock = u32::try_from(self.half_seconds as i32 - play_duration as i32).unwrap_or_default();

        // Check if end of half
        if next_clock == 0 && (self.quarter == 2 || self.quarter >=4) &&
            !(offense_score == ScoreResult::Touchdown || defense_score == ScoreResult::Touchdown) {
            return true;
        }
        false
    }

    /// Get the updated game over property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let game_over = my_context.next_game_over(10, ScoreResult::None, ScoreResult::None);
    /// assert!(!game_over);
    /// ```
    pub fn next_game_over(&self, play_duration: u32, offense_score: ScoreResult, defense_score: ScoreResult) -> bool {
        let next_clock = u32::try_from(self.half_seconds as i32 - play_duration as i32).unwrap_or_default();
        self.quarter >= 4 && next_clock == 0 && !self.next_score_tied(offense_score, defense_score)
    }

    /// Get the updated quarter
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let quarter = my_context.next_quarter(10, ScoreResult::None, ScoreResult::None);
    /// assert!(quarter == 1);
    /// ```
    pub fn next_quarter(&self, play_duration: u32, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        let next_clock = u32::try_from(self.half_seconds as i32 - play_duration as i32).unwrap_or_default();

        // Don't increment quarter if extra point still needs to be kicked
        if offense_score == ScoreResult::Touchdown || defense_score == ScoreResult::Touchdown {
            return self.quarter
        }

        // If end of 1st - 3rd quarter, increment quarter regardless
        // If end of 4th - OT, increment quarter only if tied
        if ((self.quarter == 1 || self.quarter == 3) && self.half_seconds >= 900 && next_clock <= 900) ||
            (self.quarter == 2 && next_clock == 0) ||
            (self.quarter >= 4 && next_clock == 0 && self.next_score_tied(offense_score, defense_score)) {
            return self.quarter + 1;
        }
        self.quarter
    }

    /// Get the updated home team direction
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let next_home_positive_direction = my_context.next_home_positive_direction(10, false, ScoreResult::None, ScoreResult::None);
    /// assert!(next_home_positive_direction);
    /// ```
    pub fn next_home_positive_direction(&self, play_duration: u32, end_of_half: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> bool {
        let qtr = self.next_quarter(play_duration, offense_score, defense_score);

        // Flip the field if end of quarter
        let home_dir = self.home_positive_direction;
        if self.quarter != qtr || end_of_half {
            return !home_dir;
        }
        home_dir
    }

    /// Get the updated down
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let down = my_context.next_down(10, true, false, ScoreResult::None, ScoreResult::None);
    /// assert!(down == 1);
    /// ```
    pub fn next_down(&self, net_yards: i32, turnover: bool, end_of_half: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        // If this is the end of the half, next play is a kickoff
        if end_of_half {
            return 0;
        }

        // If the result was for an extra point or 2 point conversion, next play is always a kickoff
        if self.next_play_extra_point {
            return 0;
        }

        // If the result was for a kickoff, check if a score occurred
        if self.next_play_kickoff {
            if !(offense_score == ScoreResult::None && defense_score == ScoreResult::None) {
                return 0;
            }
            return 1;
        }

        // If a touchdown, safety, or field goal occurred then next play is a down-0 play
        let off_zero_down = matches!(
            offense_score,
            ScoreResult::Touchdown | ScoreResult::FieldGoal | ScoreResult::Safety
        );
        let def_zero_down = matches!(
            defense_score,
            ScoreResult::Touchdown | ScoreResult::FieldGoal | ScoreResult::Safety
        );
        if off_zero_down || def_zero_down {
            return 0;
        }

        // If a turnover occurred then next play is first down
        if turnover {
            return 1;
        }

        // Check if a first down was reached
        if net_yards >= self.distance as i32 {
            return 1;
        }

        // Increment the down and check for a turnover on downs
        let next_down = self.down + 1;
        if next_down > 4 {
            return 1;
        }
        next_down
    }

    /// Get the updated home possession property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let next_home_possession = my_context.next_home_possession(10_i32, false, false, ScoreResult::None, ScoreResult::None);
    /// assert!(!next_home_possession);
    /// ```
    pub fn next_home_possession(&self, net_yards: i32, turnover: bool, end_of_half: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> bool {
        // If end of half, possession goes to whomever received the opening kickoff
        if end_of_half {
            return self.home_opening_kickoff;
        }

        // Change possession on successful kickoffs, defensive TDs, turnovers
        if self.next_play_kickoff || defense_score == ScoreResult::Touchdown || turnover {
            return !self.home_possession;
        }

        // Maintain possession on first downs, offensive scores
        if net_yards >= self.distance as i32 ||
            offense_score == ScoreResult::Touchdown ||
            offense_score == ScoreResult::FieldGoal ||
            offense_score == ScoreResult::ExtraPoint ||
            offense_score == ScoreResult::TwoPointConversion {
            return self.home_possession;
        }

        // Change possession on turnovers on downs
        let next_down = self.down + 1;
        if next_down > 4 {
            return !self.home_possession;
        }
        self.home_possession
    }

    /// Get the updated yard line
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let yard_line = my_context.next_yard_line(0, 10, false, false, false, ScoreResult::None, ScoreResult::None);
    /// assert!(yard_line == 45);
    /// ```
    pub fn next_yard_line(&self, play_duration: u32, net_yards: i32, touchback: bool, kickoff_oob: bool, end_of_half: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        // Kickoff and flip the field at the end of the half
        if end_of_half {
            if self.home_opening_kickoff ^ self.home_positive_direction {
                return 35;
            }
            return 65;
        }

        // Kickoff after PAT, field goals, safeties
        let qtr = self.next_quarter(play_duration, offense_score, defense_score);
        let end_of_quarter = qtr != self.quarter;
        if self.next_play_extra_point || defense_score == ScoreResult::Safety || offense_score == ScoreResult::FieldGoal {
            let next_yl = if self.home_possession ^ self.home_positive_direction {
                65
            } else {
                35
            };
            let eoq_yl = if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            };
            return eoq_yl;
        }

        // Extra point after touchdowns
        if offense_score == ScoreResult::Touchdown {
            let next_yl = if self.home_possession ^ self.home_positive_direction {
                2
            } else {
                98
            };
            let eoq_yl = if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            };
            return eoq_yl;
        } else if defense_score == ScoreResult::Touchdown {
            let next_yl = if self.home_possession ^ self.home_positive_direction {
                98
            } else {
                2
            };
            let eoq_yl = if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            };
            return eoq_yl;
        }

        // Touchbacks and kickoffs out of bounds
        if touchback {
            let next_yl = if self.home_possession ^ self.home_positive_direction {
                25
            } else {
                75
            };
            let eoq_yl = if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            };
            return eoq_yl;
        } else if kickoff_oob {
            let next_yl = if self.home_possession ^ self.home_positive_direction {
                35
            } else {
                65
            };
            let eoq_yl = if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            };
            return eoq_yl;
        }

        // Increment the yard line
        if self.home_possession ^ self.home_positive_direction {
            let next_yl = u32::try_from(0.max(100.min(self.yard_line as i32 - net_yards))).unwrap_or_default();
            if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            }
        } else {
            let next_yl = u32::try_from(0.max(100.min(self.yard_line as i32 + net_yards))).unwrap_or_default();
            if end_of_quarter {
                100 - next_yl
            } else {
                next_yl
            }
        }
    }

    /// Get the updated distance
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::ScoreResult;
    /// 
    /// let my_context = GameContext::new();
    /// let distance = my_context.next_distance(0, 10, true, false, false, false, false, ScoreResult::None, ScoreResult::None);
    /// assert!(distance == 10);
    /// ```
    pub fn next_distance(&self, play_duration: u32, net_yards: i32, turnover: bool, touchback: bool, kickoff_oob: bool, end_of_half: bool, between_play: bool, offense_score: ScoreResult, defense_score: ScoreResult) -> u32 {
        // Kickoff after PAT, field goals, safeties, end of half
        if self.next_play_extra_point || end_of_half || defense_score == ScoreResult::Safety || offense_score == ScoreResult::FieldGoal {
            return 10;
        }

        // Extra point after touchdowns
        if offense_score == ScoreResult::Touchdown || defense_score == ScoreResult::Touchdown {
            return 2;
        }

        // If a turnover occurred, determine the distance based on the defense's direction
        // Note it will always be a first down after a turnover
        let qtr = self.next_quarter(play_duration, offense_score, defense_score);
        let end_of_quarter = qtr != self.quarter;
        let mut next_yl = self.next_yard_line(play_duration, net_yards, touchback, kickoff_oob, false, offense_score, defense_score);
        next_yl = if end_of_quarter {
            100 - next_yl
        } else {
            next_yl
        };
        if turnover || (self.next_play_kickoff && !between_play) {
            if self.home_possession ^ self.home_positive_direction {
                return 0.max(10.min(100_i32 - next_yl as i32)) as u32;
            }
            return 10.min(next_yl);
        }

        // If no turnover occurred, check for a first down
        if net_yards >= self.distance as i32 {
            if self.home_possession ^ self.home_positive_direction {
                return 10.min(next_yl);
            }
            return 0.max(10.min(100_i32 - next_yl as i32)) as u32;
        } else if self.down == 4 && !between_play {
            if self.home_possession ^ self.home_positive_direction {
                return 0.max(10.min(100_i32 - next_yl as i32)) as u32;
            }
            return 10.min(next_yl);
        }
        let next_dist = self.distance as i32 - net_yards;
        u32::try_from(next_dist).unwrap_or_default()
    }

    /// Get the updated home timetous
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let next_home_timeouts = my_context.next_home_timeouts(true, false);
    /// assert!(next_home_timeouts == 2);
    /// ```
    pub fn next_home_timeouts(&self, offense_timeout: bool, defense_timeout: bool) -> u32 {
        if self.end_of_half {
            return 3; // Reset at end of half
        }
        let home_tos = self.home_timeouts;
        if self.home_possession {
            if offense_timeout {
                return 0.max(home_tos as i32 - 1_i32) as u32;
            }
            return home_tos;
        }
        if defense_timeout {
            return 0.max(home_tos as i32 - 1_i32) as u32;
        }
        home_tos
    }

    /// Get the updated away timetous
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// let my_context = GameContext::new();
    /// let next_away_timeouts = my_context.next_away_timeouts(false, true);
    /// assert!(next_away_timeouts == 2);
    /// ```
    pub fn next_away_timeouts(&self, offense_timeout: bool, defense_timeout: bool) -> u32 {
        if self.end_of_half {
            return 3; // Reset at end of half
        }
        let away_tos = self.away_timeouts;
        if self.home_possession && defense_timeout {
            return 0.max(away_tos as i32 - 1_i32) as u32;
        }
        if offense_timeout {
            return 0.max(away_tos as i32 - 1_i32) as u32;
        }
        away_tos
    }

    /// Get the next context given the results of the previous play
    pub fn next_context(&self, result: &(impl PlayResult + ?Sized)) -> GameContext {
        let duration = result.play_duration();
        let net_yards = result.net_yards();
        let off_score = result.offense_score();
        let def_score = result.defense_score();
        let turnover = result.turnover();
        let touchback = result.touchback();
        let kickoff_oob = result.kickoff() && result.out_of_bounds();
        let off_timeout = result.offense_timeout();
        let def_timeout = result.defense_timeout();
        let next_play_extra_point = result.next_play_extra_point();
        let end_of_half = if self.end_of_half {
            false
        } else {
            self.next_end_of_half(duration, off_score, def_score) && !(next_play_extra_point)
        };
        let next_quarter = if end_of_half {
            self.quarter()
        } else {
            self.next_quarter(duration, off_score, def_score)
        };
        let raw = GameContextRaw{
            home_team_short: self.home_team_short.clone(),
            away_team_short: self.away_team_short.clone(),
            quarter: next_quarter,
            half_seconds: self.next_half_seconds(duration, end_of_half, false, off_score, def_score),
            down: self.next_down(net_yards, turnover, false, off_score, def_score),
            distance: self.next_distance(duration, net_yards, turnover, touchback, kickoff_oob, end_of_half, false, off_score, def_score),
            yard_line: self.next_yard_line(duration, net_yards, touchback, kickoff_oob, end_of_half, off_score, def_score),
            home_score: self.next_home_score(off_score, def_score),
            away_score: self.next_away_score(off_score, def_score),
            home_timeouts: self.next_home_timeouts(off_timeout, def_timeout),
            away_timeouts: self.next_away_timeouts(off_timeout, def_timeout),
            home_positive_direction: self.next_home_positive_direction(duration, end_of_half, off_score, def_score),
            home_opening_kickoff: self.home_opening_kickoff,
            home_possession: self.next_home_possession(net_yards, turnover, end_of_half, off_score, def_score),
            last_play_turnover: turnover,
            last_play_incomplete: result.incomplete(),
            last_play_out_of_bounds: result.out_of_bounds(),
            last_play_timeout: off_timeout || def_timeout,
            last_play_kickoff: result.kickoff(),
            last_play_punt: result.punt(),
            next_play_extra_point,
            next_play_kickoff: result.next_play_kickoff() || (end_of_half && !next_play_extra_point),
            end_of_half,
            game_over: self.next_game_over(duration, off_score, def_score)
        };
        GameContext::try_from(raw).unwrap()
    }
}

impl std::fmt::Display for GameContext {
    /// Format a `GameContext` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::context::GameContext;
    ///
    /// // Initialize a game context and display it
    /// let my_context = GameContext::new();
    /// println!("{}", my_context);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let play_context = PlayContext::from(self);
        let (home_team_str, away_team_str) = if self.home_possession {
            (format!("*{}", &self.home_team_short), String::from(&self.away_team_short))
        } else {
            (String::from(&self.home_team_short), format!("*{}", &self.away_team_short))
        };
        let context_str = format!(
            "{} ({} {} - {} {})",
            &play_context,
            &home_team_str,
            self.home_score,
            &away_team_str,
            self.away_score
        );
        f.write_str(&context_str)
    }
}

/// # `GameContextBuilder` struct
///
/// A `GameContextBuilder` implements the builder pattern for the `GameContext`
/// struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct GameContextBuilder {
    home_team_short: String,
    away_team_short: String,
    quarter: u32,
    half_seconds: u32,
    down: u32,
    distance: u32,
    yard_line: u32,
    home_score: u32,
    away_score: u32,
    home_timeouts: u32,
    away_timeouts: u32,
    home_positive_direction: bool,
    home_opening_kickoff: bool,
    home_possession: bool,
    last_play_turnover: bool,
    last_play_incomplete: bool,
    last_play_out_of_bounds: bool,
    last_play_timeout: bool,
    last_play_kickoff: bool,
    last_play_punt: bool,
    next_play_extra_point: bool,
    next_play_kickoff: bool,
    end_of_half: bool,
    game_over: bool
}

impl Default for GameContextBuilder {
    /// Default constructor for the GameContextBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContextBuilder;
    /// 
    /// let my_context = GameContextBuilder::default();
    /// ```
    fn default() -> Self {
        GameContextBuilder {
            home_team_short: String::from("HOME"),
            away_team_short: String::from("AWAY"),
            quarter: 1,
            half_seconds: 1800,
            down: 0,
            distance: 10,
            yard_line: 35,
            home_score: 0,
            away_score: 0,
            home_timeouts: 3,
            away_timeouts: 3,
            home_positive_direction: true,
            home_opening_kickoff: true,
            home_possession: true,
            last_play_turnover: false,
            last_play_incomplete: false,
            last_play_out_of_bounds: false,
            last_play_timeout: false,
            last_play_kickoff: false,
            last_play_punt: false,
            next_play_extra_point: false,
            next_play_kickoff: true,
            end_of_half: false,
            game_over: false
        }
    }
}

impl GameContextBuilder {
    /// Initialize a new game context builder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContextBuilder;
    ///
    /// let mut my_context_builder = GameContextBuilder::new();
    /// ```
    pub fn new() -> GameContextBuilder {
        GameContextBuilder::default()
    }

    /// Set the home team short name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_team_short("TEST")
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_team_short() == "TEST");
    /// ```
    pub fn home_team_short(mut self, home_team_short: &str) -> Self {
        self.home_team_short = String::from(home_team_short);
        self
    }

    /// Set the away team short name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .away_team_short("TEST")
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.away_team_short() == "TEST");
    /// ```
    pub fn away_team_short(mut self, away_team_short: &str) -> Self {
        self.away_team_short = String::from(away_team_short);
        self
    }

    /// Set the quarter
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .quarter(2)
    ///     .half_seconds(800)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.quarter() == 2);
    /// ```
    pub fn quarter(mut self, quarter: u32) -> Self {
        self.quarter = quarter;
        self
    }

    /// Set the half seconds
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .half_seconds(100)
    ///     .quarter(4)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.half_seconds() == 100);
    /// ```
    pub fn half_seconds(mut self, half_seconds: u32) -> Self {
        self.half_seconds = half_seconds;
        self
    }

    /// Set the down
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .down(4)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.down() == 4);
    /// ```
    pub fn down(mut self, down: u32) -> Self {
        self.down = down;
        self
    }

    /// Set the distance
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .distance(7)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.distance() == 7);
    /// ```
    pub fn distance(mut self, distance: u32) -> Self {
        self.distance = distance;
        self
    }

    /// Set the yard line
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .yard_line(50)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.yard_line() == 50);
    /// ```
    pub fn yard_line(mut self, yard_line: u32) -> Self {
        self.yard_line = yard_line;
        self
    }

    /// Set the home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_score(21)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_score() == 21);
    /// ```
    pub fn home_score(mut self, home_score: u32) -> Self {
        self.home_score = home_score;
        self
    }

    /// Set the away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .away_score(14)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.away_score() == 14);
    /// ```
    pub fn away_score(mut self, away_score: u32) -> Self {
        self.away_score = away_score;
        self
    }

    /// Set the home timeouts
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_timeouts(2)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_timeouts() == 2);
    /// ```
    pub fn home_timeouts(mut self, home_timeouts: u32) -> Self {
        self.home_timeouts = home_timeouts;
        self
    }

    /// Set the away timeouts
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .away_timeouts(2)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.away_timeouts() == 2);
    /// ```
    pub fn away_timeouts(mut self, away_timeouts: u32) -> Self {
        self.away_timeouts = away_timeouts;
        self
    }
    
    /// Set the home positive direction property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_positive_direction(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_positive_direction() == false);
    /// ```
    pub fn home_positive_direction(mut self, home_positive_direction: bool) -> Self {
        self.home_positive_direction = home_positive_direction;
        self
    }
    
    /// Set the home opening kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_opening_kickoff(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_opening_kickoff() == false);
    /// ```
    pub fn home_opening_kickoff(mut self, home_opening_kickoff: bool) -> Self {
        self.home_opening_kickoff = home_opening_kickoff;
        self
    }
    
    /// Set the home opening kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_possession(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.home_possession() == false);
    /// ```
    pub fn home_possession(mut self, home_possession: bool) -> Self {
        self.home_possession = home_possession;
        self
    }
    
    /// Set the last play turnover property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_turnover(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_turnover() == true);
    /// ```
    pub fn last_play_turnover(mut self, last_play_turnover: bool) -> Self {
        self.last_play_turnover = last_play_turnover;
        self
    }
    
    /// Set the last play incomplete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_incomplete(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_incomplete() == true);
    /// ```
    pub fn last_play_incomplete(mut self, last_play_incomplete: bool) -> Self {
        self.last_play_incomplete = last_play_incomplete;
        self
    }
    
    /// Set the last play out of bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_out_of_bounds(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_out_of_bounds() == true);
    /// ```
    pub fn last_play_out_of_bounds(mut self, last_play_out_of_bounds: bool) -> Self {
        self.last_play_out_of_bounds = last_play_out_of_bounds;
        self
    }
    
    /// Set the last play timeout property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_timeout(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_timeout() == true);
    /// ```
    pub fn last_play_timeout(mut self, last_play_timeout: bool) -> Self {
        self.last_play_timeout = last_play_timeout;
        self
    }
    
    /// Set the last play kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_kickoff(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_kickoff() == true);
    /// ```
    pub fn last_play_kickoff(mut self, last_play_kickoff: bool) -> Self {
        self.last_play_kickoff = last_play_kickoff;
        self
    }
    
    /// Set the last play punt property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .last_play_punt(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.last_play_punt() == true);
    /// ```
    pub fn last_play_punt(mut self, last_play_punt: bool) -> Self {
        self.last_play_punt = last_play_punt;
        self
    }
    
    /// Set the next play extra point property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .next_play_kickoff(false)
    ///     .next_play_extra_point(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.next_play_extra_point() == true);
    /// ```
    pub fn next_play_extra_point(mut self, next_play_extra_point: bool) -> Self {
        self.next_play_extra_point = next_play_extra_point;
        self
    }
    
    /// Set the next play kickoff property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .next_play_kickoff(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.next_play_kickoff() == false);
    /// ```
    pub fn next_play_kickoff(mut self, next_play_kickoff: bool) -> Self {
        self.next_play_kickoff = next_play_kickoff;
        self
    }
    
    /// Set the end of half property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .half_seconds(0)
    ///     .quarter(4)
    ///     .end_of_half(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.end_of_half() == true);
    /// ```
    pub fn end_of_half(mut self, end_of_half: bool) -> Self {
        self.end_of_half = end_of_half;
        self
    }
    
    /// Set the game over property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .half_seconds(0)
    ///     .quarter(4)
    ///     .game_over(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_context.game_over() == true);
    /// ```
    pub fn game_over(mut self, game_over: bool) -> Self {
        self.game_over = game_over;
        self
    }

    /// Build the game context
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::{GameContext, GameContextBuilder};
    /// 
    /// let my_context = GameContextBuilder::new()
    ///     .home_team_short("NYM")
    ///     .away_team_short("CAR")
    ///     .quarter(2)
    ///     .half_seconds(700)
    ///     .down(3)
    ///     .distance(4)
    ///     .yard_line(14)
    ///     .home_score(0)
    ///     .away_score(3)
    ///     .home_timeouts(2)
    ///     .away_timeouts(3)
    ///     .home_positive_direction(true)
    ///     .home_possession(true)
    ///     .last_play_incomplete(true)
    ///     .last_play_out_of_bounds(false)
    ///     .last_play_timeout(false)
    ///     .last_play_punt(false)
    ///     .last_play_kickoff(false)
    ///     .next_play_extra_point(false)
    ///     .next_play_kickoff(false)
    ///     .game_over(false)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<GameContext, String> {
        let raw = GameContextRaw{
            home_team_short: self.home_team_short,
            away_team_short: self.away_team_short,
            quarter: self.quarter,
            half_seconds: self.half_seconds,
            down: self.down,
            distance: self.distance,
            yard_line: self.yard_line,
            home_score: self.home_score,
            away_score: self.away_score,
            home_timeouts: self.home_timeouts,
            away_timeouts: self.away_timeouts,
            home_positive_direction: self.home_positive_direction,
            home_opening_kickoff: self.home_opening_kickoff,
            home_possession: self.home_possession,
            last_play_turnover: self.last_play_turnover,
            last_play_incomplete: self.last_play_incomplete,
            last_play_out_of_bounds: self.last_play_out_of_bounds,
            last_play_timeout: self.last_play_timeout,
            last_play_kickoff: self.last_play_kickoff,
            last_play_punt: self.last_play_punt,
            next_play_extra_point: self.next_play_extra_point,
            next_play_kickoff: self.next_play_kickoff,
            end_of_half: self.end_of_half,
            game_over: self.game_over
        };
        GameContext::try_from(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::play::result::kickoff::{KickoffResult, KickoffResultBuilder};

    #[test]
    fn test_long_kickoff_return_fumble_result() {
        // Create a new context
        let context: GameContext = GameContext::new();

        // Create a kickoff return result in which the return team returns
        // 60+ yards and then fumbles
        let kickoff_return: KickoffResult = KickoffResultBuilder::new()
            .kickoff_yards(49)
            .kick_return_yards(67)
            .play_duration(10)
            .fumble_return_yards(3)
            .touchback(false)
            .out_of_bounds(false)
            .fair_catch(false)
            .fumble(true)
            .touchdown(false)
            .build()
            .unwrap();

        // Get the next context
        let next_context: GameContext = kickoff_return.next_context(&context);

        // Assert the next distance is 10
        assert!(next_context.distance() == 10);
    }
}
