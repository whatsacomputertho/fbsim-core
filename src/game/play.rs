pub mod call;
pub mod context;
pub mod result;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use rand::Rng;

use crate::game::context::GameContext;
use crate::game::play::call::{PlayCallSimulator, PlayCall};
use crate::game::play::result::{PlayResultSimulator, PlayResult, PlayTypeResult};
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
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Play {
    context: GameContext,
    result: PlayTypeResult,
    post_play: PlayTypeResult
}

impl Play {
    /// Initialize a new play
    ///
    /// ```
    /// use fbsim_core::game::play::Play;
    /// use fbsim_core::game::play::result::PlayTypeResult;
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// use fbsim_core::game::context::GameContext;
    /// 
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a play type result
    /// let my_res = PlayTypeResult::Pass(PassResult::new());
    /// let my_between = PlayTypeResult::BetweenPlay(BetweenPlayResult::new());
    ///
    /// // Initialize a play
    /// let my_play = Play::new(my_context, my_res, my_between);
    /// ```
    pub fn new(context: GameContext, result: PlayTypeResult, post_play: PlayTypeResult) -> Play {
        Play{
            context: context,
            result: result,
            post_play: post_play
        }
    }

    /// Borrow the play result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Play;
    /// use fbsim_core::game::play::result::PlayTypeResult;
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// use fbsim_core::game::context::GameContext;
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a play type result
    /// let my_res = PlayTypeResult::Pass(PassResult::new());
    /// let my_between = PlayTypeResult::BetweenPlay(BetweenPlayResult::new());
    ///
    /// // Initialize a play and borrow its result
    /// let my_play = Play::new(my_context, my_res, my_between);
    /// let my_borrowed_res = my_play.result();
    /// ```
    pub fn result(&self) -> &PlayTypeResult {
        &self.result
    }
}

impl std::fmt::Display for Play {
    /// Format a `Play` as a string.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Play;
    /// use fbsim_core::game::play::result::PlayTypeResult;
    /// use fbsim_core::game::play::result::betweenplay::BetweenPlayResult;
    /// use fbsim_core::game::play::result::pass::PassResult;
    /// use fbsim_core::game::context::GameContext;
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    ///
    /// // Initialize a play type result
    /// let my_res = PlayTypeResult::Pass(PassResult::new());
    /// let my_between = PlayTypeResult::BetweenPlay(BetweenPlayResult::new());
    ///
    /// // Initialize a play and display it
    /// let my_play = Play::new(my_context, my_res, my_between);
    /// println!("{}", my_play);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let score_str = format!(
            "{} {} {}",
            &self.context,
            self.result,
            self.post_play
        );
        f.write_str(&score_str.trim())
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
        let result = if *context.home_possession() {
            match play_call {
                PlayCall::Run => self.run.sim(home, away, &context, rng),
                PlayCall::Pass => self.pass.sim(home, away, &context, rng),
                PlayCall::FieldGoal => self.fieldgoal.sim(home, away, &context, rng),
                PlayCall::Punt => self.punt.sim(home, away, &context, rng),
                PlayCall::Kickoff => self.kickoff.sim(home, away, &context, rng),
                PlayCall::ExtraPoint => self.fieldgoal.sim(home, away, &context, rng),
                PlayCall::QbKneel => self.run.sim(home, away, &context, rng),
                PlayCall::QbSpike => self.pass.sim(home, away, &context, rng)
            }
        } else {
            match play_call {
                PlayCall::Run => self.run.sim(away, home, &context, rng),
                PlayCall::Pass => self.pass.sim(away, home, &context, rng),
                PlayCall::FieldGoal => self.fieldgoal.sim(away, home, &context, rng),
                PlayCall::Punt => self.punt.sim(away, home, &context, rng),
                PlayCall::Kickoff => self.kickoff.sim(away, home, &context, rng),
                PlayCall::ExtraPoint => self.fieldgoal.sim(away, home, &context, rng),
                PlayCall::QbKneel => self.run.sim(away, home, &context, rng),
                PlayCall::QbSpike => self.pass.sim(away, home, &context, rng)
            }
        };
        let next_context = result.next_context(&context);

        // Simulate between plays
        let between_res = if *context.home_possession() {
            self.betweenplay.sim(home, away, &next_context, rng)
        } else {
            self.betweenplay.sim(away, home, &next_context, rng)
        };
        let new_context = between_res.next_context(&next_context);
        (Play::new(context, result, between_res), new_context)
    }
}

