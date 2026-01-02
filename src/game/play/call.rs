#![doc = include_str!("../../../docs/game/play/call.md")]
use rand::Rng;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::context::PlayContext;

// Run probabilities on 1st-3rd down clock management scenarios
const P_RUN_CLOCK_MANAGEMENT: f64 = 0.15_f64;
const P_RUN_CLOCK_MANAGEMENT_NO_TIMEOUTS: f64 = 0.001_f64;

// Run probability regression on 1st down
const P_RUN_FIRST_DOWN_INTR: f64 = 0.41649529080915104_f64;
const P_RUN_FIRST_DOWN_COEF: f64 = 0.2035597_f64;

// Run probability regression on 2nd down
const P_RUN_SECOND_DOWN_INTR: f64 = 0.3250691394699521_f64;
const P_RUN_SECOND_DOWN_COEF: f64 = 0.19162143_f64;

// Run probability regression on 3rd down
const P_RUN_THIRD_DOWN_INTR: f64 = 0.1340492470213823_f64;
const P_RUN_THIRD_DOWN_COEF: f64 = 0.22902729;

// Run probability regression by distance to first / goal
const P_RUN_DIST_INTR: f64 = 0.30634251685198927_f64;
const P_RUN_DIST_COEF: f64 = -0.00318081_f64;

// Field goal risk-taking-based probability regression on 4th down
const P_FIELD_GOAL_RISK_INTR: f64 = 0.7886141537295228_f64;
const P_FIELD_GOAL_RISK_COEF: f64 = -0.26532936_f64;

// Field goal yard-line-based probability regression on 4th down
const P_FIELD_GOAL_YARD_LINE_INTR: f64 = 0.24354785898372522_f64;
const P_FIELD_GOAL_YARD_LINE_COEF_1: f64 = 0.05165115_f64;
const P_FIELD_GOAL_YARD_LINE_COEF_2: f64 = -0.00112775_f64;

// Go for it probability by risk taking
const P_GO_FOR_IT_INTR: f64 = 0.19565011246401598_f64;
const P_GO_FOR_IT_COEF: f64 = 0.51602604_f64;

// Run probability regression on 4th down
const P_RUN_FOURTH_DOWN_INTR: f64 = 0.040592196833718536_f64;
const P_RUN_FOURTH_DOWN_COEF: f64 = 0.05793641_f64;

/// # `PlayCall` enum
///
/// Defines the various types of plays that can be run in football
#[derive(PartialEq, Clone, Copy)]
pub enum PlayCall {
    Run,
    Pass,
    FieldGoal,
    Punt,
    Kickoff,
    ExtraPoint,
    QbKneel,
    QbSpike
}

/// # `PlayCallSimulator` struct
///
/// A `PlayCallSimulator` generates a play call given a game scenario and coach
/// attributes
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PlayCallSimulator {}

impl PlayCallSimulator {
    /// Initialize a new PlayCallSimulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::call::PlayCallSimulator;
    ///
    /// let my_sim = PlayCallSimulator::new();
    /// ```
    pub fn new() -> PlayCallSimulator {
        PlayCallSimulator{}
    }

    /// Generate the play call for the last play
    fn last_play_playcall(&self, context: &PlayContext, rng: &mut impl Rng) -> PlayCall {
        if context.last_play_need_td() {
            return PlayCall::Pass;
        }
        let yard_line = context.yard_line();
        let p_field_goal_yardline: f64 = 1_f64.min(0_f64.max(
            0.2_f64 + P_FIELD_GOAL_YARD_LINE_INTR + (P_FIELD_GOAL_YARD_LINE_COEF_1 * yard_line as f64) +
                (P_FIELD_GOAL_YARD_LINE_COEF_2 * yard_line.pow(2) as f64)
        )); // Adjust by +0.2 to incentivize going for field goals if a field goal is all that is needed
        if rng.gen::<f64>() < p_field_goal_yardline {
            return PlayCall::FieldGoal;
        }
        PlayCall::Pass
    }

    /// Generate the play call for a clock management scenario
    fn conserve_clock_playcall(&self, context: &PlayContext, rng: &mut impl Rng) -> PlayCall {
        let p_run = if context.offense_timeouts() > 0 {
            P_RUN_CLOCK_MANAGEMENT
        } else {
            P_RUN_CLOCK_MANAGEMENT_NO_TIMEOUTS
        };
        if rng.gen::<f64>() < p_run {
            return PlayCall::Run;
        }
        PlayCall::Pass
    }

