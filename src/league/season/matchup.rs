#![doc = include_str!("../../../docs/league/season/matchup.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::game::context::{GameContext, GameContextBuilder};
use crate::game::play::Game;
use crate::game::stat::OffensiveStats;
use crate::game::matchup::FootballMatchupResult;
use crate::league::matchup::LeagueTeamRecord;

/// # `LeagueSeasonMatchup` struct
///
/// A `LeagueSeasonMatchup` represents a matchup during a week of a football season
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonMatchup {
    home_team: usize,
    away_team: usize,
    context: GameContext,
    game: Option<Game>,
    home_stats: Option<OffensiveStats>,
    away_stats: Option<OffensiveStats>
}

impl LeagueSeasonMatchup {
    /// Constructor for the LeagueSeasonMatchup struct in which the home and
    /// away team IDs are given, and the score & completion status is zeroed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// ```
    pub fn new(home_team: usize, away_team: usize, home_short_name: &str, away_short_name: &str, rng: &mut impl Rng) -> LeagueSeasonMatchup {
        // Generate a GameContext
        let home_opening_kickoff: bool = rng.gen::<bool>();
        let context: GameContext = GameContextBuilder::new()
            .home_team_short(home_short_name)
            .away_team_short(away_short_name)
            .home_possession(!home_opening_kickoff)
            .home_positive_direction(!home_opening_kickoff)
            .home_opening_kickoff(home_opening_kickoff)
            .build()
            .unwrap();
        
        // Instantiate and return a LeagueSeasonMatchup
        LeagueSeasonMatchup {
            home_team,
            away_team,
            context,
            game: None,
            home_stats: None,
            away_stats: None
        }
    }

    /// Borrow the home team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let home_id = my_matchup.home_team();
    /// ```
    pub fn home_team(&self) -> &usize {
        &self.home_team
    }

    /// Borrow the away team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let away_id = my_matchup.away_team();
    /// ```
    pub fn away_team(&self) -> &usize {
        &self.away_team
    }

    /// Borrow the matchup's GameContext
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let context = my_matchup.context();
    /// ```
    pub fn context(&self) -> &GameContext {
        &self.context
    }

    /// Mutably borrow the matchup's GameContext
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let context = my_matchup.context_mut();
    /// ```
    pub fn context_mut(&mut self) -> &mut GameContext {
        &mut self.context
    }

    /// Borrow the matchup's Game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let game = my_matchup.game();
    /// ```
    pub fn game(&self) -> &Option<Game> {
        &self.game
    }

    /// Mutably borrow the matchup's Game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let game = my_matchup.game_mut();
    /// ```
    pub fn game_mut(&mut self) -> &mut Option<Game> {
        &mut self.game
    }

    /// Take ownership of the matchup's Game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let game = my_matchup.take_game();
    /// ```
    pub fn take_game(&mut self) -> Option<Game> {
        self.game.take()
    }

    /// Borrow the matchup's home stats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let home_stats = my_matchup.home_stats();
    /// ```
    pub fn home_stats(&self) -> &Option<OffensiveStats> {
        &self.home_stats
    }

    /// Mutably borrow the matchup's home stats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let home_stats = my_matchup.home_stats_mut();
    /// ```
    pub fn home_stats_mut(&mut self) -> &mut Option<OffensiveStats> {
        &mut self.home_stats
    }

    /// Borrow the matchup's away stats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let away_stats = my_matchup.away_stats();
    /// ```
    pub fn away_stats(&self) -> &Option<OffensiveStats> {
        &self.away_stats
    }

    /// Mutably borrow the matchup's away stats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let away_stats = my_matchup.away_stats_mut();
    /// ```
    pub fn away_stats_mut(&mut self) -> &mut Option<OffensiveStats> {
        &mut self.away_stats
    }

    /// Determine whether the given team participated in the matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// assert!(my_matchup.participated(0));
    /// assert!(!my_matchup.participated(2));
    /// ```
    pub fn participated(&self, id: usize) -> bool {
        if id == self.home_team || id == self.away_team {
            return true;
        }
        false
    }

    /// Determine whether the given team was the home team in the matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// assert!(my_matchup.is_home_team(0));
    /// assert!(!my_matchup.is_home_team(1));
    /// assert!(!my_matchup.is_home_team(2));
    /// ```
    pub fn is_home_team(&self, id: usize) -> bool {
        if id == self.home_team {
            return true;
        }
        false
    }

