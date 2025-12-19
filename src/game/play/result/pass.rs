use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use rand_distr::{Normal, Distribution, Exp, SkewNormal};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayTypeResult, PlayResultSimulator, ScoreResult};

// Pressure probability regression
const P_PRESSURE_INTR: f64 = 0.271330308819705_f64;
const P_PRESSURE_COEF: f64 = -0.21949841_f64;

// Sack probability regression
const P_SACK_INTR: f64 = 0.10898853099029118_f64;
const P_SACK_COEF: f64 = -0.08144463_f64;

// Sack yards lost distribution
const MEAN_SACK_YARDS: f64 = 6.703931;
const STD_SACK_YARDS: f64 = 3.640892;

// Scramble probability regression
const P_SCRAMBLE_INTR: f64 = 0.004914770911025865_f64;
const P_SCRAMBLE_COEF: f64 = 0.13433329_f64;

// Mean scramble yards regression
const MEAN_SCRAMBLE_YARDS_INTR: f64 = 6.313938741503718_f64;
const MEAN_SCRAMBLE_YARDS_COEF: f64 = 1.61219979_f64;

// Std scramble yards regression
const STD_SCRAMBLE_YARDS_INTR: f64 = 4.974662775900808_f64;
const STD_SCRAMBLE_YARDS_COEF: f64 = 2.92020782_f64;

// Skew scramble yards regression
const SKEW_SCRAMBLE_YARDS_INTR: f64 = 4.836766323216999_f64;
const SKEW_SCRAMBLE_YARDS_COEF_1: f64 = -12.22272275_f64;
const SKEW_SCRAMBLE_YARDS_COEF_2: f64 = 11.66478691_f64;

// Short pass probability regression
const P_SHORT_PASS_INTR: f64 = 0.8010555875020549_f64; // Adjusted -0.04
const P_SHORT_PASS_COEF_1: f64 = -0.0054862949_f64;
const P_SHORT_PASS_COEF_2: f64 = 0.000050472999_f64;

// Mean short pass distance regression
const MEAN_SHORT_PASS_DIST_INTR: f64 = 3.4999015440062564_f64;
const MEAN_SHORT_PASS_DIST_COEF_1: f64 = 0.0604532760_f64;
const MEAN_SHORT_PASS_DIST_COEF_2: f64 = -0.00118944537_f64;
const MEAN_SHORT_PASS_DIST_COEF_3: f64 = 0.00000662934811_f64;

// Std short pass distance regression
const STD_SHORT_PASS_DIST_INTR: f64 = 3.365933454906047_f64;
const STD_SHORT_PASS_DIST_COEF_1: f64 = 0.130891269_f64;
const STD_SHORT_PASS_DIST_COEF_2: f64 = -0.00237804912_f64;
const STD_SHORT_PASS_DIST_COEF_3: f64 = 0.0000127875476_f64;

// Mean deep pass distance regression
const MEAN_DEEP_PASS_DIST_INTR: f64 = 2.405519456054698_f64;
const MEAN_DEEP_PASS_DIST_COEF_1: f64 = 1.23979494_f64;
const MEAN_DEEP_PASS_DIST_COEF_2: f64 = -0.0204279438_f64;
const MEAN_DEEP_PASS_DIST_COEF_3: f64 = 0.000106455687_f64;

// Std deep pass distance regression
const STD_DEEP_PASS_DIST_INTR: f64 = -1.3385882641162565_f64;
const STD_DEEP_PASS_DIST_COEF_1: f64 = 0.277596854_f64;
const STD_DEEP_PASS_DIST_COEF_2: f64 = -0.00120030840_f64;
const STD_DEEP_PASS_DIST_COEF_3: f64 = -0.00000553839342_f64;

// Interception probability regression
const P_INTERCEPTION_INTR: f64 = 0.05628420712097409_f64;
const P_INTERCEPTION_COEF: f64 = -0.06021105_f64;

// Mean interception return yards regression
const MEAN_INT_RETURN_YARDS_INTR: f64 = 11.952396063360451_f64;
const MEAN_INT_RETURN_YARDS_COEF_1: f64 = 0.134680678_f64;
const MEAN_INT_RETURN_YARDS_COEF_2: f64 = -0.00176264090_f64;
const MEAN_INT_RETURN_YARDS_COEF_3: f64 = -0.00000170755614_f64;

