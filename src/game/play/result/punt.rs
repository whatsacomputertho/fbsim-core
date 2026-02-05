#![doc = include_str!("../../../../docs/game/play/result/punt.md")]
use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "wasm")]
use tsify_next::Tsify;
use rand_distr::{Normal, Distribution, Exp, SkewNormal};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayTypeResult, PlayResultSimulator, ScoreResult};

// Punt block probability regression
const P_BLOCK_INTR: f64 = -0.0010160286505995551_f64;
const P_BLOCK_COEF: f64 = 0.00703673_f64;

// Punt inside 20 skill-based probability regression
const P_PUNT_INSIDE_20_SKILL_INTR: f64 = 0.21398823243670145_f64;
const P_PUNT_INSIDE_20_SKILL_COEF: f64 = 0.52878206_f64; // Adjusted +0.2

// Punt inside 20 yard-line-based probability regression
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_1: f64 = 0.783829627_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_2: f64 = -0.200560110_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_3: f64 = 0.651500015_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_4: f64 = -0.00178251834_f64;

// Punt inside 20 mean relative distance regression
const PUNT_INSIDE_20_MEAN_REL_DIST_INTR: f64 = 0.20907739629135946_f64;
const PUNT_INSIDE_20_MEAN_REL_DIST_COEF: f64 = -0.0001755_f64;

// Punt inside 20 std relative distance regression
const PUNT_INSIDE_20_STD_REL_DIST_INTR: f64 = 0.17519244654293623_f64;
const PUNT_INSIDE_20_STD_REL_DIST_COEF: f64 = -0.0016178_f64;

// Punt inside 20 skew relative distance regression
const PUNT_INSIDE_20_SKEW_REL_DIST_INTR: f64 = 3.691739354624472_f64;
const PUNT_INSIDE_20_SKEW_REL_DIST_COEF_1: f64 = -0.11961015_f64;
const PUNT_INSIDE_20_SKEW_REL_DIST_COEF_2: f64 = 0.00081621_f64;

// Punt outside 20 mean relative distance regression
const PUNT_OUTSIDE_20_MEAN_REL_DIST_INTR: f64 = -0.24995460069957565_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_1: f64 = 0.0400507456_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_2: f64 = -0.000758718087_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_3: f64 = 0.00000442573043_f64;

// Punt outside 20 std relative distance regression
const PUNT_OUTSIDE_20_STD_REL_DIST_INTR: f64 = 0.2748076520973469_f64;
const PUNT_OUTSIDE_20_STD_REL_DIST_COEF: f64 = -0.00196699_f64;

// Punt outside 20 skew relative distance regression
const PUNT_OUTSIDE_20_SKEW_REL_DIST_INTR: f64 = -5.631745519232158_f64;
const PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_1: f64 = 0.19789058_f64;
const PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_2: f64 = -0.00134607_f64;

// Punt out of bounds probability regression
const P_PUNT_OOB_INTR: f64 = -0.0846243447082426_f64;
const P_PUNT_OOB_COEF_1: f64 = 0.00575805979_f64;
const P_PUNT_OOB_COEF_2: f64 = -0.0000428367831_f64;

// Punt fair catch probability regression
const P_FAIR_CATCH_INTR: f64 = 0.47613371173695526_f64;
const P_FAIR_CATCH_COEF: f64 = -0.00141214_f64;

// Punt muffed probability regression
const P_MUFFED_PUNT_INTR: f64 = 0.036855240326056096_f64;
const P_MUFFED_PUNT_COEF: f64 = -0.02771741_f64;

// Mean relative punt return yards regression
const MEAN_REL_RETURN_YARDS_INTR: f64 = -0.0770321871_f64; // Adjusted - 0.02
const MEAN_REL_RETURN_YARDS_COEF_1: f64 = -0.02282631_f64;
const MEAN_REL_RETURN_YARDS_COEF_2: f64 = 0.28982747_f64;

// Std relative punt return yards regression
const STD_REL_RETURN_YARDS_INTR: f64 = 0.06751127059206394_f64;
const STD_REL_RETURN_YARDS_COEF_1: f64 = 0.01035858_f64;
const STD_REL_RETURN_YARDS_COEF_2: f64 = 0.26338509_f64;

