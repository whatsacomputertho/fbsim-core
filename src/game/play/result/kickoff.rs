#![doc = include_str!("../../../../docs/game/play/result/kickoff.md")]
use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};
use rand_distr::{Normal, Distribution, Exp, SkewNormal};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayTypeResult, PlayResultSimulator, ScoreResult};

// Touchback probability regression
const P_TOUCHBACK_INTR: f64 = 0.2528877428268531_f64;
const P_TOUCHBACK_COEF: f64 = 0.62457076_f64;

// Out of bounds probability regression
const P_OOB_INTR: f64 = 0.013879833381776598_f64;
const P_OOB_COEF: f64 = -0.01063523_f64;

// Kickoff inside 20 probability
const P_KICKOFF_INSIDE_20: f64 = 0.8_f64; // Adjusted +0.6

// Kickoff inside 20 mean distance
const MEAN_KICKOFF_INSIDE_20_DIST: f64 = 64.3_f64;

// Kickoff inside 20 std distance regression
const STD_KICKOFF_INSIDE_20_DIST_INTR: f64 = 4.516109138481186_f64;
const STD_KICKOFF_INSIDE_20_DIST_COEF: f64 = 1.97369663_f64;

// Kickoff inside 20 skew distance
const SKEW_KICKOFF_INSIDE_20_DIST: f64 = -1.7_f64;

// Kickoff outside 20 mean distance regression
const MEAN_KICKOFF_OUTSIDE_20_DIST_INTR: f64 = 59.31943845056676_f64;
const MEAN_KICKOFF_OUTSIDE_20_DIST_COEF: f64 = -3.42944893_f64;

// Kickoff outside 20 std distance regression
const STD_KICKOFF_OUTSIDE_20_DIST_INTR: f64 = 11.602550109235546_f64;
const STD_KICKOFF_OUTSIDE_20_DIST_COEF: f64 = 6.81862647_f64;

// Kickoff outside 20 skew distance
const SKEW_KICKOFF_OUTSIDE_20_DIST: f64 = -2_f64;

// Fair catch probability regression
const P_FAIR_CATCH_INTR: f64 = 0.02694588730554516_f64;
const P_FAIR_CATCH_COEF: f64 = -0.03716183_f64;

// Mean kickoff return yards regression
const MEAN_KICKOFF_RETURN_YARDS_INTR: f64 = -0.6236115656913945_f64;
const MEAN_KICKOFF_RETURN_YARDS_COEF: f64 = 20.05077203_f64;

// Std kickoff return yards regression
const STD_KICKOFF_RETURN_YARDS_INTR: f64 = 6.421970424325094_f64;
const STD_KICKOFF_RETURN_YARDS_COEF: f64 = 12.34550665_f64;

// Skew kickoff return yards regression
const SKEW_KICKOFF_RETURN_YARDS_INTR: f64 = 3.62041405111988_f64;
const SKEW_KICKOFF_RETURN_YARDS_COEF: f64 = -2.65709746_f64;

// Kickoff return fumble probability
const P_KICKOFF_RETURN_FUMBLE: f64 = 0.007_f64;

// Kickoff return play duration regression
const KICKOFF_RETURN_PLAY_DURATION_INTR: f64 = 0.11217103_f64;
const KICKOFF_RETURN_PLAY_DURATION_COEF: f64 = 1.20326252_f64;

/// # `KickoffResultRaw` struct
///
/// A `KickoffResultRaw` represents a result of a kickoff
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct KickoffResultRaw {
    kickoff_yards: i32,
    kick_return_yards: i32,
    play_duration: u32,
    fumble_return_yards: i32,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    fumble: bool,
    touchdown: bool
}

impl KickoffResultRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure kickoff yards are no more than 100
        if self.kickoff_yards > 100 {
            return Err(
                format!(
                    "Kickoff yards is not in range [0, 100]: {}",
                    self.kickoff_yards
                )
            )
        }

        // Ensure kick return yards are no more than 110
        if self.kick_return_yards > 110 {
            return Err(
                format!(
                    "Kick return yards is not in range [0, 110]: {}",
                    self.kick_return_yards
                )
            )
        }

        // Ensure play duration is no more than 100 seconds
        if self.play_duration > 100 {
            return Err(
                format!(
                    "Play duration is not in range [0, 100]: {}",
                    self.play_duration
                )
            )
        }

        // Ensure fumble return yards are no more than 100
        if self.fumble_return_yards > 100 {
            return Err(
                format!(
                    "Fubmle return yards is not in range [0, 100]: {}",
                    self.fumble_return_yards
                )
            )
        }

        // Ensure mutual exclusivity of touchback, oob, and fair catch
        if self.out_of_bounds && (self.fair_catch || self.touchback) ||
            (self.fair_catch && self.touchback) {
            return Err(
                format!(
                    "Must have at most one true across touchback ({}), out of bounds ({}), and fair catch ({})",
                    self.touchback, self.out_of_bounds, self.fair_catch
                )
            )
        }

        // Ensure not both touchdown and either touchback, oob, fair catch
        if self.touchdown && (self.touchback || self.out_of_bounds || self.fair_catch) {
            return Err(
                format!(
                    "Cannot both score a touchdown and touchback ({}), out of bounds ({}), or fair catch ({})",
                    self.touchback, self.out_of_bounds, self.fair_catch
                )
            )
        }
        Ok(())
    }
}