// Std interception return yards regression
const STD_INT_RETURN_YARDS_INTR: f64 = 27.359295307597726_f64;
const STD_INT_RETURN_YARDS_COEF_1: f64 = -0.298495830_f64;
const STD_INT_RETURN_YARDS_COEF_2: f64 = 0.00302760757_f64;
const STD_INT_RETURN_YARDS_COEF_3: f64 = -0.0000206954185_f64;

// Skew interception return yards regression
const SKEW_INT_RETURN_YARDS_INTR: f64 = 2.4745876927563324_f64;
const SKEW_INT_RETURN_YARDS_COEF_1: f64 = -0.00592938387_f64;
const SKEW_INT_RETURN_YARDS_COEF_2: f64 = -0.000720407529_f64;
const SKEW_INT_RETURN_YARDS_COEF_3: f64 = 0.00000700818986_f64;

// Completed pass probability regression
const P_COMPLETE_INTR: f64 = 0.6353317321473931_f64;
const P_COMPLETE_COEF: f64 = 0.09651794_f64;

// Completed pass probability regression (pass distance based)
const P_COMPLETE_DIST_INTR: f64 = 0.7706457923470589_f64;
const P_COMPLETE_DIST_COEF: f64 = -0.00670494_f64;

// Zero yards after catch probability regression
const P_ZERO_YAC_INTR: f64 = 0.46761265601223527_f64; // Adjusted + 0.3
const P_ZERO_YAC_COEF: f64 = -0.06038915_f64;

// Mean yards after catch regression
const MEAN_YAC_INTR: f64 = 3.744998660966435_f64;
const MEAN_YAC_COEF_1: f64 = 2.21147177_f64;
const MEAN_YAC_COEF_2: f64 = 2.36122192_f64;

// Std yards after catch regression
const STD_YAC_INTR: f64 = 5.404781207922575_f64;
const STD_YAC_COEF_1: f64 = 0.28690679_f64;
const STD_YAC_COEF_2: f64 = 5.88666152_f64;

// Skew yards after catch regression
const SKEW_YAC_INTR: f64 = 3.0784534230008083_f64;
const SKEW_YAC_COEF: f64 = -0.10326043_f64;

// Fumble probability
const P_FUMBLE: f64 = 0.1_f64;

// Mean play duration regression
const MEAN_PLAY_DURATION_INTR: f64 = 5.32135821_f64;
const MEAN_PLAY_DURATION_COEF_1: f64 = 0.11343699_f64;
const MEAN_PLAY_DURATION_COEF_2: f64 = -0.00056798_f64;

/// # `PassResult` struct
///
/// A `PassResult` represents a result of a pass play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct PassResult {
    play_duration: u32,
    sack_yards_lost: i32,
    scramble_yards: i32,
    pass_dist: i32,
    return_yards: i32,
    yards_after_catch: i32,
    pressure: bool,
    sack: bool,
    scramble: bool,
    interception: bool,
    complete: bool,
    fumble: bool,
    touchdown: bool,
    safety: bool
}

impl Default for PassResult {
    /// Default constructor for the PassResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_result = PassResult::default();
    /// ```
    fn default() -> Self {
        PassResult{
            play_duration: 0,
            sack_yards_lost: 0,
            scramble_yards: 0,
            pass_dist: 0,
            return_yards: 0,
            yards_after_catch: 0,
            pressure: false,
            sack: false,
            scramble: false,
            interception: false,
            complete: false,
            fumble: false,
            touchdown: false,
            safety: false
        }
    }
}