// Skew relative punt return yards regression
const SKEW_REL_RETURN_YARDS_INTR: f64 = -0.0167472281_f64;
const SKEW_REL_RETURN_YARDS_COEF_1: f64 = 7.06931813_f64;
const SKEW_REL_RETURN_YARDS_COEF_2: f64 = -6.94528823_f64;

// Fumble probability regression
const P_FUMBLE_INTR: f64 = 0.0460047101408259_f64;
const P_FUMBLE_COEF: f64 = -0.08389777_f64; // Adjusted - 0.04

// Punt play duration regression
const PUNT_PLAY_DURATION_INTR: f64 = 8.2792296_f64; // Adjusted + 3
const PUNT_PLAY_DURATION_COEF: f64 = 0.09291598_f64;

/// # `PuntResultRaw` struct
///
/// A `PuntResultRaw` is a `PuntResult` before its properties have been
/// validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct PuntResultRaw {
    fumble_return_yards: i32,
    punt_yards: i32,
    punt_return_yards: i32,
    play_duration: u32,
    blocked: bool,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    muffed: bool,
    fumble: bool,
    touchdown: bool
}

impl PuntResultRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure the play duration is not greater than 100 seconds
        if self.play_duration > 100 {
            return Err(
                format!(
                    "Play duration is not in range [0, 100]: {}",
                    self.play_duration
                )
            )
        }

        // Ensure the fumble return yards are in range [-100, 100]
        if self.fumble_return_yards.abs() > 100 {
            return Err(
                format!(
                    "Fumble return yards is not in range [-100, 100]: {}",
                    self.fumble_return_yards
                )
            )
        }

        // Ensure the fumble return yards are zero if a fumble did not occur
        if !self.fumble && self.fumble_return_yards != 0 {
            return Err(
                format!(
                    "Fumble did not occur but fumble return yards are nonzero: {}",
                    self.fumble_return_yards
                )
            )
        }

        // Ensure the punt yards are in range [-100, 100]
        if self.punt_yards.abs() > 100 {
            return Err(
                format!(
                    "Punt yards is not in range [-100, 100]: {}",
                    self.punt_yards
                )
            )
        }

        // Ensure the punt return yards are in range [-110, 110]
        if self.punt_return_yards.abs() > 110 {
            return Err(
                format!(
                    "Punt return yards is not in range [-110, 110]: {}",
                    self.punt_return_yards
                )
            )
        }

        // Ensure punt return yards are zero if punt was not returned
        if (self.blocked || self.out_of_bounds || self.touchback || self.fair_catch || self.muffed) && self.punt_return_yards != 0 {
            return Err(
                format!(
                    "Punt was not returned but punt return yards were nonzero: {}",
                    self.punt_return_yards
                )
            )
        }

        // Ensure if muffed, a fumble also occurred
        if self.muffed && !self.fumble {
            return Err(
                String::from("Cannot have a muffed punt that was not also fumbled")
            )
        }

        // Ensure if blocked, not also oob, touchback, fair catch, or muffed
        if self.blocked && (self.out_of_bounds || self.touchback || self.fair_catch || self.muffed) {
            return Err(
                format!(
                    "Cannot have both blocked punt and out of bounds ({}), touchback ({}), fair catch ({}), or muffed ({})",
                    self.out_of_bounds, self.touchback, self.fair_catch, self.muffed
                )
            )
        }

        // Ensure if touchback, not also oob, fair catch, muffed, fumble, or TD
        if self.touchback && (self.out_of_bounds || self.fair_catch || self.fumble || self.touchdown) {
            return Err(
                format!(
                    "Cannot have both touchback and out of bounds ({}), fair catch ({}), fumble ({}), or touchdown ({})",
                    self.out_of_bounds, self.fair_catch, self.fumble, self.touchdown
                )
            )
        }

        // Ensure if out of bounds, not also fair catch, muffed, fumble, or TD
        if self.out_of_bounds && (self.fair_catch || self.muffed || self.fumble || self.touchdown) {
            return Err(
                format!(
                    "Cannot have both punt out of bounds and fair catch ({}), muffed ({}), fumble ({}), or touchdown ({})",
                    self.fair_catch, self.muffed, self.fumble, self.touchdown
                )
            )
        }
        Ok(())
    }
}

