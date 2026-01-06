#![doc = include_str!("../../../../docs/game/play/result/run.md")]
use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};
use rand_distr::{Normal, Distribution, Exp};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayTypeResult, PlayResultSimulator, ScoreResult};

// Mean & std regression for standard rushing play
const MEAN_YARDS_INTR: f64 = 2.2503791522871384_f64; // adjusted -0.8
const MEAN_YARDS_COEF: f64 = 0.92550597_f64; // adjusted + 0.6
const STD_YARDS_INTR: f64 = 4.053915588534795_f64;
const STD_YARDS_COEF_1: f64 = 0.2487578_f64;
const STD_YARDS_COEF_2: f64 = 0.0593874_f64;

// Mean & std regression for big, non-TD rushing play
const MEAN_BP_YARDS_INTR: f64 = 12.781025340879893_f64; // adjusted -3
const MEAN_BP_YARDS_COEF: f64 = 16.32805521_f64; // adjusted +10
const STD_BP_YARDS_INTR: f64 = 10.014877063200005_f64;
const STD_BP_YARDS_COEF_1: f64 = -3.82403981_f64;
const STD_BP_YARDS_COEF_2: f64 = 7.60215528_f64;

// Mean regresion for play duration
const MEAN_DURATION_INTR: f64 = 8.32135821_f64; // Adjusted + 3
const MEAN_DURATION_COEF_1: f64 = 0.11343699_f64;
const MEAN_DURATION_COEF_2: f64 = -0.00056798_f64;

// TD probability regression for big rushing play
const P_BP_TD_INTR: f64 = -3.9968093269427603;
const P_BP_TD_COEF: f64 = 0.39426769;

// Big play probability regression
const P_BP_INTR: f64 = -2.878726031553263;
const P_BP_COEF: f64 = 0.82863208;

// Fumble probability regression
const P_FUMBLE_INTR: f64 = 0.04932479844415921;
const P_FUMBLE_COEF: f64 = -0.08432772;

/// # `RunResultRaw` struct
///
/// A `RunResultRaw` represents a result of a run play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct RunResultRaw {
    yards_gained: i32,
    play_duration: u32,
    fumble: bool,
    return_yards: i32,
    out_of_bounds: bool,
    touchdown: bool,
    safety: bool,
    two_point_conversion: bool
}

impl RunResultRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure play duration is not greater than 100
        if self.play_duration > 100 {
            return Err(
                format!(
                    "Play duration is not in range [0, 100]: {}",
                    self.play_duration
                )
            )
        }

        // Ensure yards gained are in range [-100, 100]
        if self.yards_gained.abs() > 100 {
            return Err(
                format!(
                    "Yards gained is not in range [-100, 100]: {}",
                    self.yards_gained
                )
            )
        }

        // Ensure return yards are in range [-100, 100]
        if self.return_yards.abs() > 100 {
            return Err(
                format!(
                    "Return yards is not in range [-100, 100]: {}",
                    self.return_yards
                )
            )
        }

        // Ensure if there is no fumble, the return yards are zero
        if !self.fumble && self.return_yards != 0 {
            return Err(
                format!(
                    "Fumble did not occur but return yards were nonzero: {}",
                    self.return_yards
                )
            )
        }

        // Ensure if there is a fumble, there is not also a safety
        if self.fumble && self.safety {
            return Err(
                String::from("Cannot have both a fumble and a safety")
            )
        }

        // Ensure if out of bounds, there is not also a touchdown or safety
        if self.out_of_bounds && (self.touchdown || self.safety) {
            return Err(
                format!(
                    "Cannot have both a rush out of bounds and a touchdown ({}) or safety ({})",
                    self.touchdown, self.safety
                )
            )
        }

        // Ensure there is not both a safety and a touchdown
        if self.touchdown && self.safety {
            return Err(
                String::from("Cannot have both a touchdown and a safety")
            )
        }
        Ok(())
    }
}

/// # `RunResult` struct
///
/// A `RunResult` represents a result of a run play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct RunResult {
    yards_gained: i32,
    play_duration: u32,
    fumble: bool,
    return_yards: i32,
    out_of_bounds: bool,
    touchdown: bool,
    safety: bool,
    two_point_conversion: bool
}

impl TryFrom<RunResultRaw> for RunResult {
    type Error = String;

    fn try_from(item: RunResultRaw) -> Result<Self, Self::Error> {
        // Validate the raw between play result
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            RunResult{
                yards_gained: item.yards_gained,
                play_duration: item.play_duration,
                fumble: item.fumble,
                return_yards: item.return_yards,
                out_of_bounds: item.out_of_bounds,
                touchdown: item.touchdown,
                safety: item.safety,
                two_point_conversion: item.two_point_conversion
            }
        )
    }
}

