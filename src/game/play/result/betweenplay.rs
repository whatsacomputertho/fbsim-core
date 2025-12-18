use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use rand_distr::{SkewNormal, Normal, Distribution, Exp};

use crate::game::context::{GameContext, GameContextBuilder};
use crate::game::play::PlaySimulatable;
use crate::game::play::context::PlayContext;
use crate::game::play::result::{PlayResult, PlayTypeResult, PlayResultSimulator, ScoreResult};

// Up-tempo probability regression
const P_UP_TEMPO_INTR: f64 = -4.5395125211354683_f64; // Adjusted -1
const P_UP_TEMPO_COEF: f64 = 3.03267023_f64;

// Normal between-play duration distribution parameters
const MEAN_BETWEEN_PLAY_DURATION: f64 = 27_f64; // Adjusted + 7
const STD_BETWEEN_PLAY_DURATION: f64 = 5_f64;
const SKEW_BETWEEN_PLAY_DURATION: f64 = 7_f64; // Added skew

// Up-tempo between-play duration distribution parameters
const MEAN_UP_TEMPO_BETWEEN_PLAY_DURATION: f64 = 6_f64;
const STD_UP_TEMPO_BETWEEN_PLAY_DURATION: f64 = 2_f64;

// Probability defense is not set
const P_DEFENSE_NOT_SET_CLOCK_STOPPED: f64 = 0.001_f64;
const P_DEFENSE_NOT_SET: f64 = 0.08_f64;
const P_DEFENSE_NOT_SET_UP_TEMPO: f64 = 0.3_f64;

// Probability a coach calls a timeout when the defense is not set
// Also probability a coach calls a timeout on a critical down to get set
const P_GET_SET_TIMEOUT_INTR: f64 = 0.2_f64;
const P_GET_SET_TIMEOUT_COEF: f64 = 0.4_f64;

/// # `BetweenPlayResult` struct
///
/// A `BetweenPlayResult` represents any activity between plays, such as the
/// clock running in-between plays & timeouts called after the play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct BetweenPlayResult {
    duration: u32,
    offense_timeout: bool,
    defense_timeout: bool,
    up_tempo: bool,
    defense_not_set: bool,
    critical_down: bool
}

impl Default for BetweenPlayResult {
    /// Default constructor for the BetweenPlayResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_result = BetweenPlayResult::default();
    /// ```
    fn default() -> Self {
        BetweenPlayResult{
            duration: 20,
            offense_timeout: false,
            defense_timeout: false,
            up_tempo: false,
            defense_not_set: false,
            critical_down: false
        }
    }
}

impl std::fmt::Display for BetweenPlayResult {
    /// Format a `BetweenPlayResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_result = BetweenPlayResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timeout_str = if self.offense_timeout {
            "Offense calls timeout after the play."
        } else if self.defense_timeout {
            if self.defense_not_set {
                "Defense slow to get set, calls timeout to get set."
            } else if self.critical_down {
                "Defense calls timeout to make a playcall."
            } else {
                "Defense calls timeout."
            }
        } else {
            ""
        };
        let up_tempo_str = if self.up_tempo {
            "Offense rushes to the line."
        } else {
            ""
        };
        let result_str = format!("{} {}", up_tempo_str, timeout_str);
        f.write_str(&result_str.trim())
    }
}

