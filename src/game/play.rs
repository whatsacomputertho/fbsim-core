#![doc = include_str!("../../docs/game/play.md")]
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
use crate::game::stat::{PassingStats, RushingStats, ReceivingStats, OffensiveStats};
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
            context,
            result,
            post_play
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

    /// Borrow the post-play result
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
    /// let my_borrowed_post_play = my_play.post_play();
    /// ```
    pub fn post_play(&self) -> &PlayTypeResult {
        &self.post_play
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
        f.write_str(score_str.trim())
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

impl Default for PlaySimulator {
    /// Default constructor for the `PlaySimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::PlaySimulator;
    /// 
    /// let my_sim = PlaySimulator::default();
    /// ```
    fn default() -> Self {
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
        PlaySimulator::default()
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
        let play_call = if context.next_play_kickoff() {
            PlayCall::Kickoff
        } else if context.home_possession() {
            self.playcall.sim(home, &context, rng)
        } else {
            self.playcall.sim(away, &context, rng)
        };

        // Simulate the play
        let result = if context.home_possession() {
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
        let between_res = if context.home_possession() {
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
    FieldGoalMissed,
    Touchdown,
    Safety,
    Interception,
    PickSix,
    Fumble,
    ScoopAndScore,
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
            DriveResult::FieldGoalMissed => f.write_str("Missed Field Goal"),
            DriveResult::Touchdown => f.write_str("Touchdown"),
            DriveResult::Safety => f.write_str("Safety"),
            DriveResult::Interception => f.write_str("Interception"),
            DriveResult::PickSix => f.write_str("Pick Six"),
            DriveResult::Fumble => f.write_str("Fumble"),
            DriveResult::ScoopAndScore => f.write_str("Scoop and Score"),
            DriveResult::Downs => f.write_str("Turnover on Downs"),
            DriveResult::EndOfHalf => f.write_str("End of Half")
        }
    }
}

/// # `Drive` struct
///
/// A `Drive` represents the outcome of a drive
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Drive {
    plays: Vec<Play>,
    result: DriveResult,
    complete: bool
}

impl Default for Drive {
    /// Default constructor for the Drive struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let my_drive = Drive::default();
    /// ```
    fn default() -> Self {
        Drive {
            plays: Vec::new(),
            result: DriveResult::None,
            complete: false
        }
    }
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
        Drive::default()
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

    /// Get whether the drive is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    /// 
    /// let mut my_drive = Drive::new();
    /// let complete = my_drive.complete();
    /// assert!(!complete);
    /// ```
    pub fn complete(&self) -> bool {
        self.complete
    }

    /// Mutably borrow the drive's complete property
    fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }

    /// Get the rushing stats on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    ///
    /// let drive = Drive::new();
    /// let rushing_stats = drive.rushing_stats();
    /// assert!(rushing_stats.yards() == 0);
    /// assert!(rushing_stats.rushes() == 0);
    /// ```
    pub fn rushing_stats(&self) -> RushingStats {
        let mut stats = RushingStats::new();
        for play in self.plays().iter() {
            match play.result() {
                PlayTypeResult::Run(res) => {
                    // Skip tallying stats for two-point conversions
                    if res.two_point_conversion() {
                        continue
                    }

                    // Increment rushes & rushing yards
                    stats.increment_rushes();
                    stats.increment_yards(res.net_yards());

                    // Increment rushing TDs & fumbles if either occur
                    if res.touchdown() {
                        stats.increment_touchdowns();
                    }
                    if res.fumble() {
                        stats.increment_fumbles();
                    }
                },
                PlayTypeResult::Pass(res) => {
                    if res.scramble() && !res.two_point_conversion() {
                        // Increment rushes & rushing yards
                        stats.increment_rushes();
                        stats.increment_yards(res.net_yards());

                        // Increment rushing TDs & fumbles if either occur
                        if res.touchdown() {
                            stats.increment_touchdowns();
                        }
                        if res.fumble() {
                            stats.increment_fumbles();
                        }
                    }
                },
                _ => continue
            }
        }
        stats
    }

    /// Get the passing stats on the drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Drive;
    ///
    /// let drive = Drive::new();
    /// let passing_stats = drive.passing_stats();
    /// assert!(passing_stats.yards() == 0);
    /// assert!(passing_stats.completions() == 0);
    /// ```
    pub fn passing_stats(&self) -> PassingStats {
        let mut stats = PassingStats::new();
        for play in self.plays().iter() {
            match play.result() {
                PlayTypeResult::Pass(res) => {
                    // Skip tallying stats for two-point conversions
                    if res.two_point_conversion() {
                        continue
                    }

                    // Increment attempts
                    if !(res.scramble() || res.sack()) {
                        stats.increment_attempts();
                    }
                    
                    // Increment completions and yards if complete
                    if res.complete() {
                        stats.increment_completions();
                        stats.increment_yards(res.net_yards());
                        
                        // Increment TDs if completion and touchdown
                        if res.touchdown() {
                            stats.increment_touchdowns();
                        }
                    } else if res.sack() {
                        stats.increment_yards(res.net_yards());
                    }

                    // Increment interceptions if this was an INT
                    if res.interception() {
                        stats.increment_interceptions();
                    }
                },
                _ => continue
            }
        }
        stats
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
            "{} plays, {} yards | Result: {} | Passing: {} | Rushing: {}",
            self.plays().len(),
            self.total_yards(),
            self.result(),
            self.passing_stats(),
            self.rushing_stats()
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

impl Default for DriveSimulator {
    /// Default constructor for the DriveSimulator struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::DriveSimulator;
    /// 
    /// let my_sim = DriveSimulator::default();
    /// ```
    fn default() -> Self {
        DriveSimulator{
            play: PlaySimulator::new()
        }
    }
}

impl DriveSimulator {
    /// Initialize a new drive simulator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::DriveSimulator;
    /// 
    /// let my_sim = DriveSimulator::new();
    /// ```
    pub fn new() -> DriveSimulator {
        DriveSimulator::default()
    }

    /// Simulate the next play of a drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::{Drive, DriveSimulator};
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let mut my_context = GameContext::new();
    /// let mut drive = Drive::new();
    /// 
    /// // Initialize a drive simulator & simulate a drive
    /// let my_sim = DriveSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// my_context = my_sim.sim_play(&my_home, &my_away, my_context, &mut drive, &mut rng).unwrap();
    /// ```
    pub fn sim_play(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, drive: &mut Drive, rng: &mut impl Rng) -> Result<GameContext, String> {
        // Ensure the result is none, unless last play was a touchdown
        let drive_res = *drive.result();
        let result_was_none = drive_res == DriveResult::None;
        let result_was_touchdown = drive_res == DriveResult::Touchdown ||
            drive_res == DriveResult::ScoopAndScore ||
            drive_res == DriveResult::PickSix;
        if !(result_was_none || result_was_touchdown) {
            return Err(
                format!(
                    "Cannot simulate play, result was not None ({}) and last play was not TD",
                    drive.result()
                )
            )
        }

        // Simulate a play
        let mut complete = false;
        let mut result = DriveResult::None;
        let prev_context = context.clone();
        let (play, next_context) = self.play.sim(home, away, prev_context, rng);
        let play_result = play.result();
        let new_context = next_context;

        // Determine if a drive result occurred
        let result_was_none = *drive.result() == DriveResult::None;
        if result_was_none {
            let field_goal: bool = match play_result {
                PlayTypeResult::FieldGoal(res) => res.made(),
                _ => false
            };
            if field_goal {
                result = DriveResult::FieldGoal;
                complete = true;
            }
            let field_goal_missed: bool = match play_result {
                PlayTypeResult::FieldGoal(res) => res.missed(),
                _ => false
            };
            if field_goal_missed {
                result = DriveResult::FieldGoalMissed;
                complete = true;
            }

            // Punt
            if matches!(play_result, PlayTypeResult::Punt(_)) {
                result = DriveResult::Punt;
                complete = true;
            }

            // Touchdown
            let touchdown = play_result.offense_score() == ScoreResult::Touchdown ||
                play_result.defense_score() == ScoreResult::Touchdown;
            if touchdown {
                result = DriveResult::Touchdown;
                complete = false;
            }

            // Safety
            let safety: bool = play_result.defense_score() == ScoreResult::Safety;
            if safety {
                result = DriveResult::Safety;
                complete = true;
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
                if touchdown {
                    result = DriveResult::PickSix;
                    complete = false;
                } else {
                    result = DriveResult::Interception;
                    complete = true;
                }
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
                if touchdown {
                    result = DriveResult::ScoopAndScore;
                    complete = false;
                } else {
                    result = DriveResult::Fumble;
                    complete = true;
                }
            }

            // Downs
            let prev_context = play.context();
            let downs = prev_context.down() == 4 && new_context.down() == 1 && !turnover &&
                prev_context.home_possession() != new_context.home_possession() &&
                play_result.net_yards() < prev_context.distance() as i32;
            if downs {
                result = DriveResult::Downs;
                complete = true;
            }

            // End of half
            let end_of_half = ((prev_context.quarter() == 2 || prev_context.quarter() >= 4) &&
                (prev_context.quarter() != new_context.quarter())) || new_context.game_over();
            if end_of_half {
                result = DriveResult::EndOfHalf;
                complete = true;
            }
        } else if result_was_touchdown {
            result = drive_res;
            complete = true;
        }

        // Add the play to the drive, update result, return the new drive & context
        let plays = drive.plays_mut();
        plays.push(play);
        let drive_res = drive.result_mut();
        *drive_res = result;
        let drive_complete = drive.complete_mut();
        *drive_complete = complete;
        Ok(new_context)
    }

    /// Simulate the remaining plays of a drive
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::{Drive, DriveSimulator};
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
    /// let mut my_drive = Drive::new();
    /// let my_sim = DriveSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let next_context = my_sim.sim_drive(&my_home, &my_away, my_context, &mut my_drive, &mut rng).unwrap();
    /// ```
    pub fn sim_drive(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, drive: &mut Drive, rng: &mut impl Rng) -> Result<GameContext, String> {
        let mut extra_point_complete: bool = false;
        let mut prev_context = context.clone();
        while !drive.complete() {
            // Simulate a play
            let prev_result = *drive.result();
            let next_context = match self.sim_play(home, away, prev_context, drive, rng) {
                Ok(c) => c,
                Err(e) => return Err(format!("Error simulating the next play of drive: {}", e))
            };
            let result = *drive.result();

            // Check if the result was something other than a touchdown
            // Or whether the previous result was a touchdown and this was the extra point
            if (prev_result == DriveResult::Touchdown || prev_result == DriveResult::PickSix || prev_result == DriveResult::ScoopAndScore) ||
                (prev_result == DriveResult::None && result != DriveResult::None && result != DriveResult::Touchdown) {
                extra_point_complete = true;
            }
            
            // Break the loop if necessary
            if (result == DriveResult::None && prev_result == DriveResult::None) || !extra_point_complete {
                prev_context = next_context
            } else {
                return Ok(next_context)
            }
        }
        Err(String::from("Drive was already complete"))
    }

    /// Simulate a new drive
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
        let mut prev_context = context.clone();
        loop {
            // Simulate a play
            let prev_result = *drive.result();
            let next_context = self.sim_play(home, away, prev_context, &mut drive, rng).unwrap();
            let result = *drive.result();

            // Check if the result was something other than a touchdown
            // Or whether the previous result was a touchdown and this was the extra point
            if (prev_result == DriveResult::Touchdown || prev_result == DriveResult::PickSix || prev_result == DriveResult::ScoopAndScore) ||
                (prev_result == DriveResult::None && result != DriveResult::None && result != DriveResult::Touchdown) {
                extra_point_complete = true;
            }
            
            // Break the loop if necessary
            if (result == DriveResult::None && prev_result == DriveResult::None) || !extra_point_complete {
                prev_context = next_context
            } else {
                return (drive, next_context)
            }
        }
    }
}

/// # `Game` struct
///
/// A `Game` represents the outcome of a game
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Game {
    drives: Vec<Drive>,
    complete: bool
}

impl Default for Game {
    /// Default constructor for the Game struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::default();
    /// ```
    fn default() -> Self {
        Game {
            drives: Vec::new(),
            complete: false
        }
    }
}

impl Game {
    /// Initialize a new game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// ```
    pub fn new() -> Game {
        Game::default()
    }

    /// Get whether the game is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let complete = game.complete();
    /// assert!(!complete);
    /// ```
    pub fn complete(&self) -> bool {
        self.complete
    }

    /// Borrow the game's drives
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let drives = game.drives();
    /// ```
    pub fn drives(&self) -> &Vec<Drive> {
        &self.drives
    }

    /// Borrow the game's drives mutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let mut game = Game::new();
    /// let drives = game.drives_mut();
    /// ```
    pub fn drives_mut(&mut self) -> &mut Vec<Drive> {
        &mut self.drives
    }

    /// Get the rushing stats for either team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let rushing_stats = game.rushing_stats(true);
    /// assert!(rushing_stats.yards() == 0);
    /// assert!(rushing_stats.rushes() == 0);
    /// ```
    pub fn rushing_stats(&self, home: bool) -> RushingStats {
        let mut stats = RushingStats::new();
        for drive in self.drives.iter() {
            for play in drive.plays().iter() {
                if play.context().home_possession() == home {
                    match play.result() {
                        PlayTypeResult::Run(res) => {
                            // Skip tallying stats for two-point conversions
                            if res.two_point_conversion() {
                                continue
                            }

                            // Increment rushes & rushing yards
                            stats.increment_rushes();
                            stats.increment_yards(res.net_yards());

                            // Increment rushing TDs & fumbles if either occur
                            if res.touchdown() {
                                stats.increment_touchdowns();
                            }
                            if res.fumble() {
                                stats.increment_fumbles();
                            }
                        },
                        PlayTypeResult::Pass(res) => {
                            if res.scramble() && !res.two_point_conversion() {
                                // Increment rushes & rushing yards
                                stats.increment_rushes();
                                stats.increment_yards(res.net_yards());

                                // Increment rushing TDs & fumbles if either occur
                                if res.touchdown() {
                                    stats.increment_touchdowns();
                                }
                                if res.fumble() {
                                    stats.increment_fumbles();
                                }
                            }
                        },
                        _ => continue
                    }
                }
            }
        }
        stats
    }

    /// Get the passing stats for either team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let passing_stats = game.passing_stats(true);
    /// assert!(passing_stats.yards() == 0);
    /// assert!(passing_stats.completions() == 0);
    /// ```
    pub fn passing_stats(&self, home: bool) -> PassingStats {
        let mut stats = PassingStats::new();
        for drive in self.drives.iter() {
            for play in drive.plays().iter() {
                if play.context().home_possession() == home {
                    match play.result() {
                        PlayTypeResult::Pass(res) => {
                            // Skip tallying stats for two-point conversions
                            if res.two_point_conversion() {
                                continue
                            }
                            
                            // Increment attempts
                            if !(res.scramble() || res.sack()) {
                                stats.increment_attempts();
                            }
                            
                            // Increment completions and yards if complete
                            if res.complete() {
                                stats.increment_completions();
                                stats.increment_yards(res.net_yards());
                                
                                // Increment TDs if completion and touchdown
                                if res.touchdown() {
                                    stats.increment_touchdowns();
                                }
                            } else if res.sack() {
                                stats.increment_yards(res.net_yards());
                            }

                            // Increment interceptions if this was an INT
                            if res.interception() {
                                stats.increment_interceptions();
                            }
                        },
                        _ => continue
                    }
                }
            }
        }
        stats
    }

    /// Get the receiving stats for either team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let receiving_stats = game.receiving_stats(true);
    /// assert!(receiving_stats.yards() == 0);
    /// assert!(receiving_stats.receptions() == 0);
    /// ```
    pub fn receiving_stats(&self, home: bool) -> ReceivingStats {
        let mut stats = ReceivingStats::new();
        for drive in self.drives.iter() {
            for play in drive.plays().iter() {
                if play.context().home_possession() == home {
                    match play.result() {
                        PlayTypeResult::Pass(res) => {
                            // Skip tallying stats for two-point conversions
                            if res.two_point_conversion() {
                                continue
                            }
                            
                            // Increment targets
                            if !(res.scramble() || res.sack()) {
                                stats.increment_targets(1);
                            }
                            
                            // Increment receptions and yards if complete
                            if res.complete() {
                                stats.increment_receptions(1);
                                stats.increment_yards(res.net_yards());
                                
                                // Increment TDs or fumbles if either occur
                                if res.touchdown() {
                                    stats.increment_touchdowns(1);
                                }
                                if res.fumble() {
                                    stats.increment_fumbles(1);
                                }
                            }
                        },
                        _ => continue
                    }
                }
            }
        }
        stats
    }

    /// Get the offensive stats for the home team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let home_stats = game.home_stats();
    /// ```
    pub fn home_stats(&self) -> OffensiveStats {
        OffensiveStats::from_properties(
            self.passing_stats(true),
            self.rushing_stats(true),
            self.receiving_stats(true)
        )
    }

    /// Get the offensive stats for the away team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    ///
    /// let game = Game::new();
    /// let away_stats = game.away_stats();
    /// ```
    pub fn away_stats(&self) -> OffensiveStats {
        OffensiveStats::from_properties(
            self.passing_stats(false),
            self.rushing_stats(false),
            self.receiving_stats(false)
        )
    }
}

impl std::fmt::Display for Game {
    /// Display a game as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::Game;
    /// 
    /// let my_game = Game::new();
    /// println!("{}", my_game);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the game log
        let mut game_log = String::from("");
        for drive in self.drives() {
            game_log = format!("{}\n\n{}", game_log, drive);
        }
        f.write_str(game_log.trim())
    }
}

/// # `GameSimulator` struct
///
/// A `GameSimulator` can simulate a game given a context, returning an
/// updated context and a drive
pub struct GameSimulator {
    drive: DriveSimulator
}

impl Default for GameSimulator {
    /// Default constructor for the `GameSimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::GameSimulator;
    ///
    /// let my_sim = GameSimulator::default();
    /// ```
    fn default() -> Self {
        GameSimulator {
            drive: DriveSimulator::new()
        }
    }
}

impl GameSimulator {
    /// Constructor for the `GameSimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::GameSimulator;
    ///
    /// let my_sim = GameSimulator::new();
    /// ```
    pub fn new() -> GameSimulator {
        GameSimulator::default()
    }

    /// Simulate the next play of a game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::{GameSimulator, Game};
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    /// 
    /// // Initialize a game simulator & simulate a drive
    /// let mut my_game = Game::new();
    /// let my_sim = GameSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let next_context = my_sim.sim_play(&my_home, &my_away, my_context, &mut my_game, &mut rng).unwrap();
    /// ```
    pub fn sim_play(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, game: &mut Game, rng: &mut impl Rng) -> Result<GameContext, String> {
        // Error if the game is over
        if context.game_over() {
            return Err(String::from("Game is already over, cannot simulate next play"))
        }

        // Get the latest drive to sim or create new one if latest is complete
        let drives = game.drives_mut();
        let new_context = match drives.last_mut() {
            Some(d) => {
                if !d.complete() {
                    match self.drive.sim_play(home, away, context, d, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating next play of game: {}", e))
                    }
                } else {
                    let mut new_drive = Drive::new();
                    let new_context = match self.drive.sim_play(home, away, context, &mut new_drive, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating the next play of game: {}", e))
                    };
                    drives.push(new_drive);
                    new_context
                }
            },
            None => {
                let mut new_drive = Drive::new();
                let new_context = match self.drive.sim_play(home, away, context, &mut new_drive, rng) {
                    Ok(c) => c,
                    Err(e) => return Err(format!("Error simulating the next play of game: {}", e))
                };
                drives.push(new_drive);
                new_context
            }
        };

        // Return the new context
        Ok(new_context)
    }

    /// Simulate the next drive of a game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::{GameSimulator, Game};
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    /// 
    /// // Initialize a game simulator & simulate a drive
    /// let mut my_game = Game::new();
    /// let my_sim = GameSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let next_context = my_sim.sim_drive(&my_home, &my_away, my_context, &mut my_game, &mut rng).unwrap();
    /// ```
    pub fn sim_drive(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, game: &mut Game, rng: &mut impl Rng) -> Result<GameContext, String> {
        // Error if the game is over
        if context.game_over() {
            return Err(String::from("Game is already over, cannot simulate next drive"))
        }

        // Get the latest drive to sim or create new one if latest is complete
        let drives = game.drives_mut();
        let new_context = match drives.last_mut() {
            Some(d) => {
                if !d.complete() {
                    match self.drive.sim_drive(home, away, context, d, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                    }
                } else {
                    let mut new_drive = Drive::new();
                    let new_context = match self.drive.sim_drive(home, away, context, &mut new_drive, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                    };
                    drives.push(new_drive);
                    new_context
                }
            },
            None => {
                let mut new_drive = Drive::new();
                let new_context = match self.drive.sim_drive(home, away, context, &mut new_drive, rng) {
                    Ok(c) => c,
                    Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                };
                drives.push(new_drive);
                new_context
            }
        };

        // Return the new context
        Ok(new_context)
    }

    /// Simulate the remainder of a game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::{GameSimulator, Game};
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    /// 
    /// // Initialize a game simulator and game, simulate the game
    /// let mut my_game = Game::new();
    /// let my_sim = GameSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let next_context = my_sim.sim_game(&my_home, &my_away, my_context, &mut my_game, &mut rng).unwrap();
    /// ```
    pub fn sim_game(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, game: &mut Game, rng: &mut impl Rng) -> Result<GameContext, String> {
        // Error if the game is over
        if context.game_over() {
            return Err(String::from("Game is already over, cannot simulate remainder of game"))
        }

        // Get the latest drive to sim or create new one if latest is complete
        let drives = game.drives_mut();
        let mut next_context = context.clone();
        let mut game_over = next_context.game_over();
        while !game_over {
            let new_context = match drives.last_mut() {
                Some(d) => {
                    if !d.complete() {
                        match self.drive.sim_drive(home, away, next_context, d, rng) {
                            Ok(c) => c,
                            Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                        }
                    } else {
                        let mut new_drive = Drive::new();
                        let new_context = match self.drive.sim_drive(home, away, next_context, &mut new_drive, rng) {
                            Ok(c) => c,
                            Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                        };
                        drives.push(new_drive);
                        new_context
                    }
                },
                None => {
                    let mut new_drive = Drive::new();
                    let new_context = match self.drive.sim_drive(home, away, next_context, &mut new_drive, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                    };
                    drives.push(new_drive);
                    new_context
                }
            };
            game_over = new_context.game_over();
            next_context = new_context;
        }

        // Return the final context
        Ok(next_context)
    }

    /// Simulate a new game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::GameSimulator;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// // Initialize home & away teams
    /// let my_home = FootballTeam::new();
    /// let my_away = FootballTeam::new();
    ///
    /// // Initialize a game context
    /// let my_context = GameContext::new();
    /// 
    /// // Initialize a game simulator & simulate a game
    /// let my_sim = GameSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let (game, final_context) = my_sim.sim(&my_home, &my_away, my_context, &mut rng).unwrap();
    /// ```
    pub fn sim(&self, home: &FootballTeam, away: &FootballTeam, context: GameContext, rng: &mut impl Rng) -> Result<(Game, GameContext), String> {
        // Get the latest drive to sim or create new one if latest is complete
        let mut game = Game::new();
        let drives = game.drives_mut();
        let mut next_context = context.clone();
        let mut game_over = next_context.game_over();
        while !game_over {
            let new_context = match drives.last_mut() {
                Some(d) => {
                    if !d.complete() {
                        match self.drive.sim_drive(home, away, next_context, d, rng) {
                            Ok(c) => c,
                            Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                        }
                    } else {
                        let mut new_drive = Drive::new();
                        let new_context = match self.drive.sim_drive(home, away, next_context, &mut new_drive, rng) {
                            Ok(c) => c,
                            Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                        };
                        drives.push(new_drive);
                        new_context
                    }
                },
                None => {
                    let mut new_drive = Drive::new();
                    let new_context = match self.drive.sim_drive(home, away, next_context, &mut new_drive, rng) {
                        Ok(c) => c,
                        Err(e) => return Err(format!("Error simulating the next drive of game: {}", e))
                    };
                    drives.push(new_drive);
                    new_context
                }
            };
            game_over = new_context.game_over();
            next_context = new_context;
        }

        // Return the final context
        Ok((game, next_context))
    }
}