impl<'de> Deserialize<'de> for RunResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = RunResultRaw::deserialize(deserializer)?;
        RunResult::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for RunResult {
    /// Default constructor for the RunResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_result = RunResult::default();
    /// ```
    fn default() -> Self {
        RunResult{
            yards_gained: 0,
            play_duration: 0,
            fumble: false,
            return_yards: 0,
            out_of_bounds: false,
            touchdown: false,
            safety: false,
            two_point_conversion: false
        }
    }
}

impl std::fmt::Display for RunResult {
    /// Format a `RunResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_result = RunResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dist_str = format!("Rush {} yards.", self.yards_gained);
        let fumble_str = if self.fumble {
            format!(" FUMBLE recovered by the defense, returned {} yards.", self.return_yards)
        } else {
            String::from("")
        };
        let result_str = if self.touchdown {
            if self.two_point_conversion {
                " Two point conversion is GOOD!"
            } else {
                " TOUCHDOWN!"
            }
        } else if self.safety {
            " SAFETY!"
        } else if self.two_point_conversion {
            " Two point conversion is no good."
        } else {
            ""
        };
        let run_str = format!(
            "{}{}{}",
            &dist_str,
            &fumble_str,
            result_str
        );
        f.write_str(&run_str)
    }
}

impl PlayResult for RunResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        self.yards_gained - self.return_yards
    }

    fn turnover(&self) -> bool {
        self.fumble
    }

    fn offense_score(&self) -> ScoreResult {
        if self.touchdown && !self.fumble {
            if self.two_point_conversion {
                return ScoreResult::TwoPointConversion;
            }
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.touchdown && self.fumble {
            if self.two_point_conversion {
                ScoreResult::TwoPointConversion
            } else {
                ScoreResult::Touchdown
            }
        } else if self.safety {
            ScoreResult::Safety
        } else {
            ScoreResult::None
        }
    }

    fn offense_timeout(&self) -> bool { false }

    fn defense_timeout(&self) -> bool { false }

    fn incomplete(&self) -> bool { false }

    fn out_of_bounds(&self) -> bool { false }

    fn kickoff(&self) -> bool { false }

    fn next_play_kickoff(&self) -> bool {
        self.safety || self.two_point_conversion
    }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown && !self.two_point_conversion
    }
}

impl RunResult {
    /// Initialize a new run result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// ```
    pub fn new() -> RunResult {
        RunResult::default()
    }

    /// Get a run result's play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let play_duration = my_res.play_duration();
    /// assert!(play_duration == 0);
    /// ```
    pub fn play_duration(&self) -> u32 {
        self.play_duration
    }

    /// Get a run result's yards_gained property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let yards_gained = my_res.yards_gained();
    /// assert!(yards_gained == 0);
    /// ```
    pub fn yards_gained(&self) -> i32 {
        self.yards_gained
    }

    /// Get a run result's return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let return_yards = my_res.return_yards();
    /// assert!(return_yards == 0);
    /// ```
    pub fn return_yards(&self) -> i32 {
        self.return_yards
    }

    /// Get a run result's fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let fumble = my_res.fumble();
    /// assert!(!fumble);
    /// ```
    pub fn fumble(&self) -> bool {
        self.fumble
    }

    /// Get a run result's out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let out_of_bounds = my_res.out_of_bounds();
    /// assert!(!out_of_bounds);
    /// ```
    pub fn out_of_bounds(&self) -> bool {
        self.out_of_bounds
    }

    /// Get a run result's touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let touchdown = my_res.touchdown();
    /// assert!(!touchdown);
    /// ```
    pub fn touchdown(&self) -> bool {
        self.touchdown
    }

    /// Get a run result's safety property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let safety = my_res.safety();
    /// assert!(!safety);
    /// ```
    pub fn safety(&self) -> bool {
        self.safety
    }

    /// Get a run result's two point conversion property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_res = RunResult::new();
    /// let two_point_conversion = my_res.two_point_conversion();
    /// assert!(!two_point_conversion);
    /// ```
    pub fn two_point_conversion(&self) -> bool {
        self.two_point_conversion
    }
}

/// # `RunResultBuilder` struct
///
/// A `RunResultBuilder` is a builder pattern implementation for the
/// `RunResult` struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct RunResultBuilder {
    yards_gained: i32,
    play_duration: u32,
    fumble: bool,
    return_yards: i32,
    out_of_bounds: bool,
    touchdown: bool,
    safety: bool,
    two_point_conversion: bool
}

