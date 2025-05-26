use std::collections::BTreeMap;

use chrono::Datelike;
use serde::{Serialize, Deserialize};

/// # `LeagueTeam` struct
///
/// A `LeagueTeam` represents a football team in a football league.
/// Since a team's properties (skill levels, team name, etc.) can change
/// over the course of many seasons, this struct is mainly just used
/// as a unique ID for a given team
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
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

/// # `LeagueSeasonTeam` struct
///
/// A `LeagueSeasonTeam` represents a team during a season of a football leauge
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonTeam<'a> {
    name: String,
    logo: String,
    offense_overall: i32,
    defense_overall: i32,

    #[serde(skip_serializing,skip_deserializing)]
    team: Option<&'a LeagueTeam>
}

impl<'a> LeagueSeasonTeam<'a> {
    /// Constructor for the `LeagueSeasonTeam` struct in which the league team
    /// reference is given.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{LeagueTeam, LeagueSeasonTeam};
    ///
    /// let my_league_team = LeagueTeam::new();
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50, &my_league_team);
    /// ```
    pub fn new(name: String, logo: String, offense_overall: i32, defense_overall: i32, team: &'a LeagueTeam) -> LeagueSeasonTeam<'a> {
        LeagueSeasonTeam{
            name: name, 
            logo: logo,
            offense_overall: offense_overall,
            defense_overall: defense_overall,
            team: Some(team)
        }
    }
}

/// # `LeagueSeason` struct
///
/// A `LeagueSeason` represents a season of a football league.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeason<'a> {
    year: usize,
    teams: BTreeMap<usize, LeagueSeasonTeam<'a>>,
    complete: bool
}

impl<'a> LeagueSeason<'a> {
    /// Constructor for the `LeagueSeason` struct, with the year
    /// defaulting to the current year
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// ```
    pub fn new() -> LeagueSeason<'a> {
        LeagueSeason{
            year: chrono::Utc::now().year() as usize,
            teams: BTreeMap::new(),
            complete: false
        }
    }

    /// Borrow the year the season took place
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let my_season_year = my_league_season.year();
    /// ```
    pub fn year(&self) -> &usize {
        &self.year
    }

    /// Mutably borrow the year the season took place
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut my_season_year = my_league_season.year_mut();
    /// ```
    pub fn year_mut(&mut self) -> &mut usize {
        &mut self.year
    }

    /// Borrow the teams which competed in the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let my_season_teams = my_league_season.teams();
    /// ```
    pub fn teams(&self) -> &BTreeMap<usize, LeagueSeasonTeam<'a>> {
        &self.teams
    }

    /// Mutably borrow the year the season took place
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut my_season_teams = my_league_season.teams_mut();
    /// ```
    pub fn teams_mut(&mut self) -> &mut BTreeMap<usize, LeagueSeasonTeam<'a>> {
        &mut self.teams
    }

    /// Borrows an immutable `LeagueSeasonTeam` from a `LeagueSeason` given
    /// the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{League, LeagueSeason, LeagueTeam, LeagueSeasonTeam};
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new LeagueSeasonTeam
    /// let my_league_team = LeagueTeam::new();
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50, &my_league_team);
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let my_season_team = my_league_season.team(2);
    /// ```
    pub fn team(&self, id: usize) -> Option<&LeagueSeasonTeam<'a>> {
        self.teams.get(&id)
    }

    /// Borrows a mutable `LeagueSeasonTeam` from a `LeagueSeason` given the
    /// team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{League, LeagueSeason, LeagueTeam, LeagueSeasonTeam};
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new LeagueSeasonTeam
    /// let my_league_team = LeagueTeam::new();
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50, &my_league_team);
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let mut my_season_team = my_league_season.team_mut(2);
    /// ```
    pub fn team_mut(&mut self, id: usize) -> Option<&mut LeagueSeasonTeam<'a>> {
        self.teams.get_mut(&id)
    }

    /// Borrow the value indicating whether the season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let complete = my_league_season.complete();
    /// ```
    pub fn complete(&self) -> &bool {
        &self.complete
    }

    /// Mutably borrow the value indicating whether the season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut complete = my_league_season.complete_mut();
    /// ```
    pub fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }
}