/// # `KickoffResult` struct
///
/// A `KickoffResult` represents a result of a kickoff
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct KickoffResult {
    kickoff_yards: i32,
    kick_return_yards: i32,
    play_duration: u32,
    fumble_return_yards: i32,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    fumble: bool,
    touchdown: bool
}

impl TryFrom<KickoffResultRaw> for KickoffResult {
    type Error = String;

    fn try_from(item: KickoffResultRaw) -> Result<Self, Self::Error> {
        // Validate the raw between play result
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            KickoffResult{
                kickoff_yards: item.kickoff_yards,
                kick_return_yards: item.kick_return_yards,
                play_duration: item.play_duration,
                fumble_return_yards: item.fumble_return_yards,
                touchback: item.touchback,
                out_of_bounds: item.out_of_bounds,
                fair_catch: item.fair_catch,
                fumble: item.fumble,
                touchdown: item.touchdown
            }
        )
    }
}

impl<'de> Deserialize<'de> for KickoffResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = KickoffResultRaw::deserialize(deserializer)?;
        KickoffResult::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for KickoffResult {
    /// Default constructor for the KickoffResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_result = KickoffResult::default();
    /// ```
    fn default() -> Self {
        KickoffResult{
            kickoff_yards: 65,
            kick_return_yards: 0,
            play_duration: 0,
            fumble_return_yards: 0,
            touchback: true,
            out_of_bounds: false,
            fair_catch: false,
            fumble: false,
            touchdown: false
        }
    }
}

impl std::fmt::Display for KickoffResult {
    /// Format a `KickoffResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_result = KickoffResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let distance_str = format!("Kickoff {} yards", self.kickoff_yards);
        let landing_suffix = if self.touchback {
            " for a touchback."
        } else if self.out_of_bounds {
            " out of bounds."
        } else if self.fair_catch && !self.fumble {
            " for a fair catch."
        } else {
            " fielded."
        };
        let kick_return_str = if !(
            self.touchback || self.out_of_bounds ||
                (self.fair_catch && !self.fumble) ||
                (self.fumble && self.kick_return_yards == 0)
        ) {
            format!(" Returned {} yards.", self.kick_return_yards)
        } else {
            String::from("")
        };
        let fumble_str = if self.fumble {
            format!(" FUMBLED recovered by the kicking team, returned {} yards.", self.fumble_return_yards)
        } else {
            String::from("")
        };
        let touchdown_str = if self.touchdown {
            " TOUCHDOWN!"
        } else {
            ""
        };
        let kickoff_str = format!(
            "{}{}{}{}{}",
            &distance_str,
            landing_suffix,
            &kick_return_str,
            &fumble_str,
            &touchdown_str
        );
        f.write_str(kickoff_str.trim())
    }
}