impl PlayResult for BetweenPlayResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        let off_score = ScoreResult::None;
        let def_score = ScoreResult::None;
        let end_of_half = context.next_end_of_half(self.duration, off_score, def_score);
        GameContextBuilder::new()
            .home_team_short(context.home_team_short())
            .away_team_short(context.away_team_short())
            .quarter(context.next_quarter(self.duration, off_score, def_score))
            .half_seconds(context.next_half_seconds(self.duration, off_score, def_score))
            .down(*context.down())
            .distance(*context.distance())
            .yard_line(*context.yard_line())
            .home_score(*context.home_score())
            .away_score(*context.away_score())
            .home_timeouts(context.next_home_timeouts(self.offense_timeout, self.defense_timeout, end_of_half))
            .away_timeouts(context.next_away_timeouts(self.offense_timeout, self.defense_timeout, end_of_half))
            .home_positive_direction(*context.home_positive_direction())
            .home_opening_kickoff(*context.home_opening_kickoff())
            .home_possession(*context.home_possession())
            .last_play_turnover(*context.last_play_turnover())
            .last_play_incomplete(*context.last_play_incomplete())
            .last_play_out_of_bounds(*context.last_play_out_of_bounds())
            .last_play_timeout(self.offense_timeout || self.defense_timeout)
            .last_play_kickoff(*context.last_play_kickoff())
            .next_play_extra_point(*context.next_play_extra_point())
            .next_play_kickoff(*context.next_play_kickoff())
            .end_of_half(end_of_half)
            .game_over(context.next_game_over(self.duration, off_score, def_score))
            .build()
    }
}

impl BetweenPlayResult {
    /// Initialize a new between play result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// ```
    pub fn new() -> BetweenPlayResult {
        BetweenPlayResult::default()
    }

    /// Get a between play result's duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let duration = my_res.duration();
    /// ```
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Get a between play result's offense_timeout property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let offense_timeout = my_res.offense_timeout();
    /// ```
    pub fn offense_timeout(&self) -> bool {
        self.offense_timeout
    }

    /// Get a between play result's defense_timeout property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let defense_timeout = my_res.defense_timeout();
    /// ```
    pub fn defense_timeout(&self) -> bool {
        self.defense_timeout
    }

    /// Get a between play result's up_tempo property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let up_tempo = my_res.up_tempo();
    /// ```
    pub fn up_tempo(&self) -> bool {
        self.up_tempo
    }

    /// Get a between play result's defense_not_set property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let defense_not_set = my_res.defense_not_set();
    /// ```
    pub fn defense_not_set(&self) -> bool {
        self.defense_not_set
    }

    /// Get a between play result's critical_down property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// 
    /// let my_res = BetweenPlayResult::new();
    /// let critical_down = my_res.critical_down();
    /// ```
    pub fn critical_down(&self) -> bool {
        self.critical_down
    }
}

/// # `BetweenPlayResultSimulator` struct
///
/// A `BetweenPlayResultSimulator` simulates the events that occur between plays
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct BetweenPlayResultSimulator {}

impl BetweenPlayResultSimulator {
    /// Initialize a new BetweenPlayResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResultSimulator;
    ///
    /// let my_sim = BetweenPlayResultSimulator::new();
    /// ```
    pub fn new() -> BetweenPlayResultSimulator {
        BetweenPlayResultSimulator{}
    }

    /// Generates whether the offense goes up-tempo
    fn up_tempo(&self, context: &PlayContext, norm_up_tempo: f64, rng: &mut impl Rng) -> bool {
        if context.up_tempo() {
            return true;
        }
        let p_up_tempo: f64 = 1_f64.min(0_f64.max(
            (P_UP_TEMPO_INTR + (P_UP_TEMPO_COEF * norm_up_tempo)).exp()
        ));
        rng.gen::<f64>() < p_up_tempo
    }

    /// Generates whether the defense is not set
    fn defense_not_set(&self, up_tempo: bool, clock_running: bool, rng: &mut impl Rng) -> bool {
        let p_not_set: f64 = if clock_running {
            if up_tempo {
                P_DEFENSE_NOT_SET_UP_TEMPO
            } else {
                P_DEFENSE_NOT_SET
            }
        } else {
            P_DEFENSE_NOT_SET_CLOCK_STOPPED
        };
        rng.gen::<f64>() < p_not_set
    }

    /// Generates whether the defense calls timeout due to the defense not being set
    fn defense_get_set_timeout(&self, context: &PlayContext, norm_risk_taking: f64, rng: &mut impl Rng) -> bool {
        if (context.defense_timeouts() == 0) || (context.quarter() > 2) {
            return false;
        }
        let p_timeout: f64 = 1_f64.min(0_f64.max(
            P_GET_SET_TIMEOUT_INTR + (P_GET_SET_TIMEOUT_COEF * norm_risk_taking)
        ));
        rng.gen::<f64>() < p_timeout
    }

