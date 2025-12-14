pub mod call;
pub mod context;
pub mod result;

use rand::Rng;

use crate::game::context::GameContext;
use crate::game::play::call::{PlayCallSimulator, PlayCall};
use crate::game::play::result::{PlayResultSimulator, PlayResult};
use crate::game::play::result::betweenplay::BetweenPlayResultSimulator;
use crate::game::play::result::fieldgoal::FieldGoalResultSimulator;
use crate::game::play::result::kickoff::KickoffResultSimulator;
use crate::game::play::result::punt::PuntResultSimulator;
use crate::game::play::result::pass::PassResultSimulator;
use crate::game::play::result::run::RunResultSimulator;
use crate::team::FootballTeam;
use crate::team::coach::FootballTeamCoach;
use crate::team::defense::FootballTeamDefense;
use crate::team::offense::FootballTeamOffense;

pub trait PlaySimulatable {
    fn coach(&self) -> &FootballTeamCoach;
    fn defense(&self) -> &FootballTeamDefense;
    fn offense(&self) -> &FootballTeamOffense;
}

/// # `Play` struct
///
/// A `Play` represents the outcome of a play
pub struct Play {
    context: GameContext,
    summary: String
}

impl Play {
    /// Initialize a new play
    ///
    /// ```
    /// use fbsim_core::game::play::Play;
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a play
    /// let my_play = Play::new(my_context, "Team A rushes for 10 yards");
    /// ```
    pub fn new(context: GameContext, summary: &str) -> Play {
        Play{
            context: context,
            summary: String::from(summary)
        }
    }
}

/// # `PlaySimulator` struct
///
/// A `PlaySimulator` can simulate a play given a context, returning an
/// updated context
pub struct PlaySimulator {
    betweenplay: BetweenPlayResultSimulator,
    fieldgoal: FieldGoalResultSimulator,
    kickoff: KickoffResultSimulator,
    pass: PassResultSimulator,
    punt: PuntResultSimulator,
    run: RunResultSimulator,
    playcall: PlayCallSimulator
}

impl PlaySimulator {
    /// Initialize a new play simulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::PlaySimulator;
    /// 
    /// // Initialize a play simulator
    /// let my_sim = PlaySimulator::new();
    /// ```
    pub fn new() -> PlaySimulator {
        PlaySimulator{
            betweenplay: BetweenPlayResultSimulator::new(),
            fieldgoal: FieldGoalResultSimulator::new(),
            kickoff: KickoffResultSimulator::new(),
            pass: PassResultSimulator::new(),
            punt: PuntResultSimulator::new(),
            run: RunResultSimulator::new(),
            playcall: PlayCallSimulator::new()
        }
    }

    /// Simulate a play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::PlaySimulator;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a play simulator and simulate a play
    /// let my_sim = PlaySimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let (play, new_context) = my_sim.sim(&my_home, &my_away, my_context, &mut rng);
    /// ```
    pub fn sim(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, rng: &mut impl Rng) -> (Play, GameContext) {
        // Determine the play call
        let play_call = if *context.next_play_extra_point() {
            PlayCall::ExtraPoint
        } else if *context.next_play_kickoff() {
            PlayCall::Kickoff
        } else {
            if *context.home_possession() {
                self.playcall.sim(home, &context, rng)
            } else {
                self.playcall.sim(away, &context, rng)
            }
        };

        // Simulate the play
        let (next_context, summary) = match play_call {
            PlayCall::Run => {
                let res = if *context.home_possession() {
                    self.run.sim(home, away, &context, rng)
                } else {
                    self.run.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::Pass => {
                let res = if *context.home_possession() {
                    self.pass.sim(home, away, &context, rng)
                } else {
                    self.pass.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::FieldGoal => {
                let res = if *context.home_possession() {
                    self.fieldgoal.sim(home, away, &context, rng)
                } else {
                    self.fieldgoal.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::Punt => {
                let res = if *context.home_possession() {
                    self.punt.sim(home, away, &context, rng)
                } else {
                    self.punt.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::Kickoff => {
                let res = if *context.home_possession() {
                    self.kickoff.sim(home, away, &context, rng)
                } else {
                    self.kickoff.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::ExtraPoint => {
                let res = if *context.home_possession() {
                    self.fieldgoal.sim(home, away, &context, rng)
                } else {
                    self.fieldgoal.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::QbKneel => { // TODO: Implement
                let res = if *context.home_possession() {
                    self.run.sim(home, away, &context, rng)
                } else {
                    self.run.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            },
            PlayCall::QbSpike => { // TODO: Implement
                let res = if *context.home_possession() {
                    self.pass.sim(home, away, &context, rng)
                } else {
                    self.pass.sim(away, home, &context, rng)
                };
                let next_context = res.next_context(&context);
                let summary = res.summary();
                (next_context, summary)
            }
        };

        // Simulate between plays
        let between_res = if *context.home_possession() {
            self.betweenplay.sim(home, away, &next_context, rng)
        } else {
            self.betweenplay.sim(away, home, &next_context, rng)
        };
        let new_context = between_res.next_context(&next_context);
        let between_summary = between_res.summary();
        let overall_summary = format!("{} {}", summary, between_summary);
        (Play::new(context, &overall_summary), new_context)
    }
}
