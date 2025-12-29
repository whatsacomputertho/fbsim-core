#![doc = include_str!("../../../docs/game/play/result.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod betweenplay;
pub mod fieldgoal;
pub mod kickoff;
pub mod pass;
pub mod punt;
pub mod run;

use rand::Rng;

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;
use crate::game::play::result::betweenplay::BetweenPlayResult;
use crate::game::play::result::fieldgoal::FieldGoalResult;
use crate::game::play::result::kickoff::KickoffResult;
use crate::game::play::result::pass::PassResult;
use crate::game::play::result::punt::PuntResult;
use crate::game::play::result::run::RunResult;

/// # `PlayResult` trait
///
/// The `PlayResult` trait defines the necessary methods in order to
/// update a `GameContext` after a play is complete
pub trait PlayResult {
    fn next_context(&self, context: &GameContext) -> GameContext where Self: Sized { context.next_context(self) }
    fn play_duration(&self) -> u32 { 0 }
    fn net_yards(&self) -> i32 { 0 }
    fn turnover(&self) -> bool { false }
    fn offense_score(&self) -> ScoreResult { ScoreResult::None }
    fn defense_score(&self) -> ScoreResult { ScoreResult::None }
    fn offense_timeout(&self) -> bool { false }
    fn defense_timeout(&self) -> bool { false }
    fn incomplete(&self) -> bool { false }
    fn out_of_bounds(&self) -> bool { false }
    fn touchback(&self) -> bool { false }
    fn kickoff(&self) -> bool { false }
    fn punt(&self) -> bool { false }
    fn next_play_kickoff(&self) -> bool { false }
    fn next_play_extra_point(&self) -> bool { false }
}

/// # `PlayTypeResult` enum
///
/// The `PlayTypeResult` enum is used to store the result of an arbirary play
/// for gathering game statistics
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PlayTypeResult {
    BetweenPlay(BetweenPlayResult),
    Run(RunResult),
    Pass(PassResult),
    FieldGoal(FieldGoalResult),
    Punt(PuntResult),
    Kickoff(KickoffResult),
    ExtraPoint(FieldGoalResult),
    QbKneel(RunResult),
    QbSpike(PassResult)
}

impl std::fmt::Display for PlayTypeResult {
    /// Format a `PlayTypeResult` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::play::result::PlayTypeResult;
    /// use fbsim_core::game::play::result::run::RunResult;
    /// 
    /// let my_result = PlayTypeResult::Run(RunResult::default());
    /// println!("{}", my_result);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.fmt(f),
            PlayTypeResult::Run(res) => res.fmt(f),
            PlayTypeResult::Pass(res) => res.fmt(f),
            PlayTypeResult::FieldGoal(res) => res.fmt(f),
            PlayTypeResult::Punt(res) => res.fmt(f),
            PlayTypeResult::Kickoff(res) => res.fmt(f),
            PlayTypeResult::ExtraPoint(res) => res.fmt(f),
            PlayTypeResult::QbKneel(res) => res.fmt(f),
            PlayTypeResult::QbSpike(res) => res.fmt(f)
        }
    }
}