impl std::fmt::Display for PassResult {
    /// Format a `PassResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_result = PassResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pressure_str = if self.pressure {
            let pressure_prefix = "Defense brings pressure";
            if self.sack {
                format!("{}, QB SACKED for loss of {} yards.", pressure_prefix, self.sack_yards_lost)
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };
        let scramble_str = if self.scramble {
            format!(" QB scrambles for {} yards", self.scramble_yards)
        } else {
            String::from("")
        };
        let catch_str = if !self.pressure || (self.pressure && !(self.sack || self.scramble)) {
            let pass_prefix = format!(" Pass {} yards", self.pass_dist);
            if self.interception {
                format!("{} INTERCEPTED, returned {} yards.", pass_prefix, self.return_yards)
            } else if self.complete {
                format!("{} complete for gain of {}.", pass_prefix, self.pass_dist + self.yards_after_catch)
            } else {
                format!("{} incomplete.", pass_prefix)
            }
        } else {
            String::from("")
        };
        let fumble_str = if self.scramble || (self.complete && self.fumble) {
            format!(" FUMBLE recovered by the defense, returned {} yards", self.return_yards)
        } else {
            String::from("")
        };
        let score_str = if self.touchdown {
            " TOUCHDOWN!"
        } else if self.safety {
            " SAFETY!"
        } else {
            ""
        };
        let pass_str = format!(
            "{}{}{}{}{}",
            &pressure_str,
            &scramble_str,
            &catch_str,
            &fumble_str,
            score_str
        );
        f.write_str(&pass_str.trim())
    }
}

impl PlayResult for PassResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        if self.complete {
            self.pass_dist + self.yards_after_catch - (self.return_yards + self.sack_yards_lost)
        } else {
            self.scramble_yards - (self.return_yards + self.sack_yards_lost)
        }
    }

    fn turnover(&self) -> bool {
        self.fumble || self.interception
    }

    fn offense_score(&self) -> ScoreResult {
        if self.touchdown && !(self.fumble || self.interception) {
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.touchdown && (self.fumble || self.interception) {
            ScoreResult::Touchdown
        } else if self.safety {
            ScoreResult::Safety
        } else {
            ScoreResult::None
        }
    }

    fn offense_timeout(&self) -> bool { false }

    fn defense_timeout(&self) -> bool { false }

    fn incomplete(&self) -> bool {
        !(self.complete || self.sack || self.scramble)
    }

    fn out_of_bounds(&self) -> bool { false }

    fn kickoff(&self) -> bool { false }

    fn next_play_kickoff(&self) -> bool {
        self.safety
    }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown
    }
}

impl PassResult {
    /// Initialize a new pass result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// ```
    pub fn new() -> PassResult {
        PassResult::default()
    }

    /// Get a pass result's play_duration property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let play_duration = my_res.play_duration();
    /// assert!(play_duration == 0);
    /// ```
    pub fn play_duration(&self) -> u32 {
        self.play_duration
    }

    /// Get a pass result's sack_yards_lost property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let sack_yards_lost = my_res.sack_yards_lost();
    /// assert!(sack_yards_lost == 0);
    /// ```
    pub fn sack_yards_lost(&self) -> i32 {
        self.sack_yards_lost
    }

    /// Get a pass result's scramble yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let scramble_yards = my_res.scramble_yards();
    /// assert!(scramble_yards == 0);
    /// ```
    pub fn scramble_yards(&self) -> i32 {
        self.scramble_yards
    }

    /// Get a pass result's pass_dist property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let pass_dist = my_res.pass_dist();
    /// assert!(pass_dist == 0);
    /// ```
    pub fn pass_dist(&self) -> i32 {
        self.pass_dist
    }

    /// Get a pass result's return_yards property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let return_yards = my_res.return_yards();
    /// assert!(return_yards == 0);
    /// ```
    pub fn return_yards(&self) -> i32 {
        self.return_yards
    }

    /// Get a pass result's yards_after_catch property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let yards_after_catch = my_res.yards_after_catch();
    /// assert!(yards_after_catch == 0);
    /// ```
    pub fn yards_after_catch(&self) -> i32 {
        self.yards_after_catch
    }

    /// Get a pass result's pressure property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let pressure = my_res.pressure();
    /// assert!(!pressure);
    /// ```
    pub fn pressure(&self) -> bool {
        self.pressure
    }

    /// Get a pass result's sack property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let sack = my_res.sack();
    /// assert!(!sack);
    /// ```
    pub fn sack(&self) -> bool {
        self.sack
    }

    /// Get a pass result's scramble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let scramble = my_res.scramble();
    /// assert!(!scramble);
    /// ```
    pub fn scramble(&self) -> bool {
        self.scramble
    }