impl PlayResult for KickoffResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        self.kickoff_yards - self.kick_return_yards + self.fumble_return_yards
    }

    fn turnover(&self) -> bool {
        // In this case, turnover means change of possession
        // Usually fumble means turnover but in this case fumble means no change of possession
        !self.fumble
    }

    fn offense_score(&self) -> ScoreResult {
        if self.touchdown && self.fumble {
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.touchdown && !self.fumble {
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn offense_timeout(&self) -> bool { false }

    fn defense_timeout(&self) -> bool { false }

    fn incomplete(&self) -> bool { false }

    fn out_of_bounds(&self) -> bool {
        self.out_of_bounds
    }

    fn touchback(&self) -> bool {
        self.touchback
    }

    fn kickoff(&self) -> bool { true }

    fn next_play_kickoff(&self) -> bool { false }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown
    }
}

impl KickoffResult {
    /// Initialize a new kickoff result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// ```
    pub fn new() -> KickoffResult {
        KickoffResult::default()
    }

    /// Get a kickoff result's kickoff_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let kickoff_yards = my_res.kickoff_yards();
    /// assert!(kickoff_yards == 65);
    /// ```
    pub fn kickoff_yards(&self) -> i32 {
        self.kickoff_yards
    }

    /// Get a kickoff result's kick_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let kick_return_yards = my_res.kick_return_yards();
    /// assert!(kick_return_yards == 0);
    /// ```
    pub fn kick_return_yards(&self) -> i32 {
        self.kick_return_yards
    }

    /// Get a kickoff result's play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let play_duration = my_res.play_duration();
    /// assert!(play_duration == 0);
    /// ```
    pub fn play_duration(&self) -> u32 {
        self.play_duration
    }

    /// Get a kickoff result's fumble_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let fumble_return_yards = my_res.fumble_return_yards();
    /// assert!(fumble_return_yards == 0);
    /// ```
    pub fn fumble_return_yards(&self) -> i32 {
        self.fumble_return_yards
    }

    /// Get a kickoff result's touchback property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let touchback = my_res.touchback();
    /// assert!(touchback);
    /// ```
    pub fn touchback(&self) -> bool {
        self.touchback
    }

    /// Get a kickoff result's out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let out_of_bounds = my_res.out_of_bounds();
    /// assert!(!out_of_bounds);
    /// ```
    pub fn out_of_bounds(&self) -> bool {
        self.out_of_bounds
    }

    /// Get a kickoff result's fair_catch property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let fair_catch = my_res.fair_catch();
    /// assert!(!fair_catch);
    /// ```
    pub fn fair_catch(&self) -> bool {
        self.fair_catch
    }

    /// Get a kickoff result's fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let fumble = my_res.fumble();
    /// assert!(!fumble);
    /// ```
    pub fn fumble(&self) -> bool {
        self.fumble
    }

    /// Get a kickoff result's touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResult;
    /// 
    /// let my_res = KickoffResult::new();
    /// let touchdown = my_res.touchdown();
    /// assert!(!touchdown);
    /// ```
    pub fn touchdown(&self) -> bool {
        self.touchdown
    }
}

/// # `KickoffResultBuilder` struct
///
/// A `KickoffResultBuilder` is a builder pattern implementation for the
/// `KickoffResult` struct.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct KickoffResultBuilder {
    kickoff_yards: i32,
    kick_return_yards: i32,
    play_duration: u32,
    fumble_return_yards: i32,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    fumble: bool,
    touchdown: bool
}

impl Default for KickoffResultBuilder {
    /// Default constructor for the KickoffResultBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_builder = KickoffResultBuilder::default();
    /// ```
    fn default() -> Self {
        KickoffResultBuilder{
            kickoff_yards: 65,
            kick_return_yards: 0,
            play_duration: 0,
            fumble_return_yards: 0,
            touchback: true,
            out_of_bounds: false,
            fair_catch: false,
            fumble: false,
            touchdown: false
        }
    }
}

impl KickoffResultBuilder {
    /// Initialize a new KickoffResultBuilder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_builder = KickoffResultBuilder::new();
    /// ```
    pub fn new() -> KickoffResultBuilder {
        KickoffResultBuilder::default()
    }

    /// Set the kickoff_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .kickoff_yards(62)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.kickoff_yards() == 62);
    /// ```
    pub fn kickoff_yards(mut self, kickoff_yards: i32) -> Self {
        self.kickoff_yards = kickoff_yards;
        self
    }

    /// Set the kick_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .kick_return_yards(14)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.kick_return_yards() == 14);
    /// ```
    pub fn kick_return_yards(mut self, kick_return_yards: i32) -> Self {
        self.kick_return_yards = kick_return_yards;
        self
    }

    /// Set the play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .play_duration(7)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.play_duration() == 7);
    /// ```
    pub fn play_duration(mut self, play_duration: u32) -> Self {
        self.play_duration = play_duration;
        self
    }

    /// Set the fumble_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .fumble_return_yards(4)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fumble_return_yards() == 4);
    /// ```
    pub fn fumble_return_yards(mut self, fumble_return_yards: i32) -> Self {
        self.fumble_return_yards = fumble_return_yards;
        self
    }

    /// Set the touchback property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .touchback(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(!my_result.touchback());
    /// ```
    pub fn touchback(mut self, touchback: bool) -> Self {
        self.touchback = touchback;
        self
    }

    /// Set the out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .out_of_bounds(true)
    ///     .touchback(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.out_of_bounds());
    /// ```
    pub fn out_of_bounds(mut self, out_of_bounds: bool) -> Self {
        self.out_of_bounds = out_of_bounds;
        self
    }

