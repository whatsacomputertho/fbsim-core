use std::collections::BTreeMap;
use std::fmt;

use crate::league::season::matchup::LeagueSeasonMatchups;

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
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
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
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
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
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_wins(1);
    /// let wins = my_record.wins();
    /// assert!(*wins == 1);
    /// ```
    pub fn increment_wins(&mut self, n: usize) {
        self.wins += n
    }

    /// Borrow the loss count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
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
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_losses(1);
    /// let losses = my_record.losses();
    /// assert!(*losses == 1);
    /// ```
    pub fn increment_losses(&mut self, n: usize) {
        self.losses += n
    }

    /// Borrow the tie count
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
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
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    ///
    /// let mut my_record = LeagueTeamRecord::new();
    /// my_record.increment_ties(1);
    /// let ties = my_record.ties();
    /// assert!(*ties == 1);
    /// ```
    pub fn increment_ties(&mut self, n: usize) {
        self.ties += n
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
pub struct LeagueMatchups {
    matchups: BTreeMap<usize, LeagueSeasonMatchups>
}

impl LeagueMatchups {
    /// Instantiate a new LeagueMatchups struct
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::matchup::LeagueMatchups;
    ///
    /// let my_matchups = LeagueMatchups::new(BTreeMap::new());
    /// ```
    pub fn new(matchups: BTreeMap<usize, LeagueSeasonMatchups>) -> LeagueMatchups {
        LeagueMatchups{
            matchups: matchups
        }
    }

    /// Borrow the season matchups
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::matchup::LeagueMatchups;
    /// 
    /// let my_matchups = LeagueMatchups::new(BTreeMap::new());
    /// let matchups = my_matchups.matchups();
    /// ```
    pub fn matchups(&self) -> &BTreeMap<usize, LeagueSeasonMatchups> {
        &self.matchups
    }

    /// Compute the team record
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::matchup::{LeagueMatchups, LeagueTeamRecord};
    ///
    /// let my_matchups = LeagueMatchups::new(BTreeMap::new());
    /// let record = my_matchups.record();
    /// assert!(record == LeagueTeamRecord::new());
    /// ```
    pub fn record(&self) -> LeagueTeamRecord {
        // Initialize a new LeagueTeamRecord
        let mut record = LeagueTeamRecord::new();

        // Loop through the matchups and increment the team record
        for (_, season) in self.matchups.iter() {
            let season_record = season.record();
            record.increment_wins(*season_record.wins());
            record.increment_losses(*season_record.losses());
            record.increment_ties(*season_record.ties());
        }
        record
    }
}