/// # `PuntResult` struct
///
/// A `PuntResult` represents a result of a punt play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct PuntResult {
    fumble_return_yards: i32,
    punt_yards: i32,
    punt_return_yards: i32,
    play_duration: u32,
    blocked: bool,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    muffed: bool,
    fumble: bool,
    touchdown: bool
}

impl TryFrom<PuntResultRaw> for PuntResult {
    type Error = String;

    fn try_from(item: PuntResultRaw) -> Result<Self, Self::Error> {
        // Validate the raw between play result
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            PuntResult{
                fumble_return_yards: item.fumble_return_yards,
                punt_yards: item.punt_yards,
                punt_return_yards: item.punt_return_yards,
                play_duration: item.play_duration,
                blocked: item.blocked,
                touchback: item.touchback,
                out_of_bounds: item.out_of_bounds,
                fair_catch: item.fair_catch,
                muffed: item.muffed,
                fumble: item.fumble,
                touchdown: item.touchdown
            }
        )
    }
}

impl<'de> Deserialize<'de> for PuntResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = PuntResultRaw::deserialize(deserializer)?;
        PuntResult::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for PuntResult {
    /// Default constructor for the PuntResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_result = PuntResult::default();
    /// ```
    fn default() -> Self {
        PuntResult{
            fumble_return_yards: 0,
            punt_yards: 0,
            punt_return_yards: 0,
            play_duration: 0,
            blocked: false,
            touchback: false,
            out_of_bounds: false,
            fair_catch: false,
            muffed: false,
            fumble: false,
            touchdown: false
        }
    }
}

impl std::fmt::Display for PuntResult {
    /// Format a `PuntResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_result = PuntResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blocked_prefix = if self.blocked {
            format!("Punt BLOCKED, returned {} yards.", self.fumble_return_yards)
        } else {
            String::from("")
        };
        let punt_distance_str = if !self.blocked {
            format!("Punt {} yards", self.punt_yards)
        } else {
            String::from("")
        };
        let catch_str = if self.touchback {
            String::from(" for a touchback.")
        } else if self.out_of_bounds {
            String::from(" out of bounds.")
        } else if self.fair_catch {
            if self.muffed {
                String::from(" fair catch MUFFED.")
            } else {
                String::from(" for a fair catch.")
            }
        } else {
            String::from(" fielded.")
        };
        let return_str = if !(self.touchback || self.out_of_bounds || (self.fair_catch && !self.muffed)) {
            format!(" Punt returned {} yards.", self.punt_return_yards)
        } else {
            String::from("")
        };
        let fumble_str = if self.fumble {
            format!(" FUMBLE recovered by the kicking team, returned {} yards", self.fumble_return_yards)
        } else {
            String::from("")
        };
        let punt_str = format!(
            "{}{}{}{}{}",
            &blocked_prefix,
            &punt_distance_str,
            &catch_str,
            &return_str,
            &fumble_str
        );
        f.write_str(punt_str.trim())
    }
}

impl PlayResult for PuntResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        self.punt_yards - self.punt_return_yards + self.fumble_return_yards
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

    fn kickoff(&self) -> bool { false }

    fn punt(&self) -> bool { true }

    fn next_play_kickoff(&self) -> bool { false }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown
    }
}

impl PuntResult {
    /// Initialize a new punt result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// ```
    pub fn new() -> PuntResult {
        PuntResult::default()
    }

    /// Get a punt result's play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let play_duration = my_res.play_duration();
    /// assert!(play_duration == 0);
    /// ```
    pub fn play_duration(&self) -> u32 {
        self.play_duration
    }

    /// Get a punt result's fumble_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let fumble_return_yards = my_res.fumble_return_yards();
    /// assert!(fumble_return_yards == 0);
    /// ```
    pub fn fumble_return_yards(&self) -> i32 {
        self.fumble_return_yards
    }

    /// Get a punt result's punt_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let punt_yards = my_res.punt_yards();
    /// assert!(punt_yards == 0);
    /// ```
    pub fn punt_yards(&self) -> i32 {
        self.punt_yards
    }

    /// Get a punt result's punt_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let punt_return_yards = my_res.punt_return_yards();
    /// assert!(punt_return_yards == 0);
    /// ```
    pub fn punt_return_yards(&self) -> i32 {
        self.punt_return_yards
    }