impl Default for RunResultBuilder {
    /// Default constructor for the RunResultBuilder struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::default();
    /// ```
    fn default() -> Self {
        RunResultBuilder{
            yards_gained: 0,
            play_duration: 0,
            fumble: false,
            return_yards: 0,
            out_of_bounds: false,
            touchdown: false,
            safety: false,
            two_point_conversion: false
        }
    }
}

impl RunResultBuilder {
    /// Initialize a new RunResultBuilder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_builder = RunResultBuilder::new();
    /// ```
    pub fn new() -> RunResultBuilder {
        RunResultBuilder::default()
    }

    /// Set the play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .play_duration(10)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.play_duration() == 10);
    /// ```
    pub fn play_duration(mut self, play_duration: u32) -> Self {
        self.play_duration = play_duration;
        self
    }

    /// Set the yards_gained property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .yards_gained(3)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.yards_gained() == 3);
    /// ```
    pub fn yards_gained(mut self, yards_gained: i32) -> Self {
        self.yards_gained = yards_gained;
        self
    }

    /// Set the return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .fumble(true)
    ///     .return_yards(12)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.return_yards() == 12);
    /// ```
    pub fn return_yards(mut self, return_yards: i32) -> Self {
        self.return_yards = return_yards;
        self
    }

    /// Set the fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .fumble(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fumble());
    /// ```
    pub fn fumble(mut self, fumble: bool) -> Self {
        self.fumble = fumble;
        self
    }

    /// Set the out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .out_of_bounds(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.out_of_bounds());
    /// ```
    pub fn out_of_bounds(mut self, out_of_bounds: bool) -> Self {
        self.out_of_bounds = out_of_bounds;
        self
    }

    /// Set the touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .touchdown(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.touchdown());
    /// ```
    pub fn touchdown(mut self, touchdown: bool) -> Self {
        self.touchdown = touchdown;
        self
    }

    /// Set the safety property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .safety(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.safety());
    /// ```
    pub fn safety(mut self, safety: bool) -> Self {
        self.safety = safety;
        self
    }

    /// Set the two_point_conversion property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .two_point_conversion(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.two_point_conversion());
    /// ```
    pub fn two_point_conversion(mut self, two_point_conversion: bool) -> Self {
        self.two_point_conversion = two_point_conversion;
        self
    }

    /// Build the RunResult
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultBuilder;
    /// 
    /// let my_result = RunResultBuilder::new()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<RunResult, String> {
        let raw = RunResultRaw{
            yards_gained: self.yards_gained,
            play_duration: self.play_duration,
            fumble: self.fumble,
            return_yards: self.return_yards,
            out_of_bounds: self.out_of_bounds,
            touchdown: self.touchdown,
            safety: self.safety,
            two_point_conversion: self.two_point_conversion
        };
        RunResult::try_from(raw)
    }
}

/// # `RunResultSimulator` struct
///
/// A `RunResultSimulator` represents a simulator which can produce a result of a run play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct RunResultSimulator {}

impl RunResultSimulator {
    /// Initialize a new RunResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::run::RunResultSimulator;
    ///
    /// let my_sim = RunResultSimulator::new();
    /// ```
    pub fn new() -> RunResultSimulator {
        RunResultSimulator{}
    }

    /// Generates whether this is a big rushing play
    fn big_play(&self, norm_diff_rushing: f64, rng: &mut impl Rng) -> bool {
        let p_big_play: f64 = 1_f64.min(0_f64.max((P_BP_INTR + (P_BP_COEF * norm_diff_rushing)).exp()));
        rng.gen::<f64>() < p_big_play
    }

    /// Generates whether this is a big play touchdown
    fn big_play_touchdown(&self, norm_diff_rushing: f64, rng: &mut impl Rng) -> bool {
        let p_bp_td: f64 = 1_f64.min(0_f64.max((P_BP_TD_INTR + (P_BP_TD_COEF * norm_diff_rushing)).exp()));
        rng.gen::<f64>() < p_bp_td
    }

    /// Generates the duration of the play
    fn play_duration(&self, total_yards: u32, rng: &mut impl Rng) -> u32 {
        let mean_duration: f64 = MEAN_DURATION_INTR + (MEAN_DURATION_COEF_1 * total_yards as f64) + (MEAN_DURATION_COEF_2 * total_yards.pow(2) as f64);
        let duration_dist = Normal::new(mean_duration, 2_f64).unwrap();
        u32::try_from(duration_dist.sample(rng).round() as i32).unwrap_or_default()
    }