/// # `DriveResult` enum
///
/// Enumerates the possible outcomes of a drive
pub enum DriveResult {
    None,
    FieldGoal,
    Touchdown,
    Safety,
    Interception,
    Fumble,
    Downs,
    EndOfHalf
}

/// # `Drive` struct
///
/// A `Drive` represents the outcome of a drive
pub struct Drive {
    plays: Vec<Play>,
    result: DriveResult
}

impl Drive {
    /// Initialize a new drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// ```
    pub fn new() -> Drive {
        Drive {
            plays: Vec::new(),
            result: DriveResult::None
        }
    }

    /// Borrow the plays in the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let plays = my_drive.plays();
    /// ```
    pub fn plays(&self) -> &Vec<Play> {
        &self.plays
    }

    /// Mutably borrow the plays in the drive
    fn plays_mut(&mut self) -> &mut Vec<Play> {
        &mut self.plays
    }

    /// Get the result of the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::{Drive, DriveResult};
    /// 
    /// let my_drive = Drive::new();
    /// let result = my_drive.result();
    /// ```
    pub fn result(&self) -> &DriveResult {
        &self.result
    }

    /// Mutably borrow the result of the drive
    fn result_mut(&mut self) -> &mut DriveResult {
        &mut self.result
    }

    /// Get the number of pass plays on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let pass_plays = my_drive.pass_plays();
    /// assert!(pass_plays == 0);
    /// ```
    pub fn pass_plays(&self) -> u32 {
        let mut count: u32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Pass(_) => count += 1,
                _ => continue
            }
        }
        count
    }

    /// Get the number of completed passes on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let completed_passes = my_drive.completed_passes();
    /// assert!(completed_passes == 0);
    /// ```
    pub fn completed_passes(&self) -> u32 {
        let mut count: u32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Pass(res) => {
                    if res.complete() {
                        count += 1;
                    }
                },
                _ => continue
            }
        }
        count
    }

    /// Get the passing yards on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let passing_yards = my_drive.passing_yards();
    /// assert!(passing_yards == 0);
    /// ```
    pub fn passing_yards(&self) -> i32 {
        let mut yards: i32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Pass(res) => {
                    if res.complete() {
                        yards += res.net_yards();
                    }
                },
                _ => continue
            }
        }
        yards
    }

    /// Get the number of run plays on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let run_plays = my_drive.run_plays();
    /// assert!(run_plays == 0);
    /// ```
    pub fn run_plays(&self) -> u32 {
        let mut count: u32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Run(_) => count += 1,
                _ => continue
            }
        }
        count
    }

    /// Get the rushing yards on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let rushing_yards = my_drive.rushing_yards();
    /// assert!(rushing_yards == 0);
    /// ```
    pub fn rushing_yards(&self) -> i32 {
        let mut yards: i32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Run(res) => {
                    yards += res.net_yards();
                },
                _ => continue
            }
        }
        yards
    }
}

/// # `DriveSimulator` struct
///
/// A `DriveSimulator` can simulate a drive given a context, returning an
/// updated context and a drive
pub struct DriveSimulator {
    play: PlaySimulator
}

impl DriveSimulator {
    /// Initialize a new drive simulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::DriveSimulator;
    /// 
    /// // Initialize a drive simulator
    /// let my_sim = DriveSimulator::new();
    /// ```
    pub fn new() -> DriveSimulator {
        DriveSimulator{
            play: PlaySimulator::new()
        }
    }

    // TODO: Simulate a drive
}
