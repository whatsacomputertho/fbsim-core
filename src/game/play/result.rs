pub mod betweenplay;
pub mod fieldgoal;
pub mod kickoff;
pub mod pass;
pub mod punt;
pub mod run;

use rand::Rng;

use crate::game::context::GameContext;
use crate::game::play::PlaySimulatable;

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
    fn next_play_kickoff(&self) -> bool { false }
    fn next_play_extra_point(&self) -> bool { false }
    fn summary(&self) -> String { String::from("Not yet implemented") }
}

pub trait PlayResultSimulator {
    fn sim(&self, offense: &impl PlaySimulatable, defense: &impl PlaySimulatable, context: &GameContext, rng: &mut impl Rng) -> impl PlayResult;
}

#[derive(PartialEq, Clone, Copy)]
pub enum ScoreResult {
    None,
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