    /// Generaes the rushing yards on the play
    fn rushing_yards(&self, norm_diff_rushing: f64, big_play: bool, rng: &mut impl Rng) -> i32 {
        let mean_yards: f64 = if big_play {
            MEAN_BP_YARDS_INTR + (MEAN_BP_YARDS_COEF * norm_diff_rushing)
        } else {
            MEAN_YARDS_INTR + (MEAN_YARDS_COEF * norm_diff_rushing)
        };
        let std_yards: f64 = if big_play {
            STD_BP_YARDS_INTR + (STD_BP_YARDS_COEF_1 * norm_diff_rushing) + (STD_BP_YARDS_COEF_2 * norm_diff_rushing.powi(2))
        } else {
            STD_YARDS_INTR + (STD_YARDS_COEF_1 * norm_diff_rushing) + (STD_YARDS_COEF_2 * norm_diff_rushing.powi(2))
        };
        let yards_dist = Normal::new(mean_yards, std_yards).unwrap();
        yards_dist.sample(rng).round() as i32
    }

    /// Generates whether a fumble occurred on the play
    fn fumble(&self, norm_diff_turnovers: f64, rng: &mut impl Rng) -> bool {
        let p_fumble: f64 = 1_f64.min(0.001_f64.max(P_FUMBLE_INTR + (P_FUMBLE_COEF * norm_diff_turnovers)));
        rng.gen::<f64>() < p_fumble
    }

    /// Generates the fumble recovery return yards on the play
    fn fumble_return_yards(&self, rng: &mut impl Rng) -> i32 {
        Exp::new(1_f64).unwrap().sample(rng).round() as i32
    }
}

impl PlayResultSimulator for RunResultSimulator {
    /// Simulate a run play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::run::RunResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a run play simulator and simulate a play
    /// let my_sim = RunResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult {
        // Derive the normalized skill differentials for each team
        let offense_advantage: bool = context.offense_advantage();
        let defense_advantage: bool = context.defense_advantage();
        let norm_diff_rushing: f64 = 0.5_f64 + (
            (
                offense.offense().rushing_advantage(offense_advantage) as f64 -
                defense.defense().rush_defense_advantage(defense_advantage) as f64
            ) / 200_f64
        );
        let norm_diff_turnovers: f64 = 0.5_f64 + (
            (
                offense.offense().turnovers_advantage(offense_advantage) as f64 -
                defense.defense().turnovers_advantage(defense_advantage) as f64
            ) / 200_f64
        );
        let td_yards = context.yards_to_touchdown();
        let safety_yards = context.yards_to_safety();

        // Generate yards gained on the play
        let yards_gained: i32 = if self.big_play(norm_diff_rushing, rng) {
            if self.big_play_touchdown(norm_diff_rushing, rng) {
                td_yards
            } else {
                safety_yards.max(td_yards.min(self.rushing_yards(norm_diff_rushing, true, rng)))
            }
        } else {
            safety_yards.max(td_yards.min(self.rushing_yards(norm_diff_rushing, false, rng)))
        };

        // Determine if a touchdown or safety occurred
        let mut touchdown: bool = yards_gained == td_yards;
        let mut safety: bool = yards_gained == safety_yards;

        // If neither a touchdown or safety occurred, determine if a fumble occurred
        let fumble: bool = if !(touchdown || safety) {
            self.fumble(norm_diff_turnovers, rng)
        } else {
            false
        };

        // If a fumble occurred, generate the return yards, re-check for TD or safety
        let return_yards: i32 = if fumble {
            self.fumble_return_yards(rng)
        } else {
            0
        };
        let total_yards: u32 = yards_gained.unsigned_abs() + return_yards.unsigned_abs();
        let net_yards: i32 = safety_yards.max(td_yards.min(yards_gained - return_yards));
        touchdown = if fumble {
            net_yards == safety_yards
        } else {
            touchdown
        };
        safety = if fumble {
            net_yards == td_yards
        } else {
            safety
        };

        // Construct the run result
        let raw = RunResultRaw{
            yards_gained,
            play_duration: self.play_duration(total_yards, rng),
            fumble,
            return_yards,
            out_of_bounds: false,
            touchdown,
            safety,
            two_point_conversion: context.next_play_extra_point()
        };
        let run_res = RunResult::try_from(raw).unwrap();
        PlayTypeResult::Run(run_res)
    }
}
