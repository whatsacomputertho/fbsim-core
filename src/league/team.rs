use serde::{Serialize, Deserialize};

/// # `LeagueTeam` struct
///
/// A `LeagueTeam` represents a football team in a football league.
/// Since a team's properties (skill levels, team name, etc.) can change
/// over the course of many seasons, this struct is mainly just used
/// as a unique ID for a given team
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueTeam {
    wins: usize,
    losses: usize
}

impl LeagueTeam {
    /// Constructor for the `LeagueTeam` struct in which the team ID is
    /// supplied by the caller, and the wins and losses are zeroed.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// ```
    pub fn new() -> LeagueTeam {
        LeagueTeam{
            wins: 0,
            losses: 0
        }
    }

    /// Getter for the league team's wins
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_league_wins = my_league_team.wins();
    /// ```
    pub fn wins(&self) -> &usize {
        &self.wins
    }

    /// Mutable getter for the league team's wins
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let mut my_league_team = LeagueTeam::new();
    /// let mut my_league_wins = my_league_team.wins_mut();
    /// ```
    pub fn wins_mut(&mut self) -> &mut usize {
        &mut self.wins
    }

    /// Getter for the league team's losses
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_league_losses = my_league_team.losses();
    /// ```
    pub fn losses(&self) -> &usize {
        &self.losses
    }

    /// Mutable getter for the league team's losses
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::team::LeagueTeam;
    ///
    /// let mut my_league_team = LeagueTeam::new();
    /// let mut my_league_losses = my_league_team.losses_mut();
    /// ```
    pub fn losses_mut(&mut self) -> &mut usize {
        &mut self.losses
    }
}