    /// Get a punt result's blocked property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let blocked = my_res.blocked();
    /// assert!(!blocked);
    /// ```
    pub fn blocked(&self) -> bool {
        self.blocked
    }

    /// Get a punt result's touchback property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let touchback = my_res.touchback();
    /// assert!(!touchback);
    /// ```
    pub fn touchback(&self) -> bool {
        self.touchback
    }

    /// Get a punt result's out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let out_of_bounds = my_res.out_of_bounds();
    /// assert!(!out_of_bounds);
    /// ```
    pub fn out_of_bounds(&self) -> bool {
        self.out_of_bounds
    }

    /// Get a punt result's fair_catch property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let fair_catch = my_res.fair_catch();
    /// assert!(!fair_catch);
    /// ```
    pub fn fair_catch(&self) -> bool {
        self.fair_catch
    }

    /// Get a punt result's muffed property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let muffed = my_res.muffed();
    /// assert!(!muffed);
    /// ```
    pub fn muffed(&self) -> bool {
        self.muffed
    }

    /// Get a punt result's fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let fumble = my_res.fumble();
    /// assert!(!fumble);
    /// ```
    pub fn fumble(&self) -> bool {
        self.fumble
    }

    /// Get a punt result's touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_res = PuntResult::new();
    /// let touchdown = my_res.touchdown();
    /// assert!(!touchdown);
    /// ```
    pub fn touchdown(&self) -> bool {
        self.touchdown
    }
}

/// # `PuntResultBuilder` struct
///
/// A `PuntResultBuilder` is a builder pattern implementation for the
/// `PuntResult` struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct PuntResultBuilder {
    fumble_return_yards: i32,
    punt_yards: i32,
    punt_return_yards: i32,
    play_duration: u32,
    blocked: bool,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    muffed: bool,
    fumble: bool,
    touchdown: bool
}

impl Default for PuntResultBuilder {
    /// Default constructor for the PuntResultBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::default();
    /// ```
    fn default() -> Self {
        PuntResultBuilder{
            fumble_return_yards: 0,
            punt_yards: 0,
            punt_return_yards: 0,
            play_duration: 0,
            blocked: false,
            touchback: false,
            out_of_bounds: false,
            fair_catch: false,
            muffed: false,
            fumble: false,
            touchdown: false
        }
    }
}

impl PuntResultBuilder {
    /// Initialize a new PuntResultBuilder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_builder = PuntResultBuilder::new();
    /// ```
    pub fn new() -> PuntResultBuilder {
        PuntResultBuilder::default()
    }

    /// Set the play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .play_duration(10)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.play_duration() == 10);
    /// ```
    pub fn play_duration(mut self, play_duration: u32) -> Self {
        self.play_duration = play_duration;
        self
    }

    /// Set the fumble_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .fumble(true)
    ///     .fumble_return_yards(4)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fumble_return_yards() == 4);
    /// ```
    pub fn fumble_return_yards(mut self, fumble_return_yards: i32) -> Self {
        self.fumble_return_yards = fumble_return_yards;
        self
    }

    /// Set the punt_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .punt_yards(41)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.punt_yards() == 41);
    /// ```
    pub fn punt_yards(mut self, punt_yards: i32) -> Self {
        self.punt_yards = punt_yards;
        self
    }

    /// Set the punt_return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .punt_return_yards(8)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.punt_return_yards() == 8);
    /// ```
    pub fn punt_return_yards(mut self, punt_return_yards: i32) -> Self {
        self.punt_return_yards = punt_return_yards;
        self
    }

    /// Set the blocked property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .blocked(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.blocked());
    /// ```
    pub fn blocked(mut self, blocked: bool) -> Self {
        self.blocked = blocked;
        self
    }

    /// Set the touchback property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .touchback(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.touchback());
    /// ```
    pub fn touchback(mut self, touchback: bool) -> Self {
        self.touchback = touchback;
        self
    }

    /// Set the out_of_bounds property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .out_of_bounds(true)
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
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .fair_catch(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.fair_catch());
    /// ```
    pub fn fair_catch(mut self, fair_catch: bool) -> Self {
        self.fair_catch = fair_catch;
        self
    }