    /// Generates whether the offense calls timeout to conserve clock
    fn offense_conserve_clock_timeout(&self, context: &PlayContext) -> bool {
        if (!context.clock_running()) || (context.offense_timeouts() == 0) {
            return false;
        }
        if context.offense_conserve_clock() {
            return true;
        }
        false
    }

    /// Generates whether the defense calls timeout to conserve clock
    fn defense_conserve_clock_timeout(&self, context: &PlayContext) -> bool {
        if (!context.clock_running()) || (context.defense_timeouts() == 0) {
            return false;
        }
        if context.defense_conserve_clock() {
            return true;
        }
        false
    }

    /// Generates the clock seconds which pass in-between plays
    fn duration(&self, context: &PlayContext, up_tempo: bool, rng: &mut impl Rng) -> u32 {
        if context.drain_clock() {
            let noise: u32 = Exp::new(1_f64).unwrap().sample(rng).round().abs() as u32;
            return 40 - noise;
        }
        let duration = if up_tempo {
            Normal::new(MEAN_UP_TEMPO_BETWEEN_PLAY_DURATION, STD_UP_TEMPO_BETWEEN_PLAY_DURATION).unwrap().sample(rng).round()
        } else {
            SkewNormal::new(MEAN_BETWEEN_PLAY_DURATION, STD_BETWEEN_PLAY_DURATION, SKEW_BETWEEN_PLAY_DURATION).unwrap().sample(rng).round()
        };
        match u32::try_from(duration as i32) {
            Ok(n) => n,
            Err(_) => 0
        }
    }
}

impl PlayResultSimulator for BetweenPlayResultSimulator {
    /// Simulate the events between plays
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a between-play simulator and simulate
    /// let my_sim = BetweenPlayResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult {
        // Calculate normalized skill diffs, skill levels, context values
        let norm_defense_risk_taking: f64 = defense.coach().risk_taking() as f64 / 100_f64;
        let norm_offense_up_tempo: f64 = offense.coach().up_tempo() as f64 / 100_f64;
        let clock_running: bool = context.clock_running();
        let last_play_turnover: bool = *context.last_play_turnover();
        let play_context = PlayContext::from(context);

        // Generate whether the offense goes up-tempo, defense is not set
        let up_tempo: bool = if clock_running && !last_play_turnover && play_context.down() != 4 {
            self.up_tempo(&play_context, norm_offense_up_tempo, rng)
        } else {
            false
        };
        let defense_not_set: bool = if !last_play_turnover {
            self.defense_not_set(up_tempo, clock_running, rng)
        } else {
            false
        };

        // Check if this is a critical down
        let critical_down: bool = play_context.critical_down();

        // Generate whether the defense calls timeout
        let defense_timeout: bool = if !last_play_turnover {
            if defense_not_set || critical_down {
                self.defense_get_set_timeout(&play_context, norm_defense_risk_taking, rng)
            } else {
                self.defense_conserve_clock_timeout(&play_context)
            }
        } else {
            false
        };

        // Generate whether the offense calls timeout
        let offense_timeout: bool = if !(last_play_turnover || defense_timeout) {
            self.offense_conserve_clock_timeout(&play_context)
        } else {
            false
        };

        // Generate the between-play duration
        let between_play_duration: u32 = if !(offense_timeout || defense_timeout) && clock_running {
            self.duration(&play_context, up_tempo, rng)
        } else {
            0
        };
        let between_res = BetweenPlayResult{
            duration: between_play_duration,
            offense_timeout: offense_timeout,
            defense_timeout: defense_timeout,
            up_tempo: up_tempo,
            defense_not_set: defense_not_set,
            critical_down: critical_down
        };
        PlayTypeResult::BetweenPlay(between_res)
    }
}
