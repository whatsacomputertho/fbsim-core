use serde::{Serialize, Deserialize};

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;

/// # `LeagueTeam` struct
///
/// A `LeagueTeam` represents a football team in a football league.
/// Since a team's properties (skill levels, team name, etc.) can change
/// over the course of many seasons, this struct is mainly just used
/// as a unique ID for a given team
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueTeam {}

impl LeagueTeam {
    /// Constructor for the `LeagueTeam` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// ```
    pub fn new() -> LeagueTeam {
        LeagueTeam{}
    }
}
