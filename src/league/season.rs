use std::collections::BTreeMap;

use chrono::Datelike;
use serde::{Serialize, Deserialize, Deserializer};

/// # `LeagueSeasonTeam` struct
///
/// A `LeagueSeasonTeam` represents a team during a season of a football leauge
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonTeam {
    name: String,
    logo: String,
    offense_overall: usize,
    defense_overall: usize
}

impl LeagueSeasonTeam {
    /// Constructor for the `LeagueSeasonTeam` struct in which the league team
    /// reference is given.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// ```
    pub fn new(name: String, logo: String, offense_overall: usize, defense_overall: usize) -> LeagueSeasonTeam {
        LeagueSeasonTeam{
            name: name, 
            logo: logo,
            offense_overall: offense_overall,
            defense_overall: defense_overall
        }
    }

    /// Borrow the season team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let team_name = my_season_team.name();
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Mutably borrow the season team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut team_name = my_season_team.name_mut();
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Borrow the season team logo
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let team_logo = my_season_team.logo();
    /// ```
    pub fn logo(&self) -> &String {
        &self.logo
    }

    /// Mutably borrow the season team logo
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut team_logo = my_season_team.logo_mut();
    /// ```
    pub fn logo_mut(&mut self) -> &mut String {
        &mut self.logo
    }

    /// Borrow the season team offensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let offense_overall = my_season_team.offense_overall();
    /// ```
    pub fn offense_overall(&self) -> &usize {
        &self.offense_overall
    }

    /// Mutably borrow the season team offensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut offense_overall = my_season_team.offense_overall_mut();
    /// ```
    pub fn offense_overall_mut(&mut self) -> &mut usize {
        &mut self.offense_overall
    }

    /// Borrow the season team defensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let defense_overall = my_season_team.defense_overall();
    /// ```
    pub fn defense_overall(&self) -> &usize {
        &self.defense_overall
    }

    /// Mutably borrow the season team defensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut defense_overall = my_season_team.defense_overall_mut();
    /// ```
    pub fn defense_overall_mut(&mut self) -> &mut usize {
        &mut self.defense_overall
    }
}

/// # `LeagueSeasonRaw` struct
///
/// A `LeagueSeasonRaw` represents a freshly deserialized `LeagueSeason` prior
/// to any validation of the type having been completed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonRaw {
    pub year: usize,
    pub teams: BTreeMap<usize, LeagueSeasonTeam>,
    pub started: bool,
    pub complete: bool
}

impl LeagueSeasonRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure the season is not both complete and not started
        if !self.started && self.complete {
            return Err("The season is both complete and not started".to_string());
        }

        // Ensure if the season is started or complete that there are an even
        // number of teams greater than 4
        if self.started || self.complete {
            let num_teams = self.teams.len();
            if num_teams < 4 {
                return Err(format!("The season has started, but has fewer than 4 team(s): {}", num_teams));
            }
            if num_teams % 2 != 0 {
                return Err(format!("The season has started, but has an odd number of teams: {}", num_teams));
            }
        }
        Ok(())
    }
}

/// # `LeagueSeason` struct
///
/// A `LeagueSeason` represents a season of a football league.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct LeagueSeason {
    year: usize,
    teams: BTreeMap<usize, LeagueSeasonTeam>,
    started: bool,
    complete: bool
}

impl TryFrom<LeagueSeasonRaw> for LeagueSeason {
    type Error = String;

    fn try_from(item: LeagueSeasonRaw) -> Result<Self, Self::Error> {
        // Validate the raw season
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            LeagueSeason{
                year: item.year,
                teams: item.teams,
                started: item.started,
                complete: item.complete
            }
        )
    }
}

impl<'de> Deserialize<'de> for LeagueSeason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = LeagueSeasonRaw::deserialize(deserializer)?;
        LeagueSeason::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl LeagueSeason {
    /// Constructor for the `LeagueSeason` struct, with the year
    /// defaulting to the current year
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// ```
    pub fn new() -> LeagueSeason {
        LeagueSeason{
            year: chrono::Utc::now().year() as usize,
            teams: BTreeMap::new(),
            started: false,
            complete: false
        }
    }

    /// Borrow the year the season took place
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
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
    /// use fbsim_core::league::season::LeagueSeason;
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
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let my_season_teams = my_league_season.teams();
    /// ```
    pub fn teams(&self) -> &BTreeMap<usize, LeagueSeasonTeam> {
        &self.teams
    }

    /// Mutably borrow the year the season took place
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut my_season_teams = my_league_season.teams_mut();
    /// ```
    pub fn teams_mut(&mut self) -> &mut BTreeMap<usize, LeagueSeasonTeam> {
        &mut self.teams
    }

    /// Add a team to the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::{LeagueSeason, LeagueSeasonTeam};
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// my_league_season.add_team(0, my_season_team);
    /// ```
    pub fn add_team(&mut self, id: usize, team: LeagueSeasonTeam) -> Result<(), String> {
        // Ensure the season has not already started
        if self.started || self.complete {
            return Err("Season has already started, cannot add new team".to_string());
        }

        // Ensure the given ID is unique
        if self.teams.contains_key(&id) {
            return Err(format!("Team with ID {} already exists", id));
        }

        // Add the team
        self.teams.insert(id, team);
        Ok(())
    }

    /// Borrows an immutable `LeagueSeasonTeam` from a `LeagueSeason` given
    /// the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::{LeagueSeason, LeagueSeasonTeam};
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new LeagueSeasonTeam
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let my_season_team = my_league_season.team(2);
    /// ```
    pub fn team(&self, id: usize) -> Option<&LeagueSeasonTeam> {
        self.teams.get(&id)
    }

    /// Borrows a mutable `LeagueSeasonTeam` from a `LeagueSeason` given the
    /// team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::{LeagueSeason, LeagueSeasonTeam};
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new LeagueSeasonTeam
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let mut my_season_team = my_league_season.team_mut(2);
    /// ```
    pub fn team_mut(&mut self, id: usize) -> Option<&mut LeagueSeasonTeam> {
        self.teams.get_mut(&id)
    }

    /// Borrow the value indicating whether the season has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let started = my_league_season.started();
    /// ```
    pub fn started(&self) -> &bool {
        &self.started
    }

    /// Mutably borrow the value indicating whether the season has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut complete = my_league_season.started_mut();
    /// ```
    pub fn started_mut(&mut self) -> &mut bool {
        &mut self.started
    }

    /// Borrow the value indicating whether the season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
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
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let mut complete = my_league_season.complete_mut();
    /// ```
    pub fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }
}