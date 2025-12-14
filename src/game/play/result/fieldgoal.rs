use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use rand_distr::{Distribution, Exp, SkewNormal};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::{PlayResult, PlayResultSimulator, ScoreResult};

// Field goal blocked skill-based regression
const P_BLOCKED_SKILL_INTR: f64 = 0.013200206956159479_f64;
const P_BLOCKED_SKILL_COEF: f64 = 0.01919733_f64;

// Field goal blocked yard-line-based regression
const P_BLOCKED_YARD_LINE_INTR: f64 = -5.320426815163247_f64;
const P_BLOCKED_YARD_LINE_COEF: f64 = 0.05875677_f64;

// Field goal made skill-based regression
const P_FIELD_GOAL_MADE_SKILL_INTR: f64 = 0.44298810053776055_f64;
const P_FIELD_GOAL_MADE_SKILL_COEF: f64 = 0.57103524_f64;

// Field goal made yard-line-based regression
const P_FIELD_GOAL_MADE_YARD_LINE_INTR: f64 = 0.9580405463949037_f64;
const P_FIELD_GOAL_MADE_YARD_LINE_COEF_1: f64 = 0.00399668_f64;
const P_FIELD_GOAL_MADE_YARD_LINE_COEF_2: f64 = -0.00035704_f64;

// Field goal blocked duration distribution parameters
const FIELD_GOAL_BLOCKED_DURATION_MEAN: f64 = 6.843750_f64;
const FIELD_GOAL_BLOCKED_DURATION_STD: f64 = 3.385612_f64;
const FIELD_GOAL_BLOCKED_DURATION_SKEW: f64 = 1.541247_f64;

// Field goal not blocked duration distribution parameters
const FIELD_GOAL_NOT_BLOCKED_DURATION_MEAN: f64 = 4.054470_f64;
const FIELD_GOAL_NOT_BLOCKED_DURATION_STD: f64 = 1.001211_f64;
const FIELD_GOAL_NOT_BLOCKED_DURATION_SKEW: f64 = -0.440028_f64;

/// # `FieldGoalResult` struct
///
/// A `FieldGoalResult` represents a result of a field goal
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct FieldGoalResult {
    field_goal_distance: i32,
    return_yards: i32,
    play_duration: u32,
    made: bool,
    blocked: bool,
    touchdown: bool,
    extra_point: bool
}

impl Default for FieldGoalResult {
    /// Default constructor for the FieldGoalResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::fieldgoal::FieldGoalResult;
    /// 
    /// let my_result = FieldGoalResult::default();
    /// ```
    fn default() -> Self {
        FieldGoalResult{
            field_goal_distance: 12,
            return_yards: 0,
            play_duration: 0,
            made: true,
            blocked: false,
            touchdown: false,
            extra_point: true
        }
    }
}

impl std::fmt::Display for FieldGoalResult {
    /// Format a `FieldGoalResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::fieldgoal::FieldGoalResult;
    /// 
    /// let my_result = FieldGoalResult::default();
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let distance_str = format!("{} yard", self.field_goal_distance);
        let fg_type_str = if self.extra_point {
            "extra point"
        } else {
            "field goal"
        };
        let result_str = if self.made {
            "is good."
        } else if self.blocked {
            "BLOCKED."
        } else {
            "NO GOOD."
        };
        let return_str = if self.blocked {
            let return_prefix = format!("Returned {} yards", self.return_yards);
            if self.touchdown {
                format!("{}, TOUCHDOWN!", &return_prefix)
            } else {
                format!("{}.", return_prefix)
            }
        } else {
            String::from("")
        };
        let fg_str = format!(
            "{} {} {} {}",
            &distance_str,
            fg_type_str,
            result_str,
            &return_str
        );
        f.write_str(&fg_str.trim())
    }
}

