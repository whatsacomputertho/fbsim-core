use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

/// # `LeagueTeam` struct
///
/// A `LeagueTeam` represents a football team in a football league.
/// Since a team's properties (skill levels, team name, etc.) can change
/// over the course of many seasons, this struct is mainly just used
/// as a unique ID for a given team
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct LeagueTeam {
    wins: i32,
    losses: i32
}

impl LeagueTeam {
    /// Constructor for the `LeagueTeam` struct in which the team ID is
    /// supplied by the caller, and the wins and losses are zeroed.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
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
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_league_wins = my_league_team.wins();
    /// ```
    pub fn wins(&self) -> &i32 {
        &self.wins
    }

    /// Mutable getter for the league team's wins
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let mut my_league_team = LeagueTeam::new();
    /// let mut my_league_wins = my_league_team.wins_mut();
    /// ```
    pub fn wins_mut(&mut self) -> &mut i32 {
        &mut self.wins
    }

    /// Getter for the league team's losses
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_league_losses = my_league_team.losses();
    /// ```
    pub fn losses(&self) -> &i32 {
        &self.losses
    }

    /// Mutable getter for the league team's losses
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let mut my_league_team = LeagueTeam::new();
    /// let mut my_league_losses = my_league_team.losses_mut();
    /// ```
    pub fn losses_mut(&mut self) -> &mut i32 {
        &mut self.losses
    }
}

/// # `League` struct
///
/// A `League` represents a football league. It contains a vector of teams in
/// the league as `LeagueTeam` objects. It also contains the season that is
/// currently in-progress, and it contains a vector of past seasons
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct League {
    teams: BTreeMap<usize, LeagueTeam>
}

impl League {
    /// Constructor for the `League` struct in which the vec of league
    /// teams is empty
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// let my_league = League::new();
    /// ```
    pub fn new() -> League {
        League{
            teams: BTreeMap::new()
        }
    }

    /// Adds a `LeagueTeam` to a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// let mut my_league = League::new();
    /// my_league.add_team();
    /// ```
    pub fn add_team(&mut self) -> () {
        // Get the last item in the BTreeMap, which is auto-sorted by ID
        if let Some((&max_id, _)) = self.teams.iter().next_back() {
            // The list is non-empty and has a max ID
            self.teams.insert(max_id + 1, LeagueTeam::new());
        } else {
            // The list is empty
            self.teams.insert(0, LeagueTeam::new());
        }
    }

    /// Borrows the BTreeMap of teams immutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the BTreeMap of LeagueTeams
    /// let my_teams = my_league.teams();
    /// ```
    pub fn teams(&self) -> &BTreeMap<usize, LeagueTeam> {
        &self.teams
    }


    /// Borrows the BTreeMap of teams immutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the BTreeMap of LeagueTeams
    /// let mut my_teams = my_league.teams_mut();
    /// ```
    pub fn teams_mut(&mut self) -> &mut BTreeMap<usize, LeagueTeam> {
        &mut self.teams
    }

    /// Borrows an immutable `LeagueTeam` from a `League` given the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the LeagueTeam with ID 2
    /// let my_team = my_league.team(2);
    /// ```
    pub fn team(&self, id: usize) -> Option<&LeagueTeam> {
        self.teams.get(&id)
    }

    /// Borrows a mutable `LeagueTeam` from a `League` given the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the LeagueTeam with ID 1
    /// let mut my_team = my_league.team_mut(1);
    /// ```
    pub fn team_mut(&mut self, id: usize) -> Option<&mut LeagueTeam> {
        self.teams.get_mut(&id)
    }
}