/// # `League` struct
///
/// A `League` represents a football league. It contains a vector of teams in
/// the league as `LeagueTeam` objects. It also contains the season that is
/// currently in-progress, and it contains a vector of past seasons
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct League<'a> {
    teams: BTreeMap<usize, LeagueTeam>,
    current_season: Option<LeagueSeason<'a>>,
    seasons: Vec<LeagueSeason<'a>>
}

impl<'a> League<'a> {
    /// Constructor for the `League` struct in which the vec of league
    /// teams is empty
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// let my_league = League::new();
    /// ```
    pub fn new() -> League<'a> {
        League{
            teams: BTreeMap::new(),
            current_season: None,
            seasons: Vec::new()
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

    /// Borrows a season from a `League` identified by its year
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::{League, LeagueSeasonTeam};
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add some teams to the new League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a map of LeagueSeasonTeams corresponding to league teams
    /// let mut my_season_teams = BTreeMap::new();
    ///
    /// // Add some LeagueSeasonTeams to the map
    /// let season_team_0 = my_league.team(0).unwrap().clone();
    /// let season_team_1 = my_league.team(1).unwrap().clone();
    /// let season_team_3 = my_league.team(3).unwrap().clone();
    /// let season_team_4 = my_league.team(4).unwrap().clone();
    /// my_season_teams.insert(0, LeagueSeasonTeam::new("My Team 0".to_string(), "".to_string(), 50, 50, &season_team_0));
    /// my_season_teams.insert(1, LeagueSeasonTeam::new("My Team 1".to_string(), "".to_string(), 50, 50, &season_team_1));
    /// my_season_teams.insert(3, LeagueSeasonTeam::new("My Team 3".to_string(), "".to_string(), 50, 50, &season_team_3));
    /// my_season_teams.insert(4, LeagueSeasonTeam::new("My Team 4".to_string(), "".to_string(), 50, 50, &season_team_4));
    ///
    /// // Create a new season in the league
    /// let _ = my_league.create_season(my_season_teams);
    ///
    /// // Borrow the past seasons from the League
    /// let my_season = my_league.season(2025);
    /// ```
    pub fn season(&self, year: usize) -> Option<&LeagueSeason<'a>> {
        // If the year corresponds to the current season, return it
        if let Some(season) = self.current_season() {
            if *season.year() == year {
                return Some(season);
            }
        }

        // Otherwise search for it in the past seasons
        for season in self.seasons().iter() {
            if *season.year() == year {
                return Some(season);
            }
        }
        return None
    }

