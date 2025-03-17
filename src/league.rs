use crate::team::FootballTeam;

/// # `LeagueTeam` struct
///
/// A `LeagueTeam` represents a football team in a football league.
/// In addition to a team's attributes, the team's wins and losses
/// are also tracked.
pub struct LeagueTeam {
    team: FootballTeam,
    wins: i32,
    losses: i32
}

impl LeagueTeam {
    /// Constructor for the `LeagueTeam` struct in which the underlying
    /// FootballTeam is instantiated as the default team, and the wins
    /// and losses are zeroed.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// ```
    pub fn new() -> LeagueTeam {
        LeagueTeam{
            team: FootballTeam::new(),
            wins: 0,
            losses: 0
        }
    }

    /// Getter for the league team's underlying `FootballTeam`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_football_team = my_league_team.team();
    /// ```
    pub fn team(&self) -> &FootballTeam {
        &self.team
    }

    /// Mutable getter for the league team's underlying `FootballTeam`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueTeam;
    ///
    /// let mut my_league_team = LeagueTeam::new();
    /// let mut my_football_team = my_league_team.team_mut();
    /// ```
    pub fn team_mut(&mut self) -> &mut FootballTeam {
        &mut self.team
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

/// # `LeagueMatchup` struct
///
/// A `LeagueMatchup` represents a football matchup in a football league.
/// It contains references to the home and away league teams, and tracks
/// whether the matchup was completed, and whether the home team won.
pub struct LeagueMatchup<'a> {
    home_team: &'a LeagueTeam,
    away_team: &'a LeagueTeam,
    complete: bool,
    home_win: bool
}

impl<'a> LeagueMatchup<'a> {
    /// Constructor for the `LeagueMatchup` struct in which home and away
    /// team references are given. The matchup is set as not complete by
    /// default.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// ```
    pub fn new(home_team: &'a LeagueTeam, away_team: &'a LeagueTeam) -> LeagueMatchup<'a> {
        LeagueMatchup{
            home_team: home_team,
            away_team: away_team,
            complete: false,
            home_win: false
        }
    }

    /// Getter for the matchup's home team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_home_team_ref = my_league_matchup.home_team();
    /// ```
    pub fn home_team(&self) -> &'a LeagueTeam {
        &self.home_team
    }

    /// Getter for the matchup's away team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_away_team_ref = my_league_matchup.away_team();
    /// ```
    pub fn away_team(&self) -> &'a LeagueTeam {
        &self.away_team
    }

    /// Getter for the matchup's complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_matchup_complete = my_league_matchup.complete();
    /// ```
    pub fn complete(&self) -> &bool {
        &self.complete
    }

    /// Mutable getter for the matchup's complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let mut my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let mut my_matchup_complete = my_league_matchup.complete_mut();
    /// ```
    pub fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }

    /// Getter for the matchup's home win property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_matchup_home_win = my_league_matchup.home_win();
    /// ```
    pub fn home_win(&self) -> &bool {
        &self.home_win
    }

    /// Mutable getter for the matchup's home win property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup};
    ///
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let mut my_league_matchup = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let mut my_matchup_home_win = my_league_matchup.home_win_mut();
    /// ```
    pub fn home_win_mut(&mut self) -> &mut bool {
        &mut self.home_win
    }
}

/// # `LeagueMatchupWeek` struct
///
/// A `LeagueMatchupWeek` represents a week's worth of football matchups in a
/// football league. It contains a vector of `LeagueMatchup` structs.
pub struct LeagueMatchupWeek<'a> {
    matchups: Vec<LeagueMatchup<'a>>
}

impl<'a> LeagueMatchupWeek<'a> {
    /// Getter for the vector of league matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup, LeagueMatchupWeek};
    ///
    /// // Instantiate the league teams
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_team_3 = LeagueTeam::new();
    /// let my_league_team_4 = LeagueTeam::new();
    ///
    /// // Instantiate a week of league matchups
    /// let my_league_matchup_1 = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_league_matchup_2 = LeagueMatchup::new(&my_league_team_3, &my_league_team_4);
    ///
    /// // Add to a vector of league matchups
    /// let mut my_league_matchups: Vec<LeagueMatchup> = Vec::new();
    /// my_league_matchups.push(my_league_matchup_1);
    /// my_league_matchups.push(my_league_matchup_2);
    /// 
    /// // Instantiate the week of matchups from the vector
    /// let my_league_matchup_week = LeagueMatchupWeek::from(my_league_matchups);
    ///
    /// // Borrow the matchups vector
    /// let my_borrowed_matchups = my_league_matchup_week.matchups();
    /// ```
    pub fn matchups(&self) -> &Vec<LeagueMatchup<'a>> {
        &self.matchups
    }

    /// Mutable getter for the vector of league matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup, LeagueMatchupWeek};
    ///
    /// // Instantiate the league teams
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_team_3 = LeagueTeam::new();
    /// let my_league_team_4 = LeagueTeam::new();
    ///
    /// // Instantiate a week of league matchups
    /// let my_league_matchup_1 = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_league_matchup_2 = LeagueMatchup::new(&my_league_team_3, &my_league_team_4);
    ///
    /// // Add to a vector of league matchups
    /// let mut my_league_matchups: Vec<LeagueMatchup> = Vec::new();
    /// my_league_matchups.push(my_league_matchup_1);
    /// my_league_matchups.push(my_league_matchup_2);
    /// 
    /// // Instantiate the week of matchups from the vector
    /// let mut my_league_matchup_week = LeagueMatchupWeek::from(my_league_matchups);
    ///
    /// // Borrow the matchups vector
    /// let mut my_borrowed_matchups = my_league_matchup_week.matchups_mut();
    /// ```
    pub fn matchups_mut(&mut self) -> &mut Vec<LeagueMatchup<'a>> {
        &mut self.matchups
    }
}

impl<'a> From<Vec<LeagueMatchup<'a>>> for LeagueMatchupWeek<'a> {
    /// From trait implementation for instantiating a league matchup week
    /// from a vector of league matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueMatchup, LeagueMatchupWeek};
    ///
    /// // Instantiate the league teams
    /// let my_league_team_1 = LeagueTeam::new();
    /// let my_league_team_2 = LeagueTeam::new();
    /// let my_league_team_3 = LeagueTeam::new();
    /// let my_league_team_4 = LeagueTeam::new();
    ///
    /// // Instantiate a week of league matchups
    /// let my_league_matchup_1 = LeagueMatchup::new(&my_league_team_1, &my_league_team_2);
    /// let my_league_matchup_2 = LeagueMatchup::new(&my_league_team_3, &my_league_team_4);
    ///
    /// // Add to a vector of league matchups
    /// let mut my_league_matchups: Vec<LeagueMatchup> = Vec::new();
    /// my_league_matchups.push(my_league_matchup_1);
    /// my_league_matchups.push(my_league_matchup_2);
    /// 
    /// // Instantiate the week of matchups from the vector
    /// let my_league_matchup_week = LeagueMatchupWeek::from(my_league_matchups);
    /// ```
    fn from(matchups: Vec<LeagueMatchup<'a>>) -> Self {
        LeagueMatchupWeek{
            matchups: matchups
        }
    }
}

/// # `League` struct
///
/// A `League` represents a football league. It contains a vector of teams in
/// the league as `LeagueTeam` objects. It also contains a vector of weeks of
/// games as `LeagueMatchupWeek` objects.
pub struct League<'a> {
    teams: Vec<LeagueTeam>,
    schedule: Vec<LeagueMatchupWeek<'a>>
}