    /// Set the muffed property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .fumble(true)
    ///     .muffed(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.muffed());
    /// ```
    pub fn muffed(mut self, muffed: bool) -> Self {
        self.muffed = muffed;
        self
    }

    /// Set the fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
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
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .touchdown(true)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_result.touchdown());
    /// ```
    pub fn touchdown(mut self, touchdown: bool) -> Self {
        self.touchdown = touchdown;
        self
    }

    /// Build the PuntResult
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultBuilder;
    /// 
    /// let my_result = PuntResultBuilder::new()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<PuntResult, String> {
        let raw = PuntResultRaw{
            fumble_return_yards: self.fumble_return_yards,
            punt_yards: self.punt_yards,
            punt_return_yards: self.punt_return_yards,
            play_duration: self.play_duration,
            blocked: self.blocked,
            touchback: self.touchback,
            out_of_bounds: self.out_of_bounds,
            fair_catch: self.fair_catch,
            muffed: self.muffed,
            fumble: self.fumble,
            touchdown: self.touchdown
        };
        PuntResult::try_from(raw)
    }
}

/// # `PuntResultSimulator` struct
///
/// A `PuntResultSimulator` represents a simulator which can produce a result of a punt play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PuntResultSimulator {}

impl PuntResultSimulator {
    /// Initialize a new PuntResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResultSimulator;
    ///
    /// let my_sim = PuntResultSimulator::new();
    /// ```
    pub fn new() -> PuntResultSimulator {
        PuntResultSimulator{}
    }

    /// Generates whether the punt was blocked
    fn blocked(&self, norm_diff_blocking: f64, rng: &mut impl Rng) -> bool {
        let p_block: f64 = 1_f64.min(0_f64.max(P_BLOCK_INTR + (P_BLOCK_COEF * norm_diff_blocking)));
        rng.gen::<f64>() < p_block
    }

    /// Generates whether the punt landed inside the 20 yard line
    fn inside_20(&self, norm_punting: f64, yard_line: i32, rng: &mut impl Rng) -> bool {
        let p_inside_20_skill: f64 = P_PUNT_INSIDE_20_SKILL_INTR + (P_PUNT_INSIDE_20_SKILL_COEF * norm_punting);
        let p_inside_20_yardline: f64 = P_PUNT_INSIDE_20_YARD_LINE_PARAM_1 / (
            1_f64 + (
                -P_PUNT_INSIDE_20_YARD_LINE_PARAM_2 * (
                    yard_line as f64 - P_PUNT_INSIDE_20_YARD_LINE_PARAM_3
                )
            ).exp() + P_PUNT_INSIDE_20_YARD_LINE_PARAM_4
        ); // Logistic curve fit
        let p_inside_20: f64 = 1_f64.min(0_f64.max(
            ((p_inside_20_skill * 0.5) + (p_inside_20_yardline * 0.5)) * 1.18
        )); // Weighted average
        rng.gen::<f64>() < p_inside_20
    }

    /// Generates the distance of the punt
    fn distance(&self, yard_line: i32, punt_inside_20: bool, rng: &mut impl Rng) -> i32 {
        let mean_rel_dist: f64 = if punt_inside_20 {
            PUNT_INSIDE_20_MEAN_REL_DIST_INTR + (PUNT_INSIDE_20_MEAN_REL_DIST_COEF * yard_line as f64)
        } else {
            PUNT_OUTSIDE_20_MEAN_REL_DIST_INTR + (PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_1 * yard_line as f64) +
                (PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_2 * yard_line.pow(2) as f64) +
                (PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_3 * yard_line.pow(3) as f64)
        };
        let std_rel_dist: f64 = if punt_inside_20 {
            PUNT_INSIDE_20_STD_REL_DIST_INTR + (PUNT_INSIDE_20_STD_REL_DIST_COEF * yard_line as f64)
        } else {
            PUNT_OUTSIDE_20_STD_REL_DIST_INTR + (PUNT_OUTSIDE_20_STD_REL_DIST_COEF * yard_line as f64)
        };
        let skew_rel_dist: f64 = if punt_inside_20 {
            PUNT_INSIDE_20_SKEW_REL_DIST_INTR + (PUNT_INSIDE_20_SKEW_REL_DIST_COEF_1 * yard_line as f64) +
                (PUNT_INSIDE_20_SKEW_REL_DIST_COEF_2 * yard_line.pow(2) as f64)
        } else {
            PUNT_OUTSIDE_20_SKEW_REL_DIST_INTR + (PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_1 * yard_line as f64) +
                (PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_2 * yard_line.pow(2) as f64)
        };
        let rel_dist_dist = SkewNormal::new(mean_rel_dist, std_rel_dist, skew_rel_dist).unwrap();
        let rel_dist: f64 = rel_dist_dist.sample(rng);
        let new_yard_line: f64 = yard_line as f64 * rel_dist;
        let punt_distance: i32 = yard_line - new_yard_line as i32;
        punt_distance
    }