    /// Borrows the past seasons from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let my_league = League::new();
    ///
    /// // Borrow the past seasons from the League
    /// let past_seasons = my_league.seasons();
    /// ```
    pub fn seasons(&self) -> &Vec<LeagueSeason<'a>> {
        &self.seasons
    }

    /// Mutably borrows the past seasons from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Borrow the past seasons from the League
    /// let mut past_seasons = my_league.seasons_mut();
    /// ```
    pub fn seasons_mut(&mut self) -> &mut Vec<LeagueSeason<'a>> {
        &mut self.seasons
    }

    /// Borrows the current season from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{League, LeagueSeason};
    ///
    /// // Instantiate a new League
    /// let my_league = League::new();
    ///
    /// // Borrow the current season from the League
    /// let my_season: &Option<LeagueSeason> = my_league.current_season();
    /// ```
    pub fn current_season(&self) -> &Option<LeagueSeason<'a>> {
        &self.current_season
    }

    /// Mutably borrows the current season from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::{League, LeagueSeason};
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Borrow the current season from the `League`
    /// let mut my_season: &mut Option<LeagueSeason> = my_league.current_season_mut();
    /// ```
    pub fn current_season_mut(&mut self) -> &mut Option<LeagueSeason<'a>> {
        &mut self.current_season
    }

    /// Gets the most recent year among the completed seasons
    fn most_recent_year(&self) -> usize {
        let mut most_recent_year = 0_usize;
        let seasons = self.seasons();
        for season in seasons.iter() {
            let season_year = season.year();
            if *season_year > most_recent_year {
                most_recent_year = season_year.clone();
            }
        }
        most_recent_year
    }

    /// Creates a new `LeagueSeason` and archives the current `LeagueSeason`
    /// if the current `LeagueSeason` is complete
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::{League, LeagueSeasonTeam};
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add some teams to the new League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a map of LeagueSeasonTeams corresponding to league teams
    /// let mut my_season_teams = BTreeMap::new();
    ///
    /// // Add some LeagueSeasonTeams to the map
    /// let season_team_0 = my_league.team(0).unwrap().clone();
    /// let season_team_1 = my_league.team(1).unwrap().clone();
    /// let season_team_3 = my_league.team(3).unwrap().clone();
    /// let season_team_4 = my_league.team(4).unwrap().clone();
    /// my_season_teams.insert(0, LeagueSeasonTeam::new("My Team 0".to_string(), "".to_string(), 50, 50, &season_team_0));
    /// my_season_teams.insert(1, LeagueSeasonTeam::new("My Team 1".to_string(), "".to_string(), 50, 50, &season_team_1));
    /// my_season_teams.insert(3, LeagueSeasonTeam::new("My Team 3".to_string(), "".to_string(), 50, 50, &season_team_3));
    /// my_season_teams.insert(4, LeagueSeasonTeam::new("My Team 4".to_string(), "".to_string(), 50, 50, &season_team_4));
    ///
    /// // Create a new season for the new League
    /// let res = my_league.create_season(my_season_teams);
    /// ```
    pub fn create_season(&mut self, teams: BTreeMap<usize, LeagueSeasonTeam<'a>>) -> Result<(), String> {
        // Check if the given list of teams is valid
        let num_teams = teams.len();
        if num_teams < 4 {
            return Err(
                format!(
                    "A league must have at least 4 teams, found {}",
                    num_teams
                )
            )
        }
        if num_teams % 2 != 0 {
            return Err(
                format!(
                    "A league must have an even number of teams, found {}",
                    num_teams
                )
            )
        }
        for (i, (i_id, _team_i) )in teams.iter().enumerate() {
            let league_team_i = self.team(*i_id);
            match league_team_i {
                Some(_league_team) => (),
                None => return Err(format!("No team exists with ID {}", i_id)),
            }
            for (j, (j_id, _team_j)) in teams.iter().enumerate() {
                if i == j {
                    continue
                }
                if i_id == j_id {
                    return Err(
                        format!(
                            "All league teams must have unique IDs, but IDs match for team {} and {}: {}",
                            i, j, i_id
                        )
                    )
                }
            }
        }

        // Check if the current season exists
        if let Some(season) = self.current_season_mut() {
            // If so, then check if the season is complete
            if *season.complete() {
                // If the season is complete, archive and create new season
                let most_recent_year = season.year();
                let mut new_season = LeagueSeason::new();
                let new_year = new_season.year_mut();
                *new_year = most_recent_year + 1;
                let season_teams = new_season.teams_mut();
                for (id, team) in teams.iter() {
                    season_teams.insert(*id, team.clone());
                }
                let old_season = season.clone();
                *season = new_season;
                self.seasons.push(old_season);
                return Ok(());
            }

            // If the season is not complete, then error
            return Err(
                format!(
                    "Cannot create new season: {}",
                    "Current season still in progress"
                )
            );
        }

        // Create a new league season and add the teams to the season
        let mut new_season = LeagueSeason::new();
        let season_teams = new_season.teams_mut();
        for (id, team) in teams.iter() {
            season_teams.insert(*id, team.clone());
        }

        // If the past seasons list is empty then stick with the default year
        if self.seasons.len() == 0 {
            self.current_season = Some(new_season);
            return Ok(());
        }

        // If the past seasons list is nonempty then update the year
        let most_recent_year = self.most_recent_year();
        let new_year = new_season.year_mut();
        *new_year = most_recent_year + 1;
        self.current_season = Some(new_season);
        return Ok(());
    }
}
