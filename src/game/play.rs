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
use crate::game::play::result::{PlayResultSimulator, PlayResult, PlayTypeResult, ScoreResult};
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

    /// Borrow the play's game context
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
    /// let my_borrowed_context = my_play.context();
    /// ```
    pub fn context(&self) -> &GameContext {
        &self.context
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
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum DriveResult {
    None,
    Punt,
    FieldGoal,
    Touchdown,
    Safety,
    Interception,
    Fumble,
    Downs,
    EndOfHalf
}

impl std::fmt::Display for DriveResult {
    /// Display a drive result as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::DriveResult;
    /// 
    /// println!("{}", DriveResult::None);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriveResult::None => f.write_str("In Progress"),
            DriveResult::Punt => f.write_str("Punt"),
            DriveResult::FieldGoal => f.write_str("Field Goal"),
            DriveResult::Touchdown => f.write_str("Touchdown"),
            DriveResult::Safety => f.write_str("Safety"),
            DriveResult::Interception => f.write_str("Interception"),
            DriveResult::Fumble => f.write_str("Fumble"),
            DriveResult::Downs => f.write_str("Turnover on Downs"),
            DriveResult::EndOfHalf => f.write_str("End of Half")
        }
    }
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

    /// Get the total yards on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// let total_yards = my_drive.total_yards();
    /// assert!(total_yards == 0);
    /// ```
    pub fn total_yards(&self) -> i32 {
        let mut yards: i32 = 0;
        for play in self.plays.iter() {
            match play.result() {
                PlayTypeResult::Run(res) => {
                    yards += res.net_yards();
                },
                PlayTypeResult::Pass(res) => {
                    yards += res.net_yards();
                },
                _ => continue
            }
        }
        yards
    }
}

impl std::fmt::Display for Drive {
    /// Display a drive as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::new();
    /// println!("{}", my_drive);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut drive_str = format!(
            "{} plays, {} yards | Result: {} | Passing: {}/{}, {} yards | Rushing: {} rush, {} yards",
            self.plays().len(),
            self.total_yards(),
            self.result(),
            self.completed_passes(),
            self.pass_plays(),
            self.passing_yards(),
            self.run_plays(),
            self.rushing_yards()
        );
        for play in self.plays() {
            drive_str = format!("{}\n{}", drive_str, play);
        }
        f.write_str(&drive_str)
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

    /// Simulate a drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::DriveSimulator;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    /// 
    /// // Initialize a drive simulator & simulate a drive
    /// let my_sim = DriveSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let (drive, next_context) = my_sim.sim(&my_home, &my_away, my_context, &mut rng);
    /// ```
    pub fn sim(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, rng: &mut impl Rng) -> (Drive, GameContext) {
        let mut extra_point_complete: bool = false;
        let mut drive: Drive = Drive::new();
        let mut result: DriveResult = DriveResult::None;
        let plays = drive.plays_mut();
        let mut prev_context = context.clone();
        let mut new_context: GameContext;
        loop {
            // Simulate a play
            let (play, next_context) = self.play.sim(home, away, prev_context, rng);
            let play_result = play.result();
            new_context = next_context;

            // Determine if a drive result occurred
            let mut touchdown: bool = false;
            let result_was_none = result == DriveResult::None;
            if result_was_none {
                let field_goal: bool = match play_result {
                    PlayTypeResult::FieldGoal(res) => res.made(),
                    _ => false
                };
                if field_goal {
                    result = DriveResult::FieldGoal;
                }

                // Punt
                let punt: bool = match play_result {
                    PlayTypeResult::Punt(_) => true,
                    _ => false
                };
                if punt {
                    result = DriveResult::Punt;
                }

                // Touchdown
                touchdown = play_result.offense_score() == ScoreResult::Touchdown ||
                    play_result.defense_score() == ScoreResult::Touchdown;
                if touchdown {
                    result = DriveResult::Touchdown;
                }

                // Safety
                let safety: bool = play_result.defense_score() == ScoreResult::Safety;
                if safety {
                    result = DriveResult::Safety;
                }

                // Interception
                let turnover = play_result.turnover();
                let interception = if turnover {
                    match play_result {
                        PlayTypeResult::Pass(res) => res.interception(),
                        _ => false
                    }
                } else {
                    false
                };
                if interception {
                    result = DriveResult::Interception;
                }

                // Fumble
                let fumble = if turnover {
                    match play_result {
                        PlayTypeResult::Run(res) => res.fumble(),
                        PlayTypeResult::Pass(res) => res.fumble(),
                        PlayTypeResult::FieldGoal(res) => res.blocked(),
                        PlayTypeResult::Punt(res) => res.fumble(),
                        PlayTypeResult::Kickoff(res) => res.fumble(),
                        PlayTypeResult::QbKneel(res) => res.fumble(),
                        PlayTypeResult::QbSpike(res) => res.fumble(),
                        _ => false
                    }
                } else {
                    false
                };
                if fumble {
                    result = DriveResult::Fumble;
                }

                // Downs
                let prev_context = play.context();
                let downs = *prev_context.down() == 4 && *new_context.down() == 1 && !turnover &&
                    *prev_context.home_possession() != *new_context.home_possession() &&
                    play_result.net_yards() < *prev_context.distance() as i32;
                if downs {
                    result = DriveResult::Downs;
                }

                // End of half
                let end_of_half = ((*prev_context.quarter() == 2 || *prev_context.quarter() >= 4) &&
                    (*prev_context.quarter() != *new_context.quarter())) || *new_context.game_over();
                if end_of_half {
                    result = DriveResult::EndOfHalf;
                }
            } else if result == DriveResult::Touchdown {
                extra_point_complete = true;
            }

            // Check if the result changed to something other than a touchdown
            if result_was_none && result != DriveResult::None && result != DriveResult::Touchdown && !touchdown {
                extra_point_complete = true;
            }

            // Add the play to the drive and update the previous context
            plays.push(play);
            
            // Break the loop if necessary
            if result == DriveResult::None || !extra_point_complete {
                prev_context = new_context
            } else {
                let drive_res = drive.result_mut();
                *drive_res = result;
                return (drive, new_context)
            }
        }
    }
}