    /// Determine whether the given team won, lost, or tied
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let res = my_matchup.result(0);
    /// assert!(res.is_none());
    /// ```
    pub fn result(&self, id: usize) -> Option<FootballMatchupResult> {
        // If the team did not participate or the game is not complete
        // Then it has no result
        if !(self.context.game_over() && self.participated(id)) {
            return None;
        }

        // Calculate and return the result
        if self.context.home_score() == self.context.away_score() {
            return Some(FootballMatchupResult::Tie);
        }
        if self.is_home_team(id) {
            if self.context.home_score() > self.context.away_score() {
                Some(FootballMatchupResult::Win)
            } else {
                Some(FootballMatchupResult::Loss)
            }
        } else if self.context.home_score() > self.context.away_score() {
            Some(FootballMatchupResult::Loss)
        } else {
            Some(FootballMatchupResult::Win)
        }
    }

    /// Returns the winner of the matchup if the matchup is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut rng = rand::thread_rng();
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1, "HOME", "AWAY", &mut rng);
    /// let winner = my_matchup.winner();
    /// assert!(winner.is_none());
    /// ```
    pub fn winner(&self) -> Option<usize> {
        // There is no winner if the game is not complete
        if !self.context.game_over() {
            return None;
        }

        // If the game is complete, determine the winner based on the result
        let result = self.result(self.home_team);
        if let Some(r) = result {
            match r {
                FootballMatchupResult::Win => Some(self.home_team),
                FootballMatchupResult::Loss => Some(self.away_team),
                FootballMatchupResult::Tie => None
            }
        } else {
            None
        }
    }
}

/// # `LeagueSeasonMatchups` struct
///
/// Represents a list of matchups for a given team during a given season
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LeagueSeasonMatchups {
    team_id: usize,
    matchups: Vec<Option<LeagueSeasonMatchup>>
}

impl LeagueSeasonMatchups {
    /// Instantiate a new LeagueSeasonMatchups struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
    ///
    /// let my_matchups = LeagueSeasonMatchups::new(0, Vec::new());
    /// ```
    pub fn new(team_id: usize, matchups: Vec<Option<LeagueSeasonMatchup>>) -> LeagueSeasonMatchups {
        LeagueSeasonMatchups{
            team_id,
            matchups
        }
    }

    /// Borrow the season matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
    /// 
    /// let my_matchups = LeagueSeasonMatchups::new(0, Vec::new());
    /// let matchups = my_matchups.matchups();
    /// ```
    pub fn matchups(&self) -> &Vec<Option<LeagueSeasonMatchup>> {
        &self.matchups
    }

    /// Compute the team record
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
    ///
    /// let my_matchups = LeagueSeasonMatchups::new(0, Vec::new());
    /// let record = my_matchups.record();
    /// assert!(record == LeagueTeamRecord::new());
    /// ```
    pub fn record(&self) -> LeagueTeamRecord {
        // Initialize a new LeagueTeamRecord
        let mut record = LeagueTeamRecord::new();

        // Loop through the matchups and increment the team record
        for matchup in self.matchups.iter().flatten() {
            if let Some(r) = matchup.result(self.team_id) {
                match r {
                    FootballMatchupResult::Win => record.increment_wins(1),
                    FootballMatchupResult::Loss => record.increment_losses(1),
                    FootballMatchupResult::Tie => record.increment_ties(1)
                }
            }
        }
        record
    }

    /// Compute the team stats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::OffensiveStats;
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
    ///
    /// let my_matchups = LeagueSeasonMatchups::new(0, Vec::new());
    /// let stats = my_matchups.stats();
    /// assert!(stats == OffensiveStats::new());
    /// ```
    pub fn stats(&self) -> OffensiveStats {
        // Initialize a new OffensiveStats
        let mut stats = OffensiveStats::new();

        // Loop through the matchups and increment the team stats
        for matchup in self.matchups.iter().flatten() {
            // Get the game stats for the team
            let game_stat_opt = if self.team_id == *matchup.home_team() {
                matchup.home_stats()
            } else {
                matchup.away_stats()
            };

            // If no stats, then the game hasn't been simulated
            let game_stats = match game_stat_opt {
                Some(s) => s,
                None => continue
            };

            // Increment the season stats using the game stats
            stats.increment_passing(game_stats.passing());
            stats.increment_rushing(game_stats.rushing());
            stats.increment_receiving(game_stats.receiving());
        }
        stats
    }
}
