use serde::{Serialize, Deserialize};

use crate::league::season::matchup::LeagueSeasonMatchup;

/// # `LeagueSeasonWeek` struct
///
/// A `LeagueSeasonWeek` represents a week of a football season
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonWeek {
    matchups: Vec<LeagueSeasonMatchup>
}

impl LeagueSeasonWeek {
    /// Constructor for the LeagueSeasonWeek struct, with the week containing
    /// no matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let my_week = LeagueSeasonWeek::new();
    /// ```
    pub fn new() -> LeagueSeasonWeek {
        LeagueSeasonWeek {
            matchups: Vec::new()
        }
    }

    /// Borrow the matchups for the week
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let my_week = LeagueSeasonWeek::new();
    /// let my_matchups = my_week.matchups();
    /// ```
    pub fn matchups(&self) -> &Vec<LeagueSeasonMatchup> {
        &self.matchups
    }

    /// Mutably borrow the matchups for the week
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let mut my_week = LeagueSeasonWeek::new();
    /// let mut my_matchups = my_week.matchups_mut();
    /// ```
    pub fn matchups_mut(&mut self) -> &mut Vec<LeagueSeasonMatchup> {
        &mut self.matchups
    }
}