    /// Get a pass result's interception property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let interception = my_res.interception();
    /// assert!(!interception);
    /// ```
    pub fn interception(&self) -> bool {
        self.interception
    }

    /// Get a pass result's complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let complete = my_res.complete();
    /// assert!(!complete);
    /// ```
    pub fn complete(&self) -> bool {
        self.complete
    }

    /// Get a pass result's fumble property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let fumble = my_res.fumble();
    /// assert!(!fumble);
    /// ```
    pub fn fumble(&self) -> bool {
        self.fumble
    }

    /// Get a pass result's touchdown property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let touchdown = my_res.touchdown();
    /// assert!(!touchdown);
    /// ```
    pub fn touchdown(&self) -> bool {
        self.touchdown
    }

    /// Get a pass result's safety property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// 
    /// let my_res = PassResult::new();
    /// let safety = my_res.safety();
    /// assert!(!safety);
    /// ```
    pub fn safety(&self) -> bool {
        self.safety
    }
}

/// # `PassResultSimulator` struct
///
/// A `PassResultSimulator` represents a simulator which can produce a result of a pass play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PassResultSimulator {}

impl PassResultSimulator {
    /// Initialize a new PassResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::pass::PassResultSimulator;
    ///
    /// let my_sim = PassResultSimulator::new();
    /// ```
    pub fn new() -> PassResultSimulator {
        PassResultSimulator{}
    }

    /// Generates whether the quarterback was under pressure
    fn pressure(&self, norm_diff_blocking: f64, rng: &mut impl Rng) -> bool {
        let p_pressure: f64 = 1_f64.min(0_f64.max(P_PRESSURE_INTR + (P_PRESSURE_COEF * norm_diff_blocking)));
        rng.gen::<f64>() < p_pressure
    }

    /// Generates whether the quarterback was sacked while under pressure
    fn sack(&self, norm_diff_blocking: f64, rng: &mut impl Rng) -> bool {
        let p_sack: f64 = 1_f64.min(0_f64.max(P_SACK_INTR + (P_SACK_COEF * norm_diff_blocking)));
        rng.gen::<f64>() < p_sack
    }

    fn sack_yards_lost(&self, rng: &mut impl Rng) -> i32 {
        Normal::new(MEAN_SACK_YARDS, STD_SACK_YARDS).unwrap().sample(rng).round() as i32
    }

    /// Generates whether the quarterback scrambled while under pressure
    fn scramble(&self, norm_scrambling: f64, rng: &mut impl Rng) -> bool {
        let p_scramble: f64 = 1_f64.min(0_f64.max(P_SCRAMBLE_INTR + (P_SCRAMBLE_COEF * norm_scrambling)));
        rng.gen::<f64>() < p_scramble
    }

    fn scramble_yards(&self, norm_diff_scrambling: f64, rng: &mut impl Rng) -> i32 {
        let mean_scramble_yards: f64 = MEAN_SCRAMBLE_YARDS_INTR + (MEAN_SCRAMBLE_YARDS_COEF * norm_diff_scrambling);
        let std_scramble_yards: f64 = STD_SCRAMBLE_YARDS_INTR + (STD_SCRAMBLE_YARDS_COEF * norm_diff_scrambling);
        let skew_scramble_yards: f64 = SKEW_SCRAMBLE_YARDS_INTR + (SKEW_SCRAMBLE_YARDS_COEF_1 * norm_diff_scrambling) + (SKEW_SCRAMBLE_YARDS_COEF_2 * norm_diff_scrambling.powi(2));
        let scramble_yards_dist = SkewNormal::new(mean_scramble_yards, std_scramble_yards, skew_scramble_yards).unwrap();
        scramble_yards_dist.sample(rng).round() as i32
    }

    /// Generates whether the quarterback threw a short pass
    fn short_pass(&self, yard_line: u32, rng: &mut impl Rng) -> bool {
        let p_short_pass: f64 = 1_f64.min(0_f64.max(
            P_SHORT_PASS_INTR + (P_SHORT_PASS_COEF_1 * yard_line as f64) + (P_SHORT_PASS_COEF_2 * yard_line.pow(2) as f64)
        ));
        rng.gen::<f64>() < p_short_pass
    }