    /// Generates whether the punt went out of bounds
    fn out_of_bounds(&self, yard_line: i32, rng: &mut impl Rng) -> bool {
        let p_oob: f64 = 1_f64.min(0_f64.max(
            P_PUNT_OOB_INTR + (P_PUNT_OOB_COEF_1 * yard_line as f64) +
                (P_PUNT_OOB_COEF_2 * yard_line.pow(2) as f64)
        ));
        rng.gen::<f64>() < p_oob
    }

    /// Generates whether a fair catch was called on the punt
    fn fair_catch(&self, yard_line: i32, rng: &mut impl Rng) -> bool {
        let p_fair_catch: f64 = 1_f64.min(0_f64.max(
            P_FAIR_CATCH_INTR + (P_FAIR_CATCH_COEF * yard_line as f64)
        ));
        rng.gen::<f64>() < p_fair_catch
    }

    /// Generates whether the punt was muffed
    fn muffed(&self, norm_diff_returning: f64, rng: &mut impl Rng) -> bool {
        let p_muffed_punt: f64 = 1_f64.min(0_f64.max(
            P_MUFFED_PUNT_INTR + (P_MUFFED_PUNT_COEF * norm_diff_returning)
        ));
        rng.gen::<f64>() < p_muffed_punt
    }

    /// Generates the punt return yards
    fn return_yards(&self, landing_yard_line: i32, norm_diff_returning: f64, rng: &mut impl Rng) -> i32 {
        let mean_rel_return_yards: f64 = MEAN_REL_RETURN_YARDS_INTR + (MEAN_REL_RETURN_YARDS_COEF_1 * norm_diff_returning) +
            (MEAN_REL_RETURN_YARDS_COEF_2 * norm_diff_returning.powi(2));
        let std_rel_return_yards: f64 = STD_REL_RETURN_YARDS_INTR + (STD_REL_RETURN_YARDS_COEF_1 * norm_diff_returning) +
            (STD_REL_RETURN_YARDS_COEF_2 * norm_diff_returning.powi(2));
        let skew_rel_return_yards: f64 = SKEW_REL_RETURN_YARDS_INTR + (SKEW_REL_RETURN_YARDS_COEF_1 * norm_diff_returning) +
            (SKEW_REL_RETURN_YARDS_COEF_2 * norm_diff_returning.powi(2));
        let rel_return_yards_dist = SkewNormal::new(mean_rel_return_yards, std_rel_return_yards, skew_rel_return_yards).unwrap();
        let rel_return_yards: f64 = rel_return_yards_dist.sample(rng);
        let return_yards: i32 = (landing_yard_line as f64 * rel_return_yards) as i32;
        return_yards
    }

    /// Generates whether a fumble occurred on the punt return
    fn fumble(&self, norm_diff_returning: f64, rng: &mut impl Rng) -> bool {
        let p_fumble: f64 = 1_f64.min(0_f64.max(
            P_FUMBLE_INTR + (P_FUMBLE_COEF * norm_diff_returning)
        ));
        rng.gen::<f64>() < p_fumble
    }

    /// Generates fumble return yards
    fn fumble_return_yards(&self, rng: &mut impl Rng) -> i32 {
        Exp::new(1_f64).unwrap().sample(rng).round() as i32
    }