    /// Generate the play call for a non-clock management scenario on 1st-3rd
    fn normal_play_call(&self, context: &PlayContext, run_pass: f64, rng: &mut impl Rng) -> PlayCall {
        let down = context.down();
        let distance = context.distance();
        let p_run_call: f64 = match down {
            1 => P_RUN_FIRST_DOWN_INTR + (P_RUN_FIRST_DOWN_COEF * run_pass),
            2 => P_RUN_SECOND_DOWN_INTR + (P_RUN_SECOND_DOWN_COEF * run_pass),
            3 => P_RUN_THIRD_DOWN_INTR + (P_RUN_THIRD_DOWN_COEF * run_pass),
            4 => P_RUN_FOURTH_DOWN_INTR + (P_RUN_FOURTH_DOWN_COEF * run_pass),
            _ => P_RUN_SECOND_DOWN_INTR + (P_RUN_SECOND_DOWN_COEF * run_pass)
        };
        let p_run_dist: f64 = P_RUN_DIST_INTR + (P_RUN_DIST_COEF * distance as f64);
        let p_run: f64 = 1_f64.min(0_f64.max(
            (p_run_dist * 0.3_f64) + (p_run_call * 0.7_f64)
        ));
        if rng.gen::<f64>() < p_run {
            return PlayCall::Run;
        }
        PlayCall::Pass
    }

    /// Generate the play call for fourth down
    fn fourth_down_play_call(&self, context: &PlayContext, risk_taking: f64, run_pass: f64, rng: &mut impl Rng) -> PlayCall {
        let in_field_goal_range: bool = context.in_field_goal_range();
        let go_for_it_scenario: bool = context.can_go_for_it();
        if !(in_field_goal_range || go_for_it_scenario) {
            return PlayCall::Punt;
        }

        // Calculate go for it & field goal probabilities
        let yard_line = context.yard_line();
        let p_go_for_it: f64 = 1_f64.min(0_f64.max(
            P_GO_FOR_IT_INTR + (P_GO_FOR_IT_COEF * risk_taking)
        ));
        let p_field_goal_risk: f64 = P_FIELD_GOAL_RISK_INTR + (P_FIELD_GOAL_RISK_COEF * risk_taking);
        let p_field_goal_yardline: f64 = P_FIELD_GOAL_YARD_LINE_INTR + (P_FIELD_GOAL_YARD_LINE_COEF_1 * yard_line as f64) +
            (P_FIELD_GOAL_YARD_LINE_COEF_2 * yard_line.pow(2) as f64);
        let p_field_goal: f64 = 0.9999_f64.min(
            0_f64.max(
                (
                    (p_field_goal_risk * 0.7_f64) + (p_field_goal_yardline * 0.3_f64)
                ).max(0.0001).ln() * 0.8
            ) * 1.6 + 0.0001
        );

        // Go for it scenario
        if go_for_it_scenario {
            if rng.gen::<f64>() < p_field_goal && in_field_goal_range {
                return PlayCall::FieldGoal;
            }
            if yard_line <= 20 || rng.gen::<f64>() < p_go_for_it {
                return self.normal_play_call(context, run_pass, rng);
            }
        }

        // Otherwise field goal if in range, or punt if not in range
        if in_field_goal_range {
            PlayCall::FieldGoal
        } else {
            PlayCall::Punt
        }
    }

    /// Generate a play call
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::call::PlayCallSimulator;
    ///
    /// // Initialize offensive team
    /// let my_off = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Generate a play call
    /// let my_sim = PlayCallSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let my_play_call = my_sim.sim(&my_off, &my_context, &mut rng);
    /// ```
    pub fn sim(&self, offense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayCall {
        // Compute normalized skill levels and context
        let norm_risk_taking: f64 = offense.coach().risk_taking() as f64 / 100_f64;
        let norm_run_pass: f64 = offense.coach().run_pass() as f64 / 100_f64;
        let extra_point = context.next_play_extra_point();
        let play_context = PlayContext::from(context);

        // Extra point playcalling
        if extra_point {
            if play_context.two_point_conversion() {
                return self.normal_play_call(&play_context, norm_run_pass, rng);
            } else {
                return PlayCall::ExtraPoint;
            }
        }

        // Fourth down playcalling
        if play_context.down() == 4 {
            if play_context.must_score() {
                return self.last_play_playcall(&play_context, rng);
            }
            return self.fourth_down_play_call(&play_context, norm_risk_taking, norm_run_pass, rng);
        }

        // Clock management situation playcalling
        if play_context.offense_conserve_clock() {
            if play_context.last_play() {
                return self.last_play_playcall(&play_context, rng);
            }
            return self.conserve_clock_playcall(&play_context, rng);
        }

        self.normal_play_call(&play_context, norm_run_pass, rng)
    }
}