    /// Generates the distance of a short pass
    fn short_pass_distance(&self, yard_line: u32, rng: &mut impl Rng) -> i32 {
        let mean_short_pass_dist: f64 = MEAN_SHORT_PASS_DIST_INTR + (MEAN_SHORT_PASS_DIST_COEF_1 * yard_line as f64) + (MEAN_SHORT_PASS_DIST_COEF_2 * yard_line.pow(2) as f64) + (MEAN_SHORT_PASS_DIST_COEF_3 * yard_line.pow(3) as f64);
        let std_short_pass_dist: f64 = STD_SHORT_PASS_DIST_INTR + (STD_SHORT_PASS_DIST_COEF_1 * yard_line as f64) + (STD_SHORT_PASS_DIST_COEF_2 * yard_line.pow(2) as f64) + (STD_SHORT_PASS_DIST_COEF_3 * yard_line.pow(3) as f64);
        let short_pass_dist = Normal::new(mean_short_pass_dist, std_short_pass_dist).unwrap();
        (short_pass_dist.sample(rng).round() as i32).max(-2)
    }

    /// Generates the distance of a deep pass
    fn deep_pass_distance(&self, yard_line: u32, rng: &mut impl Rng) -> i32 {
        let mean_deep_pass_dist: f64 = MEAN_DEEP_PASS_DIST_INTR + (MEAN_DEEP_PASS_DIST_COEF_1 * yard_line as f64) + (MEAN_DEEP_PASS_DIST_COEF_2 * yard_line.pow(2) as f64) + (MEAN_DEEP_PASS_DIST_COEF_3 * yard_line.pow(3) as f64);
        let std_deep_pass_dist: f64 = STD_DEEP_PASS_DIST_INTR + (STD_DEEP_PASS_DIST_COEF_1 * yard_line as f64) + (STD_DEEP_PASS_DIST_COEF_2 * yard_line.pow(2) as f64) + (STD_DEEP_PASS_DIST_COEF_3 * yard_line.pow(3) as f64);
        let deep_pass_dist = Normal::new(mean_deep_pass_dist, std_deep_pass_dist).unwrap();
        deep_pass_dist.sample(rng).round() as i32
    }

    /// Generates whether the quarterback threw an interception
    fn interception(&self, norm_diff_turnovers: f64, rng: &mut impl Rng) -> bool {
        let p_interception: f64 = 1_f64.min(0_f64.max(P_INTERCEPTION_INTR + (P_INTERCEPTION_COEF * norm_diff_turnovers)));
        rng.gen::<f64>() < p_interception
    }

    /// Generates the interception return yards
    fn interception_return_yards(&self, yard_line: u32, rng: &mut impl Rng) -> i32 {
        let mean_int_return_yards: f64 = MEAN_INT_RETURN_YARDS_INTR + (MEAN_INT_RETURN_YARDS_COEF_1 * yard_line as f64) + (MEAN_INT_RETURN_YARDS_COEF_2 * yard_line.pow(2) as f64) + (MEAN_INT_RETURN_YARDS_COEF_3 * yard_line.pow(3) as f64);
        let std_int_return_yards: f64 = STD_INT_RETURN_YARDS_INTR + (STD_INT_RETURN_YARDS_COEF_1 * yard_line as f64) + (STD_INT_RETURN_YARDS_COEF_2 * yard_line.pow(2) as f64) + (STD_INT_RETURN_YARDS_COEF_3 * yard_line.pow(3) as f64);
        let skew_int_return_yards: f64 = SKEW_INT_RETURN_YARDS_INTR + (SKEW_INT_RETURN_YARDS_COEF_1 * yard_line as f64) + (SKEW_INT_RETURN_YARDS_COEF_2 * yard_line.pow(2) as f64) + (SKEW_INT_RETURN_YARDS_COEF_3 * yard_line.pow(3) as f64);
        let int_return_dist = SkewNormal::new(mean_int_return_yards, std_int_return_yards, skew_int_return_yards).unwrap();
        int_return_dist.sample(rng).round() as i32
    }

