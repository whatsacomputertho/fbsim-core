pub mod matchup;
pub mod team;
pub mod week;

use std::collections::BTreeMap;

use crate::league::season::team::LeagueSeasonTeam;
use crate::league::season::week::LeagueSeasonWeek;

use chrono::Datelike;
use serde::{Serialize, Deserialize, Deserializer};

/// # `LeagueSeasonRaw` struct
///
/// A `LeagueSeasonRaw` represents a freshly deserialized `LeagueSeason` prior
/// to any validation of the type having been completed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonRaw {
    pub year: usize,
    pub teams: BTreeMap<usize, LeagueSeasonTeam>,
    pub weeks: Vec<LeagueSeasonWeek>,
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

        // TODO: Validation for weeks

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
    weeks: Vec<LeagueSeasonWeek>,
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
                weeks: item.weeks,
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
            weeks: Vec::new(),
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
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
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
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
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
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
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
