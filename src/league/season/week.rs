#![doc = include_str!("../../../docs/league/season/week.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::league::season::matchup::LeagueSeasonMatchup;

/// # `LeagueSeasonWeek` struct
///
/// A `LeagueSeasonWeek` represents a week of a football season
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonWeek {
    matchups: Vec<LeagueSeasonMatchup>
}

impl Default for LeagueSeasonWeek {
    /// Default constructor for the LeagueSeasonWeek struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let my_week = LeagueSeasonWeek::default();
    /// ```
    fn default() -> Self {
        LeagueSeasonWeek {
            matchups: Vec::new()
        }
    }
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
        LeagueSeasonWeek::default()
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

    /// Determine based on the matchups whether the week has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let my_week = LeagueSeasonWeek::new();
    /// let started = my_week.started();
    /// ```
    pub fn started(&self) -> bool {
        // If no matchups, then the week hasn't started
        if self.matchups.is_empty() {
            return false;
        }

        // Loop through the matchups and check if any are complete
        for matchup in self.matchups.iter() {
            if *matchup.complete() {
                return true;
            }
        }
        false
    }

    /// Determine based on the matchups whether the week has completed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::week::LeagueSeasonWeek;
    ///
    /// let my_week = LeagueSeasonWeek::new();
    /// let complete = my_week.complete();
    /// ```
    pub fn complete(&self) -> bool {
        // If no matchups, then the week hasn't started
        if self.matchups.is_empty() {
            return false;
        }

        // Loop through the matchups and check if any are not complete
        for matchup in self.matchups.iter() {
            if !matchup.complete() {
                return false;
            }
        }
        true
    }

    /// Get a matchup involving a team
    pub fn team_matchup(&self, id: usize) -> Option<LeagueSeasonMatchup> {
        for matchup in self.matchups.iter() {
            if matchup.participated(id) {
                return Some(matchup.clone());
            }
        }
        None
    }
}
