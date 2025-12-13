use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use rand_distr::{Normal, Distribution, Exp};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayResultSimulator, ScoreResult};

// Mean & std regression for standard rushing play
const MEAN_YARDS_INTR: f64 = 3.0503791522871384_f64;
const MEAN_YARDS_COEF: f64 = 0.32550597_f64;
const STD_YARDS_INTR: f64 = 4.053915588534795_f64;
const STD_YARDS_COEF_1: f64 = 0.2487578_f64;
const STD_YARDS_COEF_2: f64 = 0.0593874_f64;

// Mean & std regression for big, non-TD rushing play
const MEAN_BP_YARDS_INTR: f64 = 15.781025340879893_f64;
const MEAN_BP_YARDS_COEF: f64 = 6.32805521_f64;
const STD_BP_YARDS_INTR: f64 = 10.014877063200005_f64;
const STD_BP_YARDS_COEF_1: f64 = -3.82403981_f64;
const STD_BP_YARDS_COEF_2: f64 = 7.60215528_f64;

// Mean regresion for play duration
const MEAN_DURATION_INTR: f64 = 5.32135821_f64;
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
const P_FUMBLE_COEF: f64 = -0.05432772;

/// # `RunResult` struct
///
/// A `RunResult` represents a result of a run play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct RunResult {
    yards_gained: i32,
    play_duration: u32,
    fumble: bool,
    return_yards: i32,
    out_of_bounds: bool,
    touchdown: bool,
    safety: bool,
    scramble: bool
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
            scramble: false
        }
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
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.touchdown && self.fumble {
            ScoreResult::Touchdown
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
        self.safety
    }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown
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
        let p_big_play: f64 = 1_f64.min(0_f64.max(P_BP_INTR + (P_BP_COEF * norm_diff_rushing)));
        rng.gen::<f64>() < p_big_play
    }

    /// Generates whether this is a big play touchdown
    fn big_play_touchdown(&self, norm_diff_rushing: f64, rng: &mut impl Rng) -> bool {
        let p_bp_td: f64 = 1_f64.min(0_f64.max(P_BP_TD_INTR + (P_BP_TD_COEF * norm_diff_rushing)));
        rng.gen::<f64>() < p_bp_td
    }

    /// Generates the duration of the play
    fn play_duration(&self, total_yards: u32, rng: &mut impl Rng) -> u32 {
        let mean_duration: f64 = MEAN_DURATION_INTR + (MEAN_DURATION_COEF_1 * total_yards as f64) + (MEAN_DURATION_COEF_2 * total_yards.pow(2) as f64);
        let duration_dist = Normal::new(mean_duration, 2_f64).unwrap();
        match u32::try_from(duration_dist.sample(rng).round() as i32) {
            Ok(n) => n,
            Err(_) => 0
        }
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
        let p_fumble: f64 = 1_f64.min(0_f64.max(P_FUMBLE_INTR + (P_FUMBLE_COEF * norm_diff_turnovers)));
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
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> impl PlayResult {
        // Derive the normalized skill differentials for each team
        let norm_diff_rushing: f64 = 0.5_f64 + ((offense.offense().rushing() as f64 - defense.defense().rush_defense() as f64) / 200_f64);
        let norm_diff_turnovers: f64 = 0.5_f64 + ((offense.offense().turnovers() as f64 - defense.defense().turnovers() as f64) / 200_f64);
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
        let total_yards: u32 = yards_gained.abs() as u32 + return_yards.abs() as u32;
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
        RunResult{
            yards_gained: yards_gained,
            play_duration: self.play_duration(total_yards, rng),
            fumble: fumble,
            return_yards: return_yards,
            out_of_bounds: false,
            touchdown: touchdown,
            safety: safety,
            scramble: false
        }
    }
}
