#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::matchup::FootballMatchupResult;
use crate::league::matchup::LeagueTeamRecord;

/// # `LeagueSeasonMatchup` struct
///
/// A `LeagueSeasonMatchup` represents a matchup during a week of a football season
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonMatchup {
    home_team: usize,
    away_team: usize,
    home_score: usize,
    away_score: usize,
    complete: bool
}

impl LeagueSeasonMatchup {
    /// Constructor for the LeagueSeasonMatchup struct in which the home and
    /// away team IDs are given, and the score & completion status is zeroed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// ```
    pub fn new(home_team: usize, away_team: usize) -> LeagueSeasonMatchup {
        LeagueSeasonMatchup {
            home_team: home_team,
            away_team: away_team,
            home_score: 0,
            away_score: 0,
            complete: false
        }
    }

    /// Borrow the home team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
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
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let away_id = my_matchup.away_team();
    /// ```
    pub fn away_team(&self) -> &usize {
        &self.away_team
    }

    /// Borrow the home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let home_score = my_matchup.home_score();
    /// ```
    pub fn home_score(&self) -> &usize {
        &self.home_score
    }

    /// Mutably borrow the home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut home_score = my_matchup.home_score_mut();
    /// ```
    pub fn home_score_mut(&mut self) -> &mut usize {
        &mut self.home_score
    }

    /// Borrow the away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let away_score = my_matchup.away_score();
    /// ```
    pub fn away_score(&self) -> &usize {
        &self.away_score
    }

    /// Mutably borrow the away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut away_score = my_matchup.away_score_mut();
    /// ```
    pub fn away_score_mut(&mut self) -> &mut usize {
        &mut self.away_score
    }

    /// Borrow the complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let complete = my_matchup.complete();
    /// ```
    pub fn complete(&self) -> &bool {
        &self.complete
    }

    /// Mutably borrow the complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut complete = my_matchup.complete_mut();
    /// ```
    pub fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }

    /// Determine whether the given team participated in the matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
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
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
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
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let res = my_matchup.result(0);
    /// assert!(res.is_none());
    /// ```
    pub fn result(&self, id: usize) -> Option<FootballMatchupResult> {
        // If the team did not participate or the game is not complete
        // Then it has no result
        if !(self.complete && self.participated(id)) {
            return None;
        }

        // Calculate and return the result
        if self.home_score == self.away_score {
            return Some(FootballMatchupResult::Tie);
        }
        if self.is_home_team(id) {
            if self.home_score > self.away_score {
                return Some(FootballMatchupResult::Win);
            } else {
                return Some(FootballMatchupResult::Loss);
            }
        } else {
            if self.home_score > self.away_score {
                return Some(FootballMatchupResult::Loss);
            } else {
                return Some(FootballMatchupResult::Win);
            }
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
            team_id: team_id,
            matchups: matchups
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
        for matchup in self.matchups.iter() {
            match matchup {
                Some(m) => match m.result(self.team_id) {
                    Some(r) => match r {
                        FootballMatchupResult::Win => record.increment_wins(1),
                        FootballMatchupResult::Loss => record.increment_losses(1),
                        FootballMatchupResult::Tie => record.increment_ties(1)
                    },
                    None => ()
                },
                None => ()
            }
        }
        record
    }
}