    /// Generates whether the quarterback threw a complete pass
    fn complete(&self, norm_diff_passing: f64, pass_dist: i32, rng: &mut impl Rng) -> bool {
        let p_complete_skill: f64 = P_COMPLETE_INTR + (P_COMPLETE_COEF * norm_diff_passing as f64);
        let p_complete_yl: f64 = P_COMPLETE_DIST_INTR + (P_COMPLETE_DIST_COEF * pass_dist as f64);
        let p_complete: f64 = 0.8_f64.min((
            (
                (
                    ((p_complete_yl * 0.3) + (p_complete_skill * 0.7)).ln() + 1.0
                ).max(0.01).ln() + 1.0
            ).max(0.01).ln() + 1.6
        ).max(0.01));
        rng.gen::<f64>() < p_complete
    }

    /// Generates whether the wide receiver had zero yards after catch
    fn zero_yards_after_catch(&self, norm_diff_receiving: f64, rng: &mut impl Rng) -> bool {
        let p_zero_yac: f64 = 1_f64.min(0_f64.max(P_ZERO_YAC_INTR + (P_ZERO_YAC_COEF * norm_diff_receiving)));
        rng.gen::<f64>() < p_zero_yac
    }

    /// Generates the yards after catch
    fn yards_after_catch(&self, norm_diff_receiving: f64, rng: &mut impl Rng) -> i32 {
        let mean_yac: f64 = MEAN_YAC_INTR + (MEAN_YAC_COEF_1 * norm_diff_receiving) + (MEAN_YAC_COEF_2 * norm_diff_receiving.powi(2));
        let std_yac: f64 = STD_YAC_INTR + (STD_YAC_COEF_1 * norm_diff_receiving) + (STD_YAC_COEF_2 * norm_diff_receiving.powi(2));
        let skew_yac: f64 = SKEW_YAC_INTR + (SKEW_YAC_COEF * norm_diff_receiving);
        let yac_dist = SkewNormal::new(mean_yac, std_yac, skew_yac).unwrap();
        yac_dist.sample(rng).round() as i32
    }

    /// Generates whether a fumble occurred
    fn fumble(&self, rng: &mut impl Rng) -> bool {
        rng.gen::<f64>() < P_FUMBLE
    }

    /// Generates the fumble recovery return yards
    fn fumble_return_yards(&self, rng: &mut impl Rng) -> i32 {
        Exp::new(1_f64).unwrap().sample(rng).round() as i32
    }

    /// Generates the duration of a pass play
    fn play_duration(&self, total_yards: u32, rng: &mut impl Rng) -> u32 {
        let mean_duration: f64 = MEAN_PLAY_DURATION_INTR + (MEAN_PLAY_DURATION_COEF_1 * total_yards as f64) + (MEAN_PLAY_DURATION_COEF_2 * total_yards.pow(2) as f64);
        let duration_dist = Normal::new(mean_duration, 2_f64).unwrap();
        match u32::try_from(duration_dist.sample(rng).round() as i32) {
            Ok(n) => n,
            Err(_) => 0
        }
    }
}

impl PlayResultSimulator for PassResultSimulator {
    /// Simulate a run play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::pass::PassResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a pass play simulator and simulate a play
    /// let my_sim = PassResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult {
        // Derive the normalized skill differentials for each team
        let norm_diff_blocking: f64 = 0.5_f64 + ((offense.offense().blocking() as f64 - defense.defense().blitzing() as f64) / 200_f64);
        let norm_diff_passing: f64 = 0.5_f64 + ((offense.offense().passing() as f64 - defense.defense().pass_defense() as f64) / 200_f64);
        let norm_diff_receiving: f64 = 0.5_f64 + ((offense.offense().receiving() as f64 - defense.defense().coverage() as f64) / 200_f64);
        let norm_diff_turnovers: f64 = 0.5_f64 + ((offense.offense().turnovers() as f64 - defense.defense().turnovers() as f64) / 200_f64);
        let norm_diff_scrambling: f64 = 0.5_f64 + ((offense.offense().scrambling() as f64 - defense.defense().rush_defense() as f64) / 200_f64);
        let norm_scrambling: f64 = offense.offense().scrambling() as f64 / 100_f64;
        let td_yards = context.yards_to_touchdown();
        let yard_line = match u32::try_from(td_yards) {
            Ok(n) => n,
            Err(_) => 0
        };
        let oob_yards = td_yards + 10;
        let safety_yards = context.yards_to_safety();