    /// Set the fair_catch property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .fair_catch(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fair_catch());
    /// ```
    pub fn fair_catch(mut self, fair_catch: bool) -> Self {
        self.fair_catch = fair_catch;
        self
    }

    /// Set the fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .fumble(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fumble());
    /// ```
    pub fn fumble(mut self, fumble: bool) -> Self {
        self.fumble = fumble;
        self
    }

    /// Set the touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .touchdown(true)
    ///     .touchback(false)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.touchdown());
    /// ```
    pub fn touchdown(mut self, touchdown: bool) -> Self {
        self.touchdown = touchdown;
        self
    }

    /// Build the KickoffResult
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultBuilder;
    /// 
    /// let my_result = KickoffResultBuilder::new()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<KickoffResult, String> {
        let raw = KickoffResultRaw{
            kickoff_yards: self.kickoff_yards,
            kick_return_yards: self.kick_return_yards,
            play_duration: self.play_duration,
            fumble_return_yards: self.fumble_return_yards,
            touchback: self.touchback,
            out_of_bounds: self.out_of_bounds,
            fair_catch: self.fair_catch,
            fumble: self.fumble,
            touchdown: self.touchdown
        };
        KickoffResult::try_from(raw)
    }
}

/// # `KickoffResultSimulator` struct
///
/// A `KickoffResultSimulator` represents a simulator which can produce a result of a kickoff
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct KickoffResultSimulator {}

impl KickoffResultSimulator {
    /// Initialize a new KickoffResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::kickoff::KickoffResultSimulator;
    ///
    /// let my_sim = KickoffResultSimulator::new();
    /// ```
    pub fn new() -> KickoffResultSimulator {
        KickoffResultSimulator{}
    }

    /// Generates whether the kickoff was a touchback
    fn touchback(&self, norm_kicking: f64, rng: &mut impl Rng) -> bool {
        let p_touchback: f64 = 1_f64.min(0_f64.max(
            P_TOUCHBACK_INTR + (P_TOUCHBACK_COEF * norm_kicking)
        ));
        rng.gen::<f64>() < p_touchback
    }

    /// Generates whether the kickoff went out of bounds
    fn out_of_bounds(&self, norm_kicking: f64, rng: &mut impl Rng) -> bool {
        let p_oob: f64 = 1_f64.min(0_f64.max(
            P_OOB_INTR + (P_OOB_COEF * norm_kicking)
        ));
        rng.gen::<f64>() < p_oob
    }

    /// Generates whether the kickoff landed inside the 20
    fn inside_20(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f64>() < P_KICKOFF_INSIDE_20
    }

    /// Generates the distance of the kickoff
    fn distance(&self, norm_kicking: f64, inside_20: bool, rng: &mut impl Rng) -> i32 {
        let mean_dist: f64 = if inside_20 {
            MEAN_KICKOFF_INSIDE_20_DIST
        } else {
            MEAN_KICKOFF_OUTSIDE_20_DIST_INTR + (MEAN_KICKOFF_OUTSIDE_20_DIST_COEF * norm_kicking)
        };
        let std_dist: f64 = if inside_20 {
            STD_KICKOFF_INSIDE_20_DIST_INTR + (STD_KICKOFF_INSIDE_20_DIST_COEF * norm_kicking)
        } else {
            STD_KICKOFF_OUTSIDE_20_DIST_INTR + (STD_KICKOFF_OUTSIDE_20_DIST_COEF * norm_kicking)
        };
        let skew_dist: f64 = if inside_20 {
            SKEW_KICKOFF_INSIDE_20_DIST
        } else {
            SKEW_KICKOFF_OUTSIDE_20_DIST
        };
        let dist_dist = SkewNormal::new(mean_dist, std_dist, skew_dist).unwrap();
        dist_dist.sample(rng).round() as i32
    }

    /// Generates whether a fair catch was called on the kickoff
    fn fair_catch(&self, norm_diff_returning: f64, rng: &mut impl Rng) -> bool {
        let p_fair_catch: f64 = 1_f64.min(0_f64.max(
            P_FAIR_CATCH_INTR + (P_FAIR_CATCH_COEF * norm_diff_returning)
        ));
        rng.gen::<f64>() < p_fair_catch
    }