impl PlayResult for FieldGoalResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        -self.return_yards
    }

    fn turnover(&self) -> bool {
        !self.extra_point && self.blocked
    }

    fn offense_score(&self) -> ScoreResult {
        if self.extra_point && self.made {
            return ScoreResult::ExtraPoint;
        } else if self.made {
            return ScoreResult::FieldGoal;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.extra_point && self.blocked && self.touchdown {
            return ScoreResult::TwoPointConversion;
        } else if self.blocked && self.touchdown {
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn offense_timeout(&self) -> bool { false }

    fn defense_timeout(&self) -> bool { false }

    fn incomplete(&self) -> bool { false }

    fn out_of_bounds(&self) -> bool { false }

    fn kickoff(&self) -> bool { false }

    fn next_play_kickoff(&self) -> bool {
        self.extra_point || self.made
    }

    fn next_play_extra_point(&self) -> bool {
        !self.extra_point && self.blocked && self.touchdown
    }

    fn summary(&self) -> String {
        format!("{}", self)
    }
}

/// # `FieldGoalResultSimulator` struct
///
/// A `FieldGoalResultSimulator` represents a simulator which can produce a result of a field goal
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FieldGoalResultSimulator {}

impl FieldGoalResultSimulator {
    /// Initialize a new FieldGoalResultSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::fieldgoal::FieldGoalResultSimulator;
    ///
    /// let my_sim = FieldGoalResultSimulator::new();
    /// ```
    pub fn new() -> FieldGoalResultSimulator {
        FieldGoalResultSimulator{}
    }

    /// Generate whether the field goal was blocked
    fn blocked(&self, norm_diff_blocking: f64, yard_line: i32, rng: &mut impl Rng) -> bool {
        let p_blocked_skill: f64 = P_BLOCKED_SKILL_INTR + (P_BLOCKED_SKILL_COEF * norm_diff_blocking);
        let p_blocked_yardline: f64 = (P_BLOCKED_YARD_LINE_INTR + (P_BLOCKED_YARD_LINE_COEF * yard_line as f64)).exp();
        let p_blocked: f64 = 1_f64.min(0_f64.max(
            0.7_f64 * ((p_blocked_skill * 0.7_f64) + (p_blocked_yardline * 0.3_f64))
        ));
        rng.gen::<f64>() < p_blocked
    }

    /// Generate the field goal block return yards
    fn return_yards(&self, rng: &mut impl Rng) -> i32 {
        Exp::new(1_f64).unwrap().sample(rng).round() as i32
    }

    /// Generate whether the field goal was made
    fn made(&self, norm_kicking: f64, yard_line: i32, rng: &mut impl Rng) -> bool {
        let p_made_skill: f64 = P_FIELD_GOAL_MADE_SKILL_INTR + (P_FIELD_GOAL_MADE_SKILL_COEF * norm_kicking);
        let p_made_yardline: f64 = P_FIELD_GOAL_MADE_YARD_LINE_INTR + (P_FIELD_GOAL_MADE_YARD_LINE_COEF_1 * yard_line as f64) +
            (P_FIELD_GOAL_MADE_YARD_LINE_COEF_2 * yard_line.pow(2) as f64);
        let p_made: f64 = 1_f64.min(0_f64.max(
            1.18_f64 * ((p_made_skill * 0.4_f64) + (p_made_yardline * 0.6_f64))
        ));
        rng.gen::<f64>() < p_made
    }

    /// Generate the duration of the field goal play
    fn play_duration(&self, is_blocked: bool, rng: &mut impl Rng) -> u32 {
        let duration_dist = if is_blocked {
            SkewNormal::new(FIELD_GOAL_BLOCKED_DURATION_MEAN, FIELD_GOAL_BLOCKED_DURATION_STD, FIELD_GOAL_BLOCKED_DURATION_SKEW).unwrap()
        } else {
            SkewNormal::new(FIELD_GOAL_NOT_BLOCKED_DURATION_MEAN, FIELD_GOAL_NOT_BLOCKED_DURATION_STD, FIELD_GOAL_NOT_BLOCKED_DURATION_SKEW).unwrap()
        };
        match u32::try_from(duration_dist.sample(rng).round() as i32) {
            Ok(n) => n,
            Err(_) => 0
        }
    }
}

impl PlayResultSimulator for FieldGoalResultSimulator {
    /// Simulate a field goal
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::result::PlayResultSimulator;
    /// use fbsim_core::game::play::result::fieldgoal::FieldGoalResultSimulator;
    ///
    /// // Initialize home & away teams
    /// let my_off = FootballTeam::new();
    /// let my_def = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a field goal simulator and simulate a play
    /// let my_sim = FieldGoalResultSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_res = my_sim.sim(&my_off, &my_def, &my_context, &mut rng);
    /// ```
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> impl PlayResult {
        // Calculate normalized skill levels and skill diffs
        let norm_diff_blocking: f64 = 0.5_f64 + ((offense.offense().blocking() as f64 - defense.defense().blitzing() as f64) / 200_f64);
        let norm_kicking: f64 = offense.offense().field_goals() as f64 / 100_f64;
        let td_yards: i32 = context.yards_to_touchdown();
        let safety_yards: i32 = context.yards_to_safety();

        // Generate whether the field goal was blocked
        let blocked: bool = self.blocked(norm_diff_blocking, td_yards, rng);

        // Generate field goal block return yards
        let return_yards: i32 = if blocked {
            self.return_yards(rng)
        } else {
            0
        };

        // Generate whether the field goal was made
        let made: bool = if !blocked {
            self.made(norm_kicking, td_yards, rng)
        } else {
            false
        };

        // Generate the duration of the play in seconds
        let play_duration: u32 = self.play_duration(blocked, rng);

        // Determine if a touchdown occurred
        let touchdown: bool = blocked && (return_yards > safety_yards.abs());
        FieldGoalResult{
            field_goal_distance: td_yards + 10,
            return_yards: return_yards,
            play_duration: play_duration,
            made: made,
            blocked: blocked,
            touchdown: touchdown,
            extra_point: *context.next_play_extra_point()
        }
    }
}
