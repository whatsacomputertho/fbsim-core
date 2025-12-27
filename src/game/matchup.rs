#![doc = include_str!("../../docs/game/matchup.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::team::FootballTeam;

/// # `FootballMatchupResult` enum
///
/// Represents a result (win, loss, tie) of a football game
pub enum FootballMatchupResult {
    Win,
    Loss,
    Tie,
}

/// # `FootballMatchup` struct
///
/// A FootballMatchup represents a matchup between a home and away team
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Deserialize, Serialize)]
pub struct FootballMatchup {
    home: FootballTeam,
    away: FootballTeam
}

impl FootballMatchup {
    /// Returns the home team in the matchup
    pub fn home_team(&self) -> &FootballTeam {
        &self.home
    }

    /// Returns the away team in the matchup
    pub fn away_team(&self) -> &FootballTeam {
        &self.away
    }
}