    /// Generates the kick return yards
    fn return_yards(&self, norm_diff_returning: f64, rng: &mut impl Rng) -> i32 {
        let mean_return_yards: f64 = MEAN_KICKOFF_RETURN_YARDS_INTR + (MEAN_KICKOFF_RETURN_YARDS_COEF * norm_diff_returning);
        let std_return_yards: f64 = STD_KICKOFF_RETURN_YARDS_INTR + (STD_KICKOFF_RETURN_YARDS_COEF * norm_diff_returning);
        let skew_return_yards: f64 = SKEW_KICKOFF_RETURN_YARDS_INTR + (SKEW_KICKOFF_RETURN_YARDS_COEF * norm_diff_returning);
        let return_yards_dist = SkewNormal::new(mean_return_yards, std_return_yards, skew_return_yards).unwrap();
        return_yards_dist.sample(rng).round() as i32
    }

    /// Generates whether a fumble occurred on the kick return
    fn fumble(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f64>() < P_KICKOFF_RETURN_FUMBLE
    }

    /// Generates the fumble recovery return yards
    fn fumble_return_yards(&self, rng: &mut impl Rng) -> i32 {
        Exp::new(1_f64).unwrap().sample(rng).round() as i32
    }

    /// Generates the duration of the kickoff play in seconds
    fn play_duration(&self, total_yards: u32, rng: &mut impl Rng) -> u32 {
        let mean_duration: f64 = KICKOFF_RETURN_PLAY_DURATION_INTR + (KICKOFF_RETURN_PLAY_DURATION_COEF * total_yards as f64);
        let duration_dist = Normal::new(mean_duration, 2_f64).unwrap();
        u32::try_from(duration_dist.sample(rng).sqrt().round() as i32).unwrap_or_default()
    }
}

impl PlayResultSimulator for KickoffResultSimulator {
    /// Simulate a kickoff
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::kickoff::KickoffResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a kickoff simulator and simulate a kickoff
    /// let my_sim = KickoffResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult {
        // Calculate normalized skill diffs & skill levels
        let norm_kicking: f64 = offense.offense().kickoffs() as f64 / 100_f64;
        let norm_diff_returning: f64 = 0.5_f64 + ((defense.defense().kick_returning() as f64 - offense.offense().kick_return_defense() as f64) / 200_f64);
        let td_yards: i32 = context.yards_to_touchdown();
        let safety_yards: i32 = context.yards_to_safety();

        // Generate whether the kickoff was a touchback
        let touchback: bool = self.touchback(norm_kicking, rng);

        // Generate whether the kickoff went out of bounds
        let out_of_bounds: bool = if !touchback {
            self.out_of_bounds(norm_kicking, rng)
        } else {
            false
        };

        // Generate whether the kickoff landed inside the 20
        let inside_20: bool = if !touchback {
            self.inside_20(rng)
        } else {
            false
        };

        // Generate the kickoff distance
        let kickoff_distance: i32 = if !touchback {
            td_yards.min(self.distance(norm_kicking, inside_20, rng))
        } else {
            td_yards
        };

        // Generate whether a fair catch was called on the kickoff
        let fair_catch: bool = if !(touchback || out_of_bounds) {
            self.fair_catch(norm_diff_returning, rng)
        } else {
            false
        };

        // Generate the kickoff return yards
        let return_yards: i32 = if !(touchback || out_of_bounds || fair_catch) {
            self.return_yards(norm_diff_returning, rng).min(safety_yards + kickoff_distance)
        } else {
            0
        };

        // Generate whether a fumble occurred on the kickoff
        let fumble: bool = if !(touchback || out_of_bounds || fair_catch) {
            self.fumble(rng)
        } else {
            false
        };

        // Generate the fumble return yards
        let fumble_return_yards: i32 = if fumble {
            self.fumble_return_yards(rng)
        } else {
            0
        };

        // Generate the duration of the kickoff in seconds
        let total_yards: u32 = kickoff_distance.unsigned_abs() + return_yards.unsigned_abs() + fumble_return_yards.unsigned_abs();
        let play_duration: u32 = if !(touchback || out_of_bounds || fair_catch) {
            self.play_duration(total_yards, rng)
        } else {
            0
        };

        // Check whether a touchdown occurred
        let touchdown: bool = if fumble {
            kickoff_distance - return_yards + fumble_return_yards > td_yards
        } else if !(touchback || out_of_bounds || fair_catch) {
            kickoff_distance - return_yards < safety_yards
        } else {
            false
        };

        let raw = KickoffResultRaw{
            kickoff_yards: kickoff_distance,
            kick_return_yards: return_yards,
            play_duration,
            fumble_return_yards,
            touchback,
            out_of_bounds,
            fair_catch,
            fumble,
            touchdown
        };
        let kickoff_res = KickoffResult::try_from(raw).unwrap();
        PlayTypeResult::Kickoff(kickoff_res)
    }
}