        // Generate whether a pressure occurred
        let pressure: bool = self.pressure(norm_diff_blocking, rng);

        // Generate whether a sack occurred
        let sack: bool = if pressure {
            self.sack(norm_diff_blocking, rng)
        } else {
            false
        };

        // Generate sack yards lost
        let sack_yards_lost: i32 = if sack {
            self.sack_yards_lost(rng)
        } else {
            0
        };

        // Determine if a safety occurred
        let mut safety: bool = if sack {
            sack_yards_lost * -1 < safety_yards
        } else {
            false
        };

        // Generate whether a scramble occurred
        let scramble: bool = if pressure && !sack {
            self.scramble(norm_scrambling, rng)
        } else {
            false
        };

        // Generate scramble yards if scramble occurred
        let scramble_yards: i32 = if scramble {
            self.scramble_yards(norm_diff_scrambling, rng)
        } else {
            0
        };

        // Generate whether a short pass occurred
        let pass: bool = !pressure || (pressure && !(sack || scramble));
        let short_pass: bool = if pass {
            self.short_pass(yard_line, rng)
        } else {
            false
        };

        // Generate pass distance
        let pass_distance: i32 = if pass && !short_pass {
            self.deep_pass_distance(yard_line, rng)
        } else {
            self.short_pass_distance(yard_line, rng)
        };

        // Generate whether an interception occurred
        let interception: bool = if pass {
            self.interception(norm_diff_turnovers, rng)
        } else {
            false
        };

        // Generate the interception return yards
        let int_return_yards: i32 = if interception {
            self.interception_return_yards(yard_line, rng)
        } else {
            0
        };

        // Check whether an interception return TD occurred
        let mut touchdown: bool = interception && ((int_return_yards * -1) < safety_yards);

        // Generate whether the pass was complete
        let complete: bool = if pass && !interception {
            self.complete(norm_diff_passing, pass_distance, rng)
        } else {
            false
        };

        // Check whether the pass was caught out of bounds or in the end zone
        let out_of_bounds: bool = complete && (pass_distance >= oob_yards);
        touchdown = if !touchdown {
            complete && !out_of_bounds && (pass_distance >= td_yards)
        } else {
            touchdown
        };

        // Generate whether there were yards after the catch
        let zero_yac: bool = if complete {
            self.zero_yards_after_catch(norm_diff_receiving, rng)
        } else {
            false
        };

        // Generate the yards after catch
        let yards_after_catch: i32 = if complete && !touchdown && !zero_yac {
            (td_yards - pass_distance).min(self.yards_after_catch(norm_diff_receiving, rng))
        } else {
            0
        };

        // Check whether the YAC led to a touchdown or safety
        safety = if !safety {
            complete && (pass_distance + yards_after_catch <= safety_yards)
        } else {
            safety
        };
        touchdown = if !touchdown {
            complete && (pass_distance + yards_after_catch >= td_yards)
        } else {
            touchdown
        };

        // Generate whether a fumble occurred
        let fumble: bool = if (scramble || complete) && !touchdown {
            self.fumble(rng)
        } else {
            false
        };

        // Generate fumble return yards
        let fumble_return_yards: i32 = if fumble {
            self.fumble_return_yards(rng)
        } else {
            0
        };

        // Generate return yards, yards gained, play duration
        let return_yards: i32 = int_return_yards + fumble_return_yards;
        let play_duration: u32 = self.play_duration(
            sack_yards_lost.abs() as u32 + pass_distance.abs() as u32 + int_return_yards.abs() as u32 +
            yards_after_catch.abs() as u32 + fumble_return_yards.abs() as u32,
            rng
        );
        let pass_res = PassResult{
            play_duration: play_duration,
            sack_yards_lost: sack_yards_lost,
            scramble_yards: scramble_yards,
            pass_dist: pass_distance,
            return_yards: return_yards,
            yards_after_catch: yards_after_catch,
            pressure: pressure,
            sack: sack,
            scramble: scramble,
            interception: interception,
            complete: complete,
            fumble: fumble,
            touchdown: touchdown,
            safety: safety
        };
        PlayTypeResult::Pass(pass_res)
    }
}
