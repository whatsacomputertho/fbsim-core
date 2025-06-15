use std::fmt;

use crate::matchup::FootballMatchupResult;
use crate::league::season::matchup::LeagueSeasonMatchup;

/// # `LeagueTeamRecord` type
///
/// A 3-tuple of usizes representing the number of wins, losses, and ties
/// for a given team.  May be for a season or for many seasons.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LeagueTeamRecord {
    wins: usize,
    losses: usize,
    ties: usize
}

impl LeagueTeamRecord {
    /// Constructor for the LeagueTeamRecord type
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let my_record = LeagueTeamRecord::new();
    /// ```
    pub fn new() -> LeagueTeamRecord {
        LeagueTeamRecord{
            wins: 0,
            losses: 0,
            ties: 0
        }
    }

    /// Borrow the win count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let my_record = LeagueTeamRecord::new();
    /// let wins = my_record.wins();
    /// assert!(*wins == 0);
    /// ```
    pub fn wins(&self) -> &usize {
        &self.wins
    }

    /// Increment the win count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_wins();
    /// let wins = my_record.wins();
    /// assert!(*wins == 1);
    /// ```
    pub fn increment_wins(&mut self) {
        self.wins += 1
    }

    /// Borrow the loss count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let my_record = LeagueTeamRecord::new();
    /// let losses = my_record.losses();
    /// assert!(*losses == 0);
    /// ```
    pub fn losses(&self) -> &usize {
        &self.losses
    }

    /// Increment the loss count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_losses();
    /// let losses = my_record.losses();
    /// assert!(*losses == 1);
    /// ```
    pub fn increment_losses(&mut self) {
        self.losses += 1
    }

    /// Borrow the tie count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let my_record = LeagueTeamRecord::new();
    /// let ties = my_record.ties();
    /// assert!(*ties == 0);
    /// ```
    pub fn ties(&self) -> &usize {
        &self.ties
    }

    /// Increment the tie count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_ties();
    /// let ties = my_record.ties();
    /// assert!(*ties == 1);
    /// ```
    pub fn increment_ties(&mut self) {
        self.ties += 1
    }
}

impl fmt::Display for LeagueTeamRecord {
    /// Display a LeagueTeamRecord as a string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.wins, self.losses, self.ties)
    }
}

/// # `LeagueMatchups` struct
///
/// Represents a list of matchups for a given team during a given season
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LeagueMatchups<'a> {
    team_id: usize,
    matchups: Vec<Option<&'a LeagueSeasonMatchup>>
}

impl<'a> LeagueMatchups<'a> {
    /// Instantiate a new LeagueMatchups struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::LeagueMatchups;
    ///
    /// let my_matchups = LeagueMatchups::new(0, Vec::new());
    /// ```
    pub fn new(team_id: usize, matchups: Vec<Option<&'a LeagueSeasonMatchup>>) -> LeagueMatchups {
        LeagueMatchups{
            team_id: team_id,
            matchups: matchups
        }
    }

    /// Compute the team record
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchups::{LeagueMatchups, LeagueTeamRecord};
    ///
    /// let my_matchups = LeagueMatchups::new(0, Vec::new());
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
                        FootballMatchupResult::Win => record.increment_wins(),
                        FootballMatchupResult::Loss => record.increment_losses(),
                        FootballMatchupResult::Tie => record.increment_ties()
                    },
                    None => ()
                },
                None => ()
            }
        }
        record
    }
}