impl PlayResult for PlayTypeResult {
    fn next_context(&self, context: &GameContext) -> GameContext where Self: Sized {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.next_context(context),
            PlayTypeResult::Run(res) => res.next_context(context),
            PlayTypeResult::Pass(res) => res.next_context(context),
            PlayTypeResult::FieldGoal(res) => res.next_context(context),
            PlayTypeResult::Punt(res) => res.next_context(context),
            PlayTypeResult::Kickoff(res) => res.next_context(context),
            PlayTypeResult::ExtraPoint(res) => res.next_context(context),
            PlayTypeResult::QbKneel(res) => res.next_context(context),
            PlayTypeResult::QbSpike(res) => res.next_context(context)
        }
    }

    fn play_duration(&self) -> u32 {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.play_duration(),
            PlayTypeResult::Run(res) => res.play_duration(),
            PlayTypeResult::Pass(res) => res.play_duration(),
            PlayTypeResult::FieldGoal(res) => res.play_duration(),
            PlayTypeResult::Punt(res) => res.play_duration(),
            PlayTypeResult::Kickoff(res) => res.play_duration(),
            PlayTypeResult::ExtraPoint(res) => res.play_duration(),
            PlayTypeResult::QbKneel(res) => res.play_duration(),
            PlayTypeResult::QbSpike(res) => res.play_duration()
        }
    }

    fn net_yards(&self) -> i32 {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.net_yards(),
            PlayTypeResult::Run(res) => res.net_yards(),
            PlayTypeResult::Pass(res) => res.net_yards(),
            PlayTypeResult::FieldGoal(res) => res.net_yards(),
            PlayTypeResult::Punt(res) => res.net_yards(),
            PlayTypeResult::Kickoff(res) => res.net_yards(),
            PlayTypeResult::ExtraPoint(res) => res.net_yards(),
            PlayTypeResult::QbKneel(res) => res.net_yards(),
            PlayTypeResult::QbSpike(res) => res.net_yards()
        }
    }

    fn turnover(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.turnover(),
            PlayTypeResult::Run(res) => res.turnover(),
            PlayTypeResult::Pass(res) => res.turnover(),
            PlayTypeResult::FieldGoal(res) => res.turnover(),
            PlayTypeResult::Punt(res) => res.turnover(),
            PlayTypeResult::Kickoff(res) => res.turnover(),
            PlayTypeResult::ExtraPoint(res) => res.turnover(),
            PlayTypeResult::QbKneel(res) => res.turnover(),
            PlayTypeResult::QbSpike(res) => res.turnover()
        }
    }

    fn offense_score(&self) -> ScoreResult {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.offense_score(),
            PlayTypeResult::Run(res) => res.offense_score(),
            PlayTypeResult::Pass(res) => res.offense_score(),
            PlayTypeResult::FieldGoal(res) => res.offense_score(),
            PlayTypeResult::Punt(res) => res.offense_score(),
            PlayTypeResult::Kickoff(res) => res.offense_score(),
            PlayTypeResult::ExtraPoint(res) => res.offense_score(),
            PlayTypeResult::QbKneel(res) => res.offense_score(),
            PlayTypeResult::QbSpike(res) => res.offense_score()
        }
    }

    fn defense_score(&self) -> ScoreResult {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.defense_score(),
            PlayTypeResult::Run(res) => res.defense_score(),
            PlayTypeResult::Pass(res) => res.defense_score(),
            PlayTypeResult::FieldGoal(res) => res.defense_score(),
            PlayTypeResult::Punt(res) => res.defense_score(),
            PlayTypeResult::Kickoff(res) => res.defense_score(),
            PlayTypeResult::ExtraPoint(res) => res.defense_score(),
            PlayTypeResult::QbKneel(res) => res.defense_score(),
            PlayTypeResult::QbSpike(res) => res.defense_score()
        }
    }

    fn offense_timeout(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.offense_timeout(),
            PlayTypeResult::Run(res) => res.offense_timeout(),
            PlayTypeResult::Pass(res) => res.offense_timeout(),
            PlayTypeResult::FieldGoal(res) => res.offense_timeout(),
            PlayTypeResult::Punt(res) => res.offense_timeout(),
            PlayTypeResult::Kickoff(res) => res.offense_timeout(),
            PlayTypeResult::ExtraPoint(res) => res.offense_timeout(),
            PlayTypeResult::QbKneel(res) => res.offense_timeout(),
            PlayTypeResult::QbSpike(res) => res.offense_timeout()
        }
    }

    fn defense_timeout(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.defense_timeout(),
            PlayTypeResult::Run(res) => res.defense_timeout(),
            PlayTypeResult::Pass(res) => res.defense_timeout(),
            PlayTypeResult::FieldGoal(res) => res.defense_timeout(),
            PlayTypeResult::Punt(res) => res.defense_timeout(),
            PlayTypeResult::Kickoff(res) => res.defense_timeout(),
            PlayTypeResult::ExtraPoint(res) => res.defense_timeout(),
            PlayTypeResult::QbKneel(res) => res.defense_timeout(),
            PlayTypeResult::QbSpike(res) => res.defense_timeout()
        }
    }

    fn incomplete(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.incomplete(),
            PlayTypeResult::Run(res) => res.incomplete(),
            PlayTypeResult::Pass(res) => res.incomplete(),
            PlayTypeResult::FieldGoal(res) => res.incomplete(),
            PlayTypeResult::Punt(res) => res.incomplete(),
            PlayTypeResult::Kickoff(res) => res.incomplete(),
            PlayTypeResult::ExtraPoint(res) => res.incomplete(),
            PlayTypeResult::QbKneel(res) => res.incomplete(),
            PlayTypeResult::QbSpike(res) => res.incomplete()
        }
    }

    fn out_of_bounds(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.out_of_bounds(),
            PlayTypeResult::Run(res) => res.out_of_bounds(),
            PlayTypeResult::Pass(res) => res.out_of_bounds(),
            PlayTypeResult::FieldGoal(res) => res.out_of_bounds(),
            PlayTypeResult::Punt(res) => res.out_of_bounds(),
            PlayTypeResult::Kickoff(res) => res.out_of_bounds(),
            PlayTypeResult::ExtraPoint(res) => res.out_of_bounds(),
            PlayTypeResult::QbKneel(res) => res.out_of_bounds(),
            PlayTypeResult::QbSpike(res) => res.out_of_bounds()
        }
    }

    fn touchback(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.touchback(),
            PlayTypeResult::Run(res) => res.touchback(),
            PlayTypeResult::Pass(res) => res.touchback(),
            PlayTypeResult::FieldGoal(res) => res.touchback(),
            PlayTypeResult::Punt(res) => res.touchback(),
            PlayTypeResult::Kickoff(res) => res.touchback(),
            PlayTypeResult::ExtraPoint(res) => res.touchback(),
            PlayTypeResult::QbKneel(res) => res.touchback(),
            PlayTypeResult::QbSpike(res) => res.touchback()
        }
    }

    fn kickoff(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.kickoff(),
            PlayTypeResult::Run(res) => res.kickoff(),
            PlayTypeResult::Pass(res) => res.kickoff(),
            PlayTypeResult::FieldGoal(res) => res.kickoff(),
            PlayTypeResult::Punt(res) => res.kickoff(),
            PlayTypeResult::Kickoff(res) => res.kickoff(),
            PlayTypeResult::ExtraPoint(res) => res.kickoff(),
            PlayTypeResult::QbKneel(res) => res.kickoff(),
            PlayTypeResult::QbSpike(res) => res.kickoff()
        }
    }

    fn punt(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.punt(),
            PlayTypeResult::Run(res) => res.punt(),
            PlayTypeResult::Pass(res) => res.punt(),
            PlayTypeResult::FieldGoal(res) => res.punt(),
            PlayTypeResult::Punt(res) => res.punt(),
            PlayTypeResult::Kickoff(res) => res.punt(),
            PlayTypeResult::ExtraPoint(res) => res.punt(),
            PlayTypeResult::QbKneel(res) => res.punt(),
            PlayTypeResult::QbSpike(res) => res.punt()
        }
    }

    fn next_play_kickoff(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.next_play_kickoff(),
            PlayTypeResult::Run(res) => res.next_play_kickoff(),
            PlayTypeResult::Pass(res) => res.next_play_kickoff(),
            PlayTypeResult::FieldGoal(res) => res.next_play_kickoff(),
            PlayTypeResult::Punt(res) => res.next_play_kickoff(),
            PlayTypeResult::Kickoff(res) => res.next_play_kickoff(),
            PlayTypeResult::ExtraPoint(res) => res.next_play_kickoff(),
            PlayTypeResult::QbKneel(res) => res.next_play_kickoff(),
            PlayTypeResult::QbSpike(res) => res.next_play_kickoff()
        }
    }

    fn next_play_extra_point(&self) -> bool {
        match self {
            PlayTypeResult::BetweenPlay(res) => res.next_play_extra_point(),
            PlayTypeResult::Run(res) => res.next_play_extra_point(),
            PlayTypeResult::Pass(res) => res.next_play_extra_point(),
            PlayTypeResult::FieldGoal(res) => res.next_play_extra_point(),
            PlayTypeResult::Punt(res) => res.next_play_extra_point(),
            PlayTypeResult::Kickoff(res) => res.next_play_extra_point(),
            PlayTypeResult::ExtraPoint(res) => res.next_play_extra_point(),
            PlayTypeResult::QbKneel(res) => res.next_play_extra_point(),
            PlayTypeResult::QbSpike(res) => res.next_play_extra_point()
        }
    }
}

/// `PlayResultSimulator` trait
///
/// Defines the sim function which is implemented by each play type simulator.
/// This trait returns a `PlayTypeResult` trait which is used to update the
/// `GameContext` after a play is simulated.
pub trait PlayResultSimulator {
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> PlayTypeResult;
}

/// `ScoreResult` enum
///
/// Enumerates the various ways a team can score points in football
#[derive(PartialEq, Clone, Copy, Eq, Ord, PartialOrd, Debug, Default)]
pub enum ScoreResult {
    #[default] None,
    ExtraPoint,
    TwoPointConversion,
    Safety,
    FieldGoal,
    Touchdown
}

impl ScoreResult {
    /// Get the point value of the result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::ScoreResult;
    ///
    /// let my_result = ScoreResult::FieldGoal;
    /// assert!(my_result.points() == 3);
    /// ```
    pub fn points(&self) -> u32 {
        match self {
            ScoreResult::None => 0,
            ScoreResult::ExtraPoint => 1,
            ScoreResult::TwoPointConversion => 2,
            ScoreResult::Safety => 2,
            ScoreResult::FieldGoal => 3,
            ScoreResult::Touchdown => 6
        }
    }
}