    /// Generates the duration of the punt play
    fn play_duration(&self, total_yards: u32, rng: &mut impl Rng) -> u32 {
        let mean_duration: f64 = PUNT_PLAY_DURATION_INTR + (PUNT_PLAY_DURATION_COEF * total_yards as f64);
        let duration_dist = Normal::new(mean_duration, 2_f64).unwrap();
        u32::try_from(duration_dist.sample(rng).round() as i32).unwrap_or_default()
    }
}

impl PlayResultSimulator for PuntResultSimulator {
    /// Simulate a punt play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::punt::PuntResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a punt play simulator and simulate a play
    /// let my_sim = PuntResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult {
        // Calculate normalized skill levels and skill diffs
        let offense_advantage: bool = context.offense_advantage();
        let defense_advantage: bool = context.defense_advantage();
        let norm_diff_blocking: f64 = 0.5_f64 + (
            (
                offense.offense().blocking_advantage(offense_advantage) as f64 -
                defense.defense().blitzing_advantage(defense_advantage) as f64
            ) / 200_f64
        );
        let norm_diff_returning: f64 = 0.5_f64 + (
            (
                defense.defense().kick_returning_advantage(defense_advantage) as f64 -
                offense.offense().kick_return_defense_advantage(offense_advantage) as f64
            ) / 200_f64
        );
        let norm_punting: f64 = offense.offense().punting_advantage(offense_advantage) as f64 / 100_f64;
        let td_yards: i32 = context.yards_to_touchdown();
        
        // Generate whether the punt was blocked
        let blocked: bool = self.blocked(norm_diff_blocking, rng);

        // Generate whether the punt landed inside the 20
        let inside_20: bool = if !blocked {
            self.inside_20(norm_punting, td_yards, rng)
        } else {
            false
        };

        // Generate the distance of the punt
        let punt_distance: i32 = if !blocked {
            self.distance(td_yards, inside_20, rng)
        } else {
            0
        };
        let punt_landing: i32 = 100.min(0.max(td_yards - punt_distance));
        let touchback: bool = punt_landing <= 0;

        // Generate whether the punt went out of bounds
        let out_of_bounds: bool = if !(blocked || touchback) {
            self.out_of_bounds(td_yards, rng)
        } else {
            false
        };

        // Generate whether a fair catch was called
        let fair_catch: bool = if !(blocked || out_of_bounds || touchback) {
            self.fair_catch(punt_landing, rng)
        } else {
            false
        };

        // Generate whether the punt was muffed
        let punt_muffed: bool = if !(blocked || out_of_bounds || touchback) {
            self.muffed(norm_diff_returning, rng)
        } else {
            false
        };

        // Generate the punt return yards
        let punt_return_yards: i32 = if !(blocked || fair_catch || out_of_bounds || touchback || punt_muffed) {
            (100 - punt_landing).min(self.return_yards(100 - punt_landing, norm_diff_returning, rng))
        } else {
            0
        };

        // Determine if a punt return touchdown occurred
        let mut touchdown: bool = if !(blocked || out_of_bounds || touchback || punt_muffed) {
            (punt_landing + punt_return_yards) >= 100
        } else {
            false
        };

        // Generate whether a fumble occurred
        let fumble: bool = if punt_muffed {
            true
        } else if !(blocked || out_of_bounds || touchback || touchdown) {
            self.fumble(norm_diff_returning, rng)
        } else {
            false
        };

        // Generate the fumble return yards
        let fumble_return_yards: i32 = if fumble {
            self.fumble_return_yards(rng)
        } else {
            0
        };

        // Determine if a fumble recovery touchdown occurred
        touchdown = if fumble {
            punt_landing + punt_return_yards - fumble_return_yards <= 0
        } else {
            touchdown
        };

        // Calculate total yardage and play duration
        let total_yards: u32 = punt_distance.unsigned_abs() + punt_return_yards.unsigned_abs() + fumble_return_yards.unsigned_abs();
        let play_duration: u32 = self.play_duration(total_yards, rng);
        let raw = PuntResultRaw{
            fumble_return_yards,
            punt_yards: punt_distance,
            punt_return_yards,
            play_duration,
            blocked,
            touchback,
            out_of_bounds,
            fair_catch,
            muffed: punt_muffed,
            fumble,
            touchdown
        };
        let punt_res = PuntResult::try_from(raw).unwrap();
        PlayTypeResult::Punt(punt_res)
    }
}
