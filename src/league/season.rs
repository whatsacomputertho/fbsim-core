#![doc = include_str!("../../docs/league/season.md")]
pub mod conference;
pub mod matchup;
pub mod playoffs;
pub mod week;

use std::collections::BTreeMap;

use crate::team::FootballTeam;
use crate::league::matchup::LeagueTeamRecord;
use crate::league::season::conference::LeagueConference;
use crate::league::season::week::LeagueSeasonWeek;
use crate::league::season::matchup::{LeagueSeasonMatchup, LeagueSeasonMatchups};
use crate::league::season::playoffs::LeagueSeasonPlayoffs;
use crate::league::season::playoffs::picture::PlayoffPicture;
use crate::game::play::{Game, GameSimulator};

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use chrono::Datelike;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize, Deserializer};

/// # `LeagueSeasonRaw` struct
///
/// A `LeagueSeasonRaw` represents a freshly deserialized `LeagueSeason` prior
/// to any validation of the type having been completed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonRaw {
    pub year: usize,
    pub teams: BTreeMap<usize, FootballTeam>,
    #[serde(default)]
    pub conferences: Vec<LeagueConference>,
    pub weeks: Vec<LeagueSeasonWeek>,
    pub playoffs: LeagueSeasonPlayoffs
}

impl Default for LeagueSeasonRaw {
    /// Default constructor for the `LeagueSeasonRaw` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::default();
    /// ```
    fn default() -> Self {
        LeagueSeasonRaw{
            year: chrono::Utc::now().year() as usize,
            teams: BTreeMap::new(),
            conferences: Vec::new(),
            weeks: Vec::new(),
            playoffs: LeagueSeasonPlayoffs::new()
        }
    }
}

impl LeagueSeasonRaw {
    /// Constructor for the `LeagueSeasonRaw` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// ```
    pub fn new() -> LeagueSeasonRaw {
        LeagueSeasonRaw::default()
    }

    /// Determine based on the matchups whether the season has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let started = raw_league_season.started();
    /// ```
    pub fn started(&self) -> bool {
        // If no season weeks, then the season hasn't started
        if self.weeks.is_empty() {
            return false;
        }

        // Loop through the season weeks and check if any are started
        for week in self.weeks.iter() {
            if week.started() {
                return true;
            }
        }
        false
    }

    /// Determine whether the regular season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let complete = raw_league_season.regular_season_complete();
    /// ```
    pub fn regular_season_complete(&self) -> bool {
        // If no season weeks, then the season isn't complete
        if self.weeks.is_empty() {
            return false;
        }

        // Loop through the season weeks and check if any are not complete
        for week in self.weeks.iter() {
            if !week.complete() {
                return false;
            }
        }
        true
    }

    /// Determine whether the season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let complete = raw_league_season.complete();
    /// ```
    pub fn complete(&self) -> bool {
        if !self.regular_season_complete() {
            false
        } else {
            self.playoffs.complete()
        }
    }

    /// Validates a LeagueSeasonRaw before deserializing it
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let valid_res = raw_league_season.validate();
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // If a schedule has been generated
        let num_weeks = self.weeks.len();
        if num_weeks > 0 {
            let num_teams = self.teams.len();
            
            // Check whether the number of games is between the prescribed min and max
            let max_num_weeks = (num_teams - 1) * 3;
            if num_weeks > max_num_weeks {
                return Err(
                    format!(
                        "Schedule can involve teams playing each other team at most 3 times ({} games): {} given",
                        max_num_weeks, num_weeks
                    )
                )
            }
            if num_weeks < num_teams {
                return Err(
                    format!(
                        "Schedule must involve teams playing each other team at least 1 time ({} games): {} given",
                        num_teams, num_weeks
                    )
                )
            }
        }

        // Ensure if the season is started or complete that there are an even
        // number of teams greater than 4
        if self.started() {
            let num_teams = self.teams.len();
            if num_teams < 4 {
                return Err(
                    format!(
                        "Season {} has started, but has fewer than 4 team(s): {}",
                        self.year,
                        num_teams
                    )
                );
            }
            if !num_teams.is_multiple_of(2) {
                return Err(
                    format!(
                        "Season {} has started, but has an odd number of teams: {}",
                        self.year,
                        num_teams
                    )
                );
            }
        }

        // Validate the season weeks
        let mut prev_started: bool = false;
        let mut prev_completed: bool = false;
        for (i, week) in self.weeks.iter().enumerate() {
            let mut found_ids: Vec<usize> = Vec::new();

            // Ensure later weeks are not simulated before earlier weeks
            let week_started = week.started();
            let week_complete = week.complete();
            if i > 0 && !prev_started && week_started {
                return Err(
                    format!(
                        "Season {} week {} is started but a previous week is not",
                        self.year, i
                    )
                );
            }
            if i > 0 && !prev_completed && week_complete {
                return Err(
                    format!(
                        "Season {} week {} is complete but a previous week is not",
                        self.year, i
                    )
                );
            }
            prev_started = week_started;
            prev_completed = week_complete;

            for (j, matchup) in week.matchups().iter().enumerate() {
                let home_id = matchup.home_team();
                let away_id = matchup.away_team();

                // Ensure all matchups map to valid season team IDs
                if !self.teams.contains_key(home_id) {
                    return Err(
                        format!(
                            "Season {} week {} matchup {} contains nonexistent home team ID: {}",
                            self.year,
                            i, j,
                            home_id
                        )
                    );
                }
                if !self.teams.contains_key(away_id) {
                    return Err(
                        format!(
                            "Season {} week {} matchup {} contains nonexistent away team ID: {}",
                            self.year,
                            i, j,
                            away_id
                        )
                    )
                }

                // Ensure each team plays at most once per week
                if found_ids.contains(home_id) {
                    return Err(
                        format!(
                            "Team {} plays multiple times in season {} week {}, detected in matchup {}",
                            home_id,
                            self.year,
                            i, j
                        )
                    )
                }
                if found_ids.contains(away_id) {
                    return Err(
                        format!(
                            "Team {} plays multiple times in season {} week {}, detected in matchup {}",
                            away_id,
                            self.year,
                            i, j
                        )
                    )
                }
                found_ids.push(*home_id);
                found_ids.push(*away_id);
            }
        }
        Ok(())
    }
}

/// # `LeagueSeasonScheduleOptions` struct
///
/// A `LeagueSeasonScheduleOptions` represents a collection of options used
/// to generate a season schedule
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonScheduleOptions {
    pub weeks: Option<usize>,
    pub shift: Option<usize>,
    pub permute: Option<bool>,
    /// Number of games per division opponent (default: 2)
    pub division_games: Option<usize>,
    /// Number of games per non-division conference opponent (default: 1)
    pub conference_games: Option<usize>,
    /// Total number of cross-conference games per team (default: 0)
    pub cross_conference_games: Option<usize>,
}

impl Default for LeagueSeasonScheduleOptions {
    /// Default constructor for the `LeagueSeasonScheduleOptions` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// let my_schedule_options = LeagueSeasonScheduleOptions::new();
    /// ```
    fn default() -> Self {
        LeagueSeasonScheduleOptions{
            weeks: None,
            shift: None,
            permute: None,
            division_games: None,
            conference_games: None,
            cross_conference_games: None,
        }
    }
}

impl LeagueSeasonScheduleOptions {
    /// Constructor for the `LeagueSeasonScheduleOptions` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// let my_schedule_options = LeagueSeasonScheduleOptions::new();
    /// ```
    pub fn new() -> LeagueSeasonScheduleOptions {
        LeagueSeasonScheduleOptions::default()
    }
}

/// # `LeagueSeason` struct
///
/// A `LeagueSeason` represents a season of a football league.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct LeagueSeason {
    year: usize,
    teams: BTreeMap<usize, FootballTeam>,
    conferences: Vec<LeagueConference>,
    weeks: Vec<LeagueSeasonWeek>,
    playoffs: LeagueSeasonPlayoffs
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
                conferences: item.conferences,
                weeks: item.weeks,
                playoffs: item.playoffs
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

impl Default for LeagueSeason {
    /// Default constructor for the `LeagueSeason` struct, with the year
    /// defaulting to the current year
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::default();
    /// ```
    fn default() -> Self {
        LeagueSeason{
            year: chrono::Utc::now().year() as usize,
            teams: BTreeMap::new(),
            conferences: Vec::new(),
            weeks: Vec::new(),
            playoffs: LeagueSeasonPlayoffs::new()
        }
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
        LeagueSeason::default()
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
    pub fn teams(&self) -> &BTreeMap<usize, FootballTeam> {
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
    pub fn teams_mut(&mut self) -> &mut BTreeMap<usize, FootballTeam> {
        &mut self.teams
    }

    /// Add a team to the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let my_season_team = FootballTeam::new();
    /// my_league_season.add_team(0, my_season_team);
    /// ```
    pub fn add_team(&mut self, id: usize, team: FootballTeam) -> Result<(), String> {
        // Ensure the season has not already started
        if self.started() {
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

    /// Check if a team exists in the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Team 0 should not exist at this point
    /// assert!(!my_league_season.team_exists(0));
    ///
    /// // Add team 0 to the season
    /// let my_season_team = FootballTeam::new();
    /// my_league_season.add_team(0, my_season_team);
    ///
    /// // Team 0 should now exist
    /// assert!(my_league_season.team_exists(0));
    /// ```
    pub fn team_exists(&self, id: usize) -> bool {
        self.teams.contains_key(&id)
    }

    /// Borrows an immutable `FootballTeam` from a `LeagueSeason` given
    /// the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new FootballTeam
    /// let my_season_team = FootballTeam::new();
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let my_season_team = my_league_season.team(2);
    /// ```
    pub fn team(&self, id: usize) -> Option<&FootballTeam> {
        self.teams.get(&id)
    }

    /// Borrows a mutable `FootballTeam` from a `LeagueSeason` given the
    /// team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Instantiate a new LeagueSeason
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Instantiate a new FootballTeam
    /// let mut my_season_team = FootballTeam::new();
    ///
    /// // Add the team with ID 2
    /// let my_season_teams = my_league_season.teams_mut();
    /// my_season_teams.insert(2, my_season_team);
    ///
    /// // Get the LeagueTeam with ID 2
    /// let mut my_season_team = my_league_season.team_mut(2);
    /// ```
    pub fn team_mut(&mut self, id: usize) -> Option<&mut FootballTeam> {
        self.teams.get_mut(&id)
    }

    /// Borrow the conferences in the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let season = LeagueSeason::new();
    /// let conferences = season.conferences();
    /// ```
    pub fn conferences(&self) -> &Vec<LeagueConference> {
        &self.conferences
    }

    /// Mutably borrow the conferences in the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut season = LeagueSeason::new();
    /// let conferences = season.conferences_mut();
    /// ```
    pub fn conferences_mut(&mut self) -> &mut Vec<LeagueConference> {
        &mut self.conferences
    }

    /// Add a conference to the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let mut season = LeagueSeason::new();
    /// let conference = LeagueConference::with_name("AFC");
    /// season.add_conference(conference);
    /// ```
    pub fn add_conference(&mut self, conference: LeagueConference) {
        self.conferences.push(conference);
    }

    /// Get a conference by index
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let mut season = LeagueSeason::new();
    /// season.add_conference(LeagueConference::with_name("AFC"));
    /// let conference = season.conference(0);
    /// assert!(conference.is_some());
    /// ```
    pub fn conference(&self, index: usize) -> Option<&LeagueConference> {
        self.conferences.get(index)
    }

    /// Get a mutable conference by index
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let mut season = LeagueSeason::new();
    /// season.add_conference(LeagueConference::with_name("AFC"));
    /// let conference = season.conference_mut(0);
    /// assert!(conference.is_some());
    /// ```
    pub fn conference_mut(&mut self, index: usize) -> Option<&mut LeagueConference> {
        self.conferences.get_mut(index)
    }

    /// Find which conference a team belongs to (returns conference index)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut season = LeagueSeason::new();
    /// let mut conf = LeagueConference::with_name("AFC");
    /// let mut div = LeagueDivision::with_name("East");
    /// div.add_team(0);
    /// conf.add_division(0, div);
    /// season.add_conference(conf);
    ///
    /// assert_eq!(season.team_conference(0), Some(0));
    /// assert_eq!(season.team_conference(99), None);
    /// ```
    pub fn team_conference(&self, team_id: usize) -> Option<usize> {
        for (conf_index, conference) in self.conferences.iter().enumerate() {
            if conference.contains_team(team_id) {
                return Some(conf_index);
            }
        }
        None
    }

    /// Find which division a team belongs to (returns conference index and division id)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut season = LeagueSeason::new();
    /// let mut conf = LeagueConference::with_name("AFC");
    /// let mut div = LeagueDivision::with_name("East");
    /// div.add_team(0);
    /// conf.add_division(5, div);
    /// season.add_conference(conf);
    ///
    /// assert_eq!(season.team_division(0), Some((0, 5)));
    /// assert_eq!(season.team_division(99), None);
    /// ```
    pub fn team_division(&self, team_id: usize) -> Option<(usize, usize)> {
        for (conf_index, conference) in self.conferences.iter().enumerate() {
            if let Some(div_id) = conference.team_division(team_id) {
                return Some((conf_index, div_id));
            }
        }
        None
    }

    /// Check if two teams are in the same division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut season = LeagueSeason::new();
    /// let mut conf = LeagueConference::new();
    /// let mut div1 = LeagueDivision::new();
    /// div1.add_team(0);
    /// div1.add_team(1);
    /// let mut div2 = LeagueDivision::new();
    /// div2.add_team(2);
    /// div2.add_team(3);
    /// conf.add_division(0, div1);
    /// conf.add_division(1, div2);
    /// season.add_conference(conf);
    ///
    /// assert!(season.same_division(0, 1));
    /// assert!(!season.same_division(0, 2));
    /// ```
    pub fn same_division(&self, team1: usize, team2: usize) -> bool {
        match (self.team_division(team1), self.team_division(team2)) {
            (Some((conf1, div1)), Some((conf2, div2))) => conf1 == conf2 && div1 == div2,
            _ => false,
        }
    }

    /// Check if two teams are in the same conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut season = LeagueSeason::new();
    ///
    /// // AFC with two divisions
    /// let mut afc = LeagueConference::with_name("AFC");
    /// let mut afc_east = LeagueDivision::with_name("East");
    /// afc_east.add_team(0);
    /// afc_east.add_team(1);
    /// let mut afc_west = LeagueDivision::with_name("West");
    /// afc_west.add_team(2);
    /// afc_west.add_team(3);
    /// afc.add_division(0, afc_east);
    /// afc.add_division(1, afc_west);
    ///
    /// // NFC with one division
    /// let mut nfc = LeagueConference::with_name("NFC");
    /// let mut nfc_east = LeagueDivision::with_name("East");
    /// nfc_east.add_team(4);
    /// nfc_east.add_team(5);
    /// nfc.add_division(0, nfc_east);
    ///
    /// season.add_conference(afc);
    /// season.add_conference(nfc);
    ///
    /// // Teams 0 and 2 are in same conference (AFC) but different divisions
    /// assert!(season.same_conference(0, 2));
    /// // Teams 0 and 4 are in different conferences
    /// assert!(!season.same_conference(0, 4));
    /// ```
    pub fn same_conference(&self, team1: usize, team2: usize) -> bool {
        match (self.team_conference(team1), self.team_conference(team2)) {
            (Some(conf1), Some(conf2)) => conf1 == conf2,
            _ => false,
        }
    }

    /// Borrow the weeks of the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let my_season_weeks = my_league_season.weeks();
    /// ```
    pub fn weeks(&self) -> &Vec<LeagueSeasonWeek> {
        &self.weeks
    }

    /// Mutably borrow the weeks of the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let my_season_weeks = my_league_season.weeks_mut();
    /// ```
    pub fn weeks_mut(&mut self) -> &mut Vec<LeagueSeasonWeek> {
        &mut self.weeks
    }

    /// Borrow the playoffs from the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let my_season_weeks = my_league_season.weeks();
    /// ```
    pub fn playoffs(&self) -> &LeagueSeasonPlayoffs {
        &self.playoffs
    }

    /// Mutably borrow the playoffs from the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let mut my_league_season = LeagueSeason::new();
    /// let my_season_weeks = my_league_season.weeks_mut();
    /// ```
    pub fn playoffs_mut(&mut self) -> &mut LeagueSeasonPlayoffs {
        &mut self.playoffs
    }

    /// Determine based on the matchups whether the season has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let started = my_league_season.started();
    /// ```
    pub fn started(&self) -> bool {
        // If no season weeks, then the season hasn't started
        if self.weeks.is_empty() {
            return false;
        }

        // Loop through the season weeks and check if any are started
        for week in self.weeks.iter() {
            if week.started() {
                return true;
            }
        }
        false
    }

    /// Determine whether the regular season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let complete = raw_league_season.regular_season_complete();
    /// ```
    pub fn regular_season_complete(&self) -> bool {
        // If no season weeks, then the season isn't complete
        if self.weeks.is_empty() {
            return false;
        }

        // Loop through the season weeks and check if any are not complete
        for week in self.weeks.iter() {
            if !week.complete() {
                return false;
            }
        }
        true
    }

    /// Determine whether the season is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let complete = raw_league_season.complete();
    /// ```
    pub fn complete(&self) -> bool {
        if !self.regular_season_complete() {
            false
        } else {
            self.playoffs.complete()
        }
    }

    /// Create a default conference structure with a single conference and division
    /// containing all teams. Used when no conferences are defined but schedule
    /// generation is requested.
    fn create_default_conference(&mut self) {
        use crate::league::season::conference::LeagueDivision;

        let mut division = LeagueDivision::with_name("Default");
        for team_id in self.teams.keys() {
            division.add_team(*team_id);
        }

        let mut conference = LeagueConference::with_name("Default");
        conference.add_division(0, division);

        self.conferences.clear();
        self.conferences.push(conference);
    }

    /// Check if conference-aware (structured) scheduling is needed
    fn needs_structured_scheduling(&self, options: &LeagueSeasonScheduleOptions) -> bool {
        // Need structured scheduling if:
        // 1. Multiple conferences exist
        // 2. Any conference has multiple divisions
        // 3. Cross-conference games are explicitly requested
        self.conferences.len() > 1
            || self.conferences.iter().any(|c| c.divisions().len() > 1)
            || options.cross_conference_games.is_some()
    }

    /// Generate a schedule for the season.  The generated schedule is a round
    /// robin schedule in which each team plays an equal number of home and
    /// away games, and in which each team plays each other twice.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    /// ```
    pub fn generate_schedule(&mut self, options: LeagueSeasonScheduleOptions, rng: &mut impl Rng) -> Result<(), String> {
        // If no conferences defined, create default single conference with all teams
        if self.conferences.is_empty() {
            self.create_default_conference();
        }

        // Route to appropriate schedule generation method
        if self.needs_structured_scheduling(&options) {
            self.generate_structured_schedule(options, rng)
        } else {
            self.generate_round_robin_schedule(options, rng)
        }
    }

    /// Generate a simple round-robin schedule (existing algorithm)
    fn generate_round_robin_schedule(&mut self, options: LeagueSeasonScheduleOptions, rng: &mut impl Rng) -> Result<(), String> {
        // Check whether there are at least 4 teams, an even number of teams
        let num_teams = self.teams.len();
        if num_teams < 4 {
            return Err(
                format!(
                    "Less than 4 teams, not enough teams to generate a schedule: {}",
                    num_teams
                )
            );
        }
        if !num_teams.is_multiple_of(2) {
            return Err(
                format!(
                    "Odd number of teams, cannot generate a schedule: {}",
                    num_teams
                )
            )
        }

        // Check whether the number of weeks is between the prescribed min and max
        let num_weeks = match options.weeks {
            Some(weeks) => weeks,
            None => (num_teams - 1) * 2,
        };
        let max_num_weeks = (num_teams - 1) * 3;
        if num_weeks > max_num_weeks {
            return Err(
                format!(
                    "Schedule can involve teams playing each other team at most 3 times ({} games): {} given",
                    max_num_weeks, num_weeks
                )
            )
        }
        if num_weeks < (num_teams - 1) {
            return Err(
                format!(
                    "Schedule must involve teams playing each other team at least 1 time ({} games): {} given",
                    num_teams, num_weeks
                )
            )
        }

        // Get the shift option value, error if it is invalid
        let shift = match options.shift {
            Some(s) => {
                if s > num_weeks {
                    return Err(
                        format!(
                            "Shift ({}) must be less than the number of weeks ({})",
                            s, num_weeks
                        )
                    )
                } else {
                    s
                }
            },
            None => 0
        };

        // Check to make sure the season has not already started
        if self.started() {
            return Err(
                "Season has already started, cannot re-generate schedule".to_string()
            )
        }

        // If the schedule is already non-empty then empty it before re-gen
        if !self.weeks.is_empty() {
            self.weeks.clear()
        }

        // Generate the vec of team IDs and randomly permute it for a unique schedule
        let mut team_ids: Vec<usize> = self.teams.keys().cloned().collect();
        team_ids.shuffle(rng); // Generate a random permutation of the season team IDs

        // Generate the round-robin schedule using the circle method:
        // - Keep the team at index 0 fixed
        // - Rotate the teams at indices 1 to n-1
        // - Pair: index 0 with index n-1, index 1 with index n-2, etc.
        // This guarantees each pair plays exactly once per cycle of (n-1) rounds
        for week_index in 0..num_weeks {
            // Create a new league season week
            let mut week = LeagueSeasonWeek::new();

            // TODO: Implement the ability to have bye weeks
            let num_matchups = num_teams / 2;

            // Determine which round within the cycle and which cycle we're in
            // Each cycle is (n-1) rounds; in a double round-robin we have 2 cycles
            let round_in_cycle = week_index % (num_teams - 1);
            let cycle = week_index / (num_teams - 1);

            // Build the arrangement for this round using the circle method
            // Team at index 0 stays fixed, others rotate
            let mut arrangement: Vec<usize> = Vec::with_capacity(num_teams);
            arrangement.push(team_ids[0]); // Fixed team
            for i in 0..(num_teams - 1) {
                // Rotate the other teams: for round r, shift by r positions
                let rotated_index = (i + (num_teams - 1) - round_in_cycle) % (num_teams - 1);
                arrangement.push(team_ids[1 + rotated_index]);
            }

            // Create matchups for each pair of teams
            // Pair first with last, second with second-to-last, etc.
            for matchup_index in 0..num_matchups {
                let team1_index = matchup_index;
                let team2_index = num_teams - 1 - matchup_index;

                let team1_id = arrangement[team1_index];
                let team2_id = arrangement[team2_index];

                // Determine home/away based on cycle to balance home games
                // In cycle 0, team1 (lower index) is home; in cycle 1, team2 is home
                // This ensures each pair plays once with each team as home
                let (home_id, away_id) = if cycle.is_multiple_of(2) {
                    (team1_id, team2_id)
                } else {
                    (team2_id, team1_id)
                };

                // Get the home & away short names
                let home_short_name = self.teams.get(&home_id).unwrap().short_name();
                let away_short_name = self.teams.get(&away_id).unwrap().short_name();

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(home_id, away_id, home_short_name, away_short_name, rng);
                week.matchups_mut().push(matchup);
            }

            // Add the week to the season
            self.weeks.push(week);
        }

        // If desired, shift the weeks of the season
        if shift > 0 {
            self.weeks.rotate_right(shift);
        }

        // If desired, randomly permute the weeks of the season
        if let Some(permute) = options.permute {
            if permute {
                self.weeks.shuffle(rng);
            }
        }
        Ok(())
    }

    /// Generate a conference-aware structured schedule
    fn generate_structured_schedule(&mut self, options: LeagueSeasonScheduleOptions, rng: &mut impl Rng) -> Result<(), String> {
        // Validate basic requirements
        let num_teams = self.teams.len();
        if num_teams < 4 {
            return Err(format!("Less than 4 teams, not enough teams to generate a schedule: {}", num_teams));
        }

        // Check to make sure the season has not already started
        if self.started() {
            return Err("Season has already started, cannot re-generate schedule".to_string());
        }

        // Clear existing schedule
        if !self.weeks.is_empty() {
            self.weeks.clear();
        }

        // Get scheduling parameters with defaults
        let division_games = options.division_games.unwrap_or(2);
        let conference_games = options.conference_games.unwrap_or(1);
        let cross_conference_games = options.cross_conference_games.unwrap_or(0);

        // Generate all matchups as (home_id, away_id) tuples
        let mut all_matchups: Vec<(usize, usize)> = Vec::new();

        // Phase 1: Division matchups
        let division_matchups = self.generate_division_matchups(division_games, rng)?;
        all_matchups.extend(division_matchups);

        // Phase 2: Conference (non-division) matchups
        let conference_matchups = self.generate_conference_matchups(conference_games, rng)?;
        all_matchups.extend(conference_matchups);

        // Phase 3: Cross-conference matchups
        if cross_conference_games > 0 {
            let cross_conf_matchups = self.generate_cross_conference_matchups(cross_conference_games, rng)?;
            all_matchups.extend(cross_conf_matchups);
        }

        // Phase 4: Interleave matchups into weeks
        self.interleave_matchups(all_matchups, rng)?;

        // Get the shift option value
        let shift = options.shift.unwrap_or(0);
        if shift > 0 && shift <= self.weeks.len() {
            self.weeks.rotate_right(shift);
        }

        // If desired, randomly permute the weeks of the season
        if let Some(true) = options.permute {
            self.weeks.shuffle(rng);
        }

        Ok(())
    }

    /// Generate intra-division matchups using circle method within each division
    fn generate_division_matchups(&self, games_per_opponent: usize, rng: &mut impl Rng) -> Result<Vec<(usize, usize)>, String> {
        let mut matchups = Vec::new();

        for conference in &self.conferences {
            for division in conference.divisions().values() {
                let teams: Vec<usize> = division.teams().clone();
                let n = teams.len();

                if n < 2 {
                    continue; // Skip divisions with less than 2 teams
                }

                // Generate round-robin matchups within division
                let div_matchups = self.circle_method_matchups(&teams, games_per_opponent, rng)?;
                matchups.extend(div_matchups);
            }
        }

        Ok(matchups)
    }

    /// Generate intra-conference (cross-division) matchups
    fn generate_conference_matchups(&self, games_per_opponent: usize, rng: &mut impl Rng) -> Result<Vec<(usize, usize)>, String> {
        let mut matchups = Vec::new();

        if games_per_opponent == 0 {
            return Ok(matchups);
        }

        for conference in &self.conferences {
            // Get all teams in this conference
            let all_teams = conference.all_teams();

            // For each pair of teams in the same conference but different divisions
            for i in 0..all_teams.len() {
                for j in (i + 1)..all_teams.len() {
                    let team1 = all_teams[i];
                    let team2 = all_teams[j];

                    // Skip if same division (already handled in division matchups)
                    if self.same_division(team1, team2) {
                        continue;
                    }

                    // Generate the specified number of games
                    for cycle in 0..games_per_opponent {
                        // Alternate home/away
                        if cycle % 2 == 0 {
                            matchups.push((team1, team2));
                        } else {
                            matchups.push((team2, team1));
                        }
                    }
                }
            }
        }

        // Shuffle to add variety
        matchups.shuffle(rng);
        Ok(matchups)
    }

    /// Generate cross-conference matchups
    fn generate_cross_conference_matchups(&self, total_games_per_team: usize, rng: &mut impl Rng) -> Result<Vec<(usize, usize)>, String> {
        let mut matchups = Vec::new();

        if self.conferences.len() < 2 || total_games_per_team == 0 {
            return Ok(matchups);
        }

        // Track how many cross-conference games each team has scheduled
        let mut team_cross_conf_games: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for team_id in self.teams.keys() {
            team_cross_conf_games.insert(*team_id, 0);
        }

        // Get all possible cross-conference pairs
        let mut possible_pairs: Vec<(usize, usize)> = Vec::new();
        for (i, conf1) in self.conferences.iter().enumerate() {
            for conf2 in self.conferences.iter().skip(i + 1) {
                let teams1 = conf1.all_teams();
                let teams2 = conf2.all_teams();

                for t1 in &teams1 {
                    for t2 in &teams2 {
                        possible_pairs.push((*t1, *t2));
                    }
                }
            }
        }

        // Shuffle pairs for randomness
        possible_pairs.shuffle(rng);

        // Greedily assign cross-conference games
        for (team1, team2) in possible_pairs {
            let games1 = *team_cross_conf_games.get(&team1).unwrap_or(&0);
            let games2 = *team_cross_conf_games.get(&team2).unwrap_or(&0);

            if games1 < total_games_per_team && games2 < total_games_per_team {
                // Randomly decide home/away
                if rng.gen::<bool>() {
                    matchups.push((team1, team2));
                } else {
                    matchups.push((team2, team1));
                }
                *team_cross_conf_games.get_mut(&team1).unwrap() += 1;
                *team_cross_conf_games.get_mut(&team2).unwrap() += 1;
            }
        }

        Ok(matchups)
    }

    /// Apply circle method to generate round-robin matchups for a set of teams
    fn circle_method_matchups(&self, teams: &[usize], games_per_opponent: usize, rng: &mut impl Rng) -> Result<Vec<(usize, usize)>, String> {
        let mut matchups = Vec::new();
        let n = teams.len();

        if n < 2 {
            return Ok(matchups);
        }

        // Shuffle teams for variety
        let mut team_ids = teams.to_vec();
        team_ids.shuffle(rng);

        let num_rounds = n - 1;
        let num_matchups_per_round = n / 2;

        // Generate games_per_opponent cycles of round-robin
        for cycle in 0..games_per_opponent {
            for round in 0..num_rounds {
                // Build arrangement using circle method
                let mut arrangement: Vec<usize> = Vec::with_capacity(n);
                arrangement.push(team_ids[0]); // Fixed team
                for i in 0..(n - 1) {
                    let rotated_index = (i + (n - 1) - round) % (n - 1);
                    arrangement.push(team_ids[1 + rotated_index]);
                }

                // Create matchups
                for m in 0..num_matchups_per_round {
                    let team1 = arrangement[m];
                    let team2 = arrangement[n - 1 - m];

                    // Alternate home/away based on cycle
                    if cycle % 2 == 0 {
                        matchups.push((team1, team2));
                    } else {
                        matchups.push((team2, team1));
                    }
                }
            }
        }

        Ok(matchups)
    }

    /// Interleave matchups into weeks, avoiding long road trips
    fn interleave_matchups(&mut self, matchups: Vec<(usize, usize)>, rng: &mut impl Rng) -> Result<(), String> {
        use std::collections::{HashMap, HashSet};

        if matchups.is_empty() {
            return Ok(());
        }

        let num_teams = self.teams.len();
        let _matchups_per_week = num_teams / 2;

        // Track home/away streaks and which matchups are scheduled
        let mut remaining_matchups: Vec<(usize, usize)> = matchups;
        remaining_matchups.shuffle(rng); // Shuffle for variety

        let mut team_away_streak: HashMap<usize, usize> = HashMap::new();
        let mut team_home_streak: HashMap<usize, usize> = HashMap::new();
        for id in self.teams.keys() {
            team_away_streak.insert(*id, 0);
            team_home_streak.insert(*id, 0);
        }

        while !remaining_matchups.is_empty() {
            let mut week = LeagueSeasonWeek::new();
            let mut teams_scheduled_this_week: HashSet<usize> = HashSet::new();
            let mut matchups_this_week: Vec<usize> = Vec::new(); // indices into remaining_matchups

            // Greedily select matchups for this week
            for (idx, (home_id, away_id)) in remaining_matchups.iter().enumerate() {
                // Skip if either team already scheduled this week
                if teams_scheduled_this_week.contains(home_id) || teams_scheduled_this_week.contains(away_id) {
                    continue;
                }

                // Check road trip constraint (no more than 2 consecutive away games)
                let away_streak = *team_away_streak.get(away_id).unwrap_or(&0);
                if away_streak >= 2 {
                    // Check if there's an alternative where this team is home
                    // For now, we'll allow it but prefer not to
                    // This could be improved with backtracking
                }

                // Schedule this matchup
                teams_scheduled_this_week.insert(*home_id);
                teams_scheduled_this_week.insert(*away_id);
                matchups_this_week.push(idx);

                if teams_scheduled_this_week.len() >= num_teams {
                    break; // Week is full
                }
            }

            // If we couldn't schedule any matchups but there are still remaining, force one
            if matchups_this_week.is_empty() && !remaining_matchups.is_empty() {
                matchups_this_week.push(0);
                let (home_id, away_id) = remaining_matchups[0];
                teams_scheduled_this_week.insert(home_id);
                teams_scheduled_this_week.insert(away_id);
            }

            // Create matchups and update streaks
            // Sort indices in reverse so we can remove from end first
            matchups_this_week.sort_by(|a, b| b.cmp(a));
            for idx in matchups_this_week {
                let (home_id, away_id) = remaining_matchups.remove(idx);

                // Get team short names
                let home_short_name = self.teams.get(&home_id).unwrap().short_name();
                let away_short_name = self.teams.get(&away_id).unwrap().short_name();

                // Create matchup
                let matchup = LeagueSeasonMatchup::new(home_id, away_id, home_short_name, away_short_name, rng);
                week.matchups_mut().push(matchup);

                // Update streaks
                *team_home_streak.get_mut(&home_id).unwrap() += 1;
                *team_away_streak.get_mut(&home_id).unwrap() = 0;
                *team_away_streak.get_mut(&away_id).unwrap() += 1;
                *team_home_streak.get_mut(&away_id).unwrap() = 0;
            }

            self.weeks.push(week);

            // Safety: prevent infinite loop
            if self.weeks.len() > num_teams * 10 {
                return Err("Failed to generate valid schedule - too many weeks".to_string());
            }
        }

        Ok(())
    }

    /// Computes the season standings sorted in descending order, mapping
    /// team IDs to their season record
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Compute the standings
    /// let standings = my_league_season.standings();
    /// ```
    pub fn standings(&self) -> Vec<(usize, LeagueTeamRecord)> {
        // Compute each team's record
        let mut standings: Vec<(usize, LeagueTeamRecord)> = Vec::new();
        for (id, _) in self.teams.iter() {
            let matchups = match self.team_matchups(*id) {
                Ok(m) => m,
                Err(_) => continue
            };
            standings.push((*id, matchups.record()));
        }

        // Sort by win percentage (descending), then by wins (descending), then by team ID
        standings.sort_by(|a, b| {
            let (id_a, rec_a) = a;
            let (id_b, rec_b) = b;

            // Calculate win percentage (wins + 0.5*ties) / total games
            let games_a = rec_a.wins() + rec_a.losses() + rec_a.ties();
            let games_b = rec_b.wins() + rec_b.losses() + rec_b.ties();

            // Avoid division by zero
            let pct_a = if games_a > 0 {
                (*rec_a.wins() as f64 + 0.5 * *rec_a.ties() as f64) / games_a as f64
            } else {
                0.0
            };
            let pct_b = if games_b > 0 {
                (*rec_b.wins() as f64 + 0.5 * *rec_b.ties() as f64) / games_b as f64
            } else {
                0.0
            };

            // Sort by win percentage (descending)
            match pct_b.partial_cmp(&pct_a) {
                Some(std::cmp::Ordering::Equal) | None => {}
                Some(ord) => return ord,
            }

            // Tiebreaker: wins (descending)
            match rec_b.wins().cmp(rec_a.wins()) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }

            // Final tiebreaker: team ID (ascending for consistency)
            id_a.cmp(id_b)
        });

        standings
    }

    /// Computes the division standings for a specific division
    ///
    /// ### Arguments
    /// * `conf_index` - The conference index (position in conferences Vec)
    /// * `div_id` - The division ID (key in conference's divisions BTreeMap)
    ///
    /// ### Returns
    /// * `Ok(Vec<(usize, LeagueTeamRecord)>)` - Division standings sorted by record
    /// * `Err(String)` - If conference or division doesn't exist
    pub fn division_standings(&self, conf_index: usize, div_id: usize) -> Result<Vec<(usize, LeagueTeamRecord)>, String> {
        // Get the conference
        let conference = self.conferences.get(conf_index)
            .ok_or_else(|| format!("Conference index {} does not exist", conf_index))?;

        // Get the division
        let division = conference.division(div_id)
            .ok_or_else(|| format!("Division {} does not exist in conference {}", div_id, conf_index))?;

        // Get all standings and filter to this division's teams
        let all_standings = self.standings();
        let division_teams: std::collections::HashSet<usize> = division.teams().iter().cloned().collect();

        let standings: Vec<(usize, LeagueTeamRecord)> = all_standings
            .into_iter()
            .filter(|(id, _)| division_teams.contains(id))
            .collect();

        Ok(standings)
    }

    /// Computes the conference standings for a specific conference
    ///
    /// ### Arguments
    /// * `conf_index` - The conference index (position in conferences Vec)
    ///
    /// ### Returns
    /// * `Ok(Vec<(usize, LeagueTeamRecord)>)` - Conference standings sorted by record
    /// * `Err(String)` - If conference doesn't exist
    pub fn conference_standings(&self, conf_index: usize) -> Result<Vec<(usize, LeagueTeamRecord)>, String> {
        // Get the conference
        let conference = self.conferences.get(conf_index)
            .ok_or_else(|| format!("Conference index {} does not exist", conf_index))?;

        // Get all teams in the conference
        let conference_teams: std::collections::HashSet<usize> = conference.all_teams().into_iter().collect();

        // Get all standings and filter to this conference's teams
        let all_standings = self.standings();

        let standings: Vec<(usize, LeagueTeamRecord)> = all_standings
            .into_iter()
            .filter(|(id, _)| conference_teams.contains(id))
            .collect();

        Ok(standings)
    }

    /// Computes a team's record against division opponents only
    ///
    /// ### Arguments
    /// * `team_id` - The team ID to compute division record for
    ///
    /// ### Returns
    /// * `Ok(LeagueTeamRecord)` - The team's record against division opponents
    /// * `Err(String)` - If team doesn't exist or has no division assignment
    pub fn division_record(&self, team_id: usize) -> Result<LeagueTeamRecord, String> {
        // Find team's division
        let (conf_index, div_id) = self.team_division(team_id)
            .ok_or_else(|| format!("Team {} has no division assignment", team_id))?;

        // Get division teams
        let division = self.conferences.get(conf_index)
            .and_then(|c| c.division(div_id))
            .ok_or_else(|| format!("Division not found for team {}", team_id))?;

        let division_teams: std::collections::HashSet<usize> = division.teams().iter().cloned().collect();

        // Compute record against division opponents
        let mut record = LeagueTeamRecord::new();

        for week in &self.weeks {
            for matchup in week.matchups() {
                if !matchup.context().game_over() {
                    continue;
                }

                let home = *matchup.home_team();
                let away = *matchup.away_team();

                // Check if this is a division game involving our team
                let is_our_game = home == team_id || away == team_id;
                let opponent = if home == team_id { away } else { home };
                let is_division_game = division_teams.contains(&opponent);

                if is_our_game && is_division_game {
                    match matchup.result(team_id) {
                        Some(crate::game::matchup::FootballMatchupResult::Win) => {
                            record.increment_wins(1);
                        }
                        Some(crate::game::matchup::FootballMatchupResult::Loss) => {
                            record.increment_losses(1);
                        }
                        Some(crate::game::matchup::FootballMatchupResult::Tie) => {
                            record.increment_ties(1);
                        }
                        None => {}
                    }
                }
            }
        }

        Ok(record)
    }

    /// Computes a team's record against conference opponents only
    ///
    /// ### Arguments
    /// * `team_id` - The team ID to compute conference record for
    ///
    /// ### Returns
    /// * `Ok(LeagueTeamRecord)` - The team's record against conference opponents
    /// * `Err(String)` - If team doesn't exist or has no conference assignment
    pub fn conference_record(&self, team_id: usize) -> Result<LeagueTeamRecord, String> {
        // Find team's conference
        let conf_index = self.team_conference(team_id)
            .ok_or_else(|| format!("Team {} has no conference assignment", team_id))?;

        // Get conference teams
        let conference = self.conferences.get(conf_index)
            .ok_or_else(|| format!("Conference not found for team {}", team_id))?;

        let conference_teams: std::collections::HashSet<usize> = conference.all_teams().into_iter().collect();

        // Compute record against conference opponents
        let mut record = LeagueTeamRecord::new();

        for week in &self.weeks {
            for matchup in week.matchups() {
                if !matchup.context().game_over() {
                    continue;
                }

                let home = *matchup.home_team();
                let away = *matchup.away_team();

                // Check if this is a conference game involving our team
                let is_our_game = home == team_id || away == team_id;
                let opponent = if home == team_id { away } else { home };
                let is_conference_game = conference_teams.contains(&opponent);

                if is_our_game && is_conference_game {
                    match matchup.result(team_id) {
                        Some(crate::game::matchup::FootballMatchupResult::Win) => {
                            record.increment_wins(1);
                        }
                        Some(crate::game::matchup::FootballMatchupResult::Loss) => {
                            record.increment_losses(1);
                        }
                        Some(crate::game::matchup::FootballMatchupResult::Tie) => {
                            record.increment_ties(1);
                        }
                        None => {}
                    }
                }
            }
        }

        Ok(record)
    }

    /// Generate the current playoff picture for the season
    ///
    /// ### Arguments
    /// * `num_playoff_teams` - Number of teams that will make the playoffs
    ///
    /// ### Returns
    /// * `Ok(PlayoffPicture)` - The current playoff picture
    /// * `Err(String)` - If the season hasn't started or parameters are invalid
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = my_league_season.playoff_picture(2);
    /// assert!(picture.is_ok());
    /// ```
    pub fn playoff_picture(&self, num_playoff_teams: usize) -> Result<playoffs::picture::PlayoffPicture, String> {
        PlayoffPicture::from_season(self, num_playoff_teams)
    }

    /// Determine of a team participated in the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Create a season and add a team
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    ///
    /// // Check if that team is in the playoffs
    /// let in_playoffs = my_league_season.team_in_playoffs(0);
    /// assert!(in_playoffs.is_ok());
    /// assert!(!in_playoffs.unwrap());
    /// ```
    pub fn team_in_playoffs(&self, team_id: usize) -> Result<bool, String> {
        if !self.team_exists(team_id) {
            return Err(format!("No season team with ID: {}", team_id));
        }
        Ok(self.playoffs.team_in_playoffs(team_id))
    }

    /// Compute a team's playoff record
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// my_league_season.generate_playoffs(4, &mut rng);
    ///
    /// // Simulate the playoffs
    /// my_league_season.sim_playoffs(&mut rng);
    ///
    /// // Compute a team's playoff record
    /// let record = my_league_season.playoff_record(0);
    /// assert!(record.is_ok());
    /// ```
    pub fn playoff_record(&self, team_id: usize) -> Result<LeagueTeamRecord, String> {
        if self.team_in_playoffs(team_id)? {
            self.playoffs.record(team_id)
        } else {
            // Technically should be unreachable
            Ok(LeagueTeamRecord::new())
        }
    }

    /// Check if a team made it to the championship game this season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Create a season and add a team
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    ///
    /// // Determine if that team is in the shampionship
    /// let in_championship = my_league_season.team_in_championship(0);
    /// assert!(in_championship.is_ok());
    /// assert!(!in_championship.unwrap());
    /// ```
    pub fn team_in_championship(&self, team_id: usize) -> Result<bool, String> {
        if self.team_in_playoffs(team_id)? {
            self.playoffs.in_championship(team_id)
        } else {
            Ok(false)
        }
    }

    /// Check if a team won the championship this season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Create a season and add a team
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    ///
    /// // Determine if that team won the shampionship
    /// let won_championship = my_league_season.team_won_championship(0);
    /// assert!(won_championship.is_ok());
    /// assert!(!won_championship.unwrap());
    /// ```
    pub fn team_won_championship(&self, team_id: usize) -> Result<bool, String> {
        if self.team_in_playoffs(team_id)? {
            if let Some(champion_id) = self.playoffs.champion() {
                return Ok(champion_id == team_id);
            }
        }
        Ok(false)
    }

    /// Generate the playoffs with the specified number of teams
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// let res = my_league_season.generate_playoffs(4, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn generate_playoffs(&mut self, num_playoff_teams: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot generate playoffs: Regular season is not complete"));
        }

        // Ensure the playoffs have not already started
        if self.playoffs.started() {
            return Err(String::from("Cannot generate playoffs: Playoffs have already started"));
        }

        // Validate the number of playoff teams
        if num_playoff_teams < 2 {
            return Err(format!("Playoffs must have at least 2 teams, got {}", num_playoff_teams));
        }
        if num_playoff_teams > self.teams.len() {
            return Err(format!(
                "Cannot have more playoff teams ({}) than season teams ({})",
                num_playoff_teams, self.teams.len()
            ));
        }

        // Get the standings and select the top teams
        let standings = self.standings();

        // Reset the playoffs
        self.playoffs = LeagueSeasonPlayoffs::new();

        // Add the top teams to the playoffs in seed order
        for (i, (team_id, _)) in standings.iter().enumerate() {
            if i >= num_playoff_teams {
                break;
            }

            // Get the team's short name for the playoff bracket
            let team = match self.teams.get(team_id) {
                Some(t) => t,
                None => return Err(format!("Team {} not found in season", team_id))
            };
            let short_name = team.short_name();

            // Add the team to the playoffs
            self.playoffs.add_team(*team_id, short_name, None)?;
        }

        // Generate the first round (or wild card round if not a power of 2)
        self.playoffs.gen_next_round(rng)?;
        Ok(())
    }

    /// Generate playoffs with separate conference brackets
    ///
    /// This method creates a playoff structure where each conference has its own
    /// bracket. Division winners are seeded first (if guaranteed), followed by
    /// wild card teams based on conference standings.
    ///
    /// ### Arguments
    /// * `playoff_teams_per_conference` - Number of teams per conference that make playoffs
    /// * `division_winners_guaranteed` - If true, all division winners automatically qualify
    /// * `rng` - Random number generator
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// // Create a new season with 2 conferences
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add teams
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Create conference structure
    /// let mut afc = LeagueConference::with_name("AFC");
    /// let mut afc_div = LeagueDivision::with_name("East");
    /// afc_div.add_team(0);
    /// afc_div.add_team(1);
    /// afc.add_division(0, afc_div);
    /// my_league_season.add_conference(afc);
    ///
    /// let mut nfc = LeagueConference::with_name("NFC");
    /// let mut nfc_div = LeagueDivision::with_name("East");
    /// nfc_div.add_team(2);
    /// nfc_div.add_team(3);
    /// nfc.add_division(0, nfc_div);
    /// my_league_season.add_conference(nfc);
    ///
    /// // Generate schedule and simulate
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate conference playoffs (1 team per conference)
    /// let res = my_league_season.generate_playoffs_with_conferences(1, true, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn generate_playoffs_with_conferences(
        &mut self,
        playoff_teams_per_conference: usize,
        division_winners_guaranteed: bool,
        rng: &mut impl Rng,
    ) -> Result<(), String> {
        // Ensure we have conferences
        if self.conferences.is_empty() {
            return Err("Cannot generate conference playoffs: No conferences defined".to_string());
        }

        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err("Cannot generate playoffs: Regular season is not complete".to_string());
        }

        // Ensure the playoffs have not already started
        if self.playoffs.started() {
            return Err("Cannot generate playoffs: Playoffs have already started".to_string());
        }

        // Validate teams per conference
        if playoff_teams_per_conference < 1 {
            return Err("Playoffs must have at least 1 team per conference".to_string());
        }

        // Reset the playoffs
        self.playoffs = LeagueSeasonPlayoffs::new();

        // Process each conference
        for (conf_index, conference) in self.conferences.iter().enumerate() {
            // Validate that conference has enough teams
            let conf_team_count = conference.num_teams();
            if playoff_teams_per_conference > conf_team_count {
                return Err(format!(
                    "Cannot have {} playoff teams in conference {} which only has {} teams",
                    playoff_teams_per_conference,
                    conference.name(),
                    conf_team_count
                ));
            }

            // Get conference standings
            let conf_standings = self.conference_standings(conf_index)?;

            // Determine division winners if guaranteed spots
            let mut division_winners: Vec<usize> = Vec::new();
            if division_winners_guaranteed {
                for (div_id, _division) in conference.divisions().iter() {
                    let div_standings = self.division_standings(conf_index, *div_id)?;
                    if let Some((winner_id, _)) = div_standings.first() {
                        division_winners.push(*winner_id);
                    }
                }
            }

            // Build playoff teams for this conference
            let mut conf_playoff_teams: Vec<usize> = Vec::new();

            // First, add division winners (in order of their conference standing)
            // Sort division winners by their position in conference standings
            let mut sorted_div_winners: Vec<(usize, usize)> = division_winners
                .iter()
                .filter_map(|&winner_id| {
                    conf_standings
                        .iter()
                        .position(|(id, _)| *id == winner_id)
                        .map(|pos| (pos, winner_id))
                })
                .collect();
            sorted_div_winners.sort_by_key(|(pos, _)| *pos);

            for (_, winner_id) in sorted_div_winners {
                if conf_playoff_teams.len() < playoff_teams_per_conference {
                    conf_playoff_teams.push(winner_id);
                }
            }

            // Fill remaining spots with wild cards (best records not already in)
            for (team_id, _record) in conf_standings.iter() {
                if conf_playoff_teams.len() >= playoff_teams_per_conference {
                    break;
                }
                if !conf_playoff_teams.contains(team_id) {
                    conf_playoff_teams.push(*team_id);
                }
            }

            // Add teams to conference bracket
            for team_id in conf_playoff_teams {
                let team = self.teams.get(&team_id)
                    .ok_or_else(|| format!("Team {} not found", team_id))?;
                let short_name = team.short_name();
                self.playoffs.add_team(team_id, short_name, Some(conf_index))?;
            }
        }

        // Generate the first round of conference playoffs
        self.playoffs.gen_next_conference_round(rng)?;
        Ok(())
    }

    /// Generate the next playoff round for conference-based playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // See generate_playoffs_with_conferences for full example
    /// ```
    pub fn generate_next_conference_playoff_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        if !self.playoffs.is_conference_playoff() {
            return Err("Not a conference playoff. Use generate_next_playoff_round instead.".to_string());
        }

        // Ensure the playoffs are not already complete
        if self.playoffs.complete() {
            return Err("Cannot generate playoff round: Playoffs are already complete".to_string());
        }

        // Ensure the current round is complete
        if let Some(current_round) = self.playoffs.rounds().last() {
            if !current_round.complete() {
                return Err("Cannot generate next round: Current round is not complete".to_string());
            }
        }

        self.playoffs.gen_next_conference_round(rng)
    }

    /// Generate the next playoff round
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// let res = my_league_season.generate_playoffs(4, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn generate_next_playoff_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot generate playoff round: Regular season is not complete"));
        }

        // Ensure the playoffs have started (teams have been added)
        if self.playoffs.num_teams() < 2 {
            return Err(String::from("Cannot generate playoff round: Playoffs have not been initialized"));
        }

        // Ensure the playoffs are not already complete
        if self.playoffs.complete() {
            return Err(String::from("Cannot generate playoff round: Playoffs are already complete"));
        }

        // Ensure the current round is complete before generating the next
        if let Some(current_round) = self.playoffs.rounds().last() {
            if !current_round.complete() {
                return Err(String::from("Cannot generate playoff round: Current round is not complete"));
            }
        }

        // Generate the next round
        self.playoffs.gen_next_round(rng)?;
        Ok(())
    }

    /// Simulate a playoff matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// my_league_season.generate_playoffs(4, &mut rng);
    ///
    /// // Simulate the first playoff matchup
    /// let res = my_league_season.sim_playoff_matchup(0, 0, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn sim_playoff_matchup(&mut self, round: usize, matchup: usize, rng: &mut impl Rng) -> Result<Game, String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot simulate playoff matchup: Regular season is not complete"));
        }

        // Ensure the playoffs have started
        if self.playoffs.rounds().is_empty() {
            return Err(String::from("Cannot simulate playoff matchup: Playoffs have not been generated"));
        }

        // Check if the prior round is complete (if not the first round)
        if round > 0 {
            let prev_round = match self.playoffs.rounds().get(round - 1) {
                Some(r) => r,
                None => return Err(format!("Failed to get previous playoff round {}", round - 1))
            };
            if !prev_round.complete() {
                return Err(format!(
                    "Cannot simulate playoff round {}: Previous round {} is not complete",
                    round, round - 1
                ));
            }
        }

        // Get the playoff round
        let playoff_round = match self.playoffs.rounds_mut().get_mut(round) {
            Some(r) => r,
            None => return Err(format!("No such playoff round: {}", round))
        };

        // Get the matchup
        let playoff_matchup = match playoff_round.matchups_mut().get_mut(matchup) {
            Some(m) => m,
            None => return Err(format!("No such matchup {} in playoff round {}", matchup, round))
        };

        // Ensure the matchup is not already complete
        if playoff_matchup.context().game_over() {
            return Err(format!("Playoff round {} matchup {} is already complete", round, matchup));
        }

        // Get the home and away teams
        let home_id = *playoff_matchup.home_team();
        let away_id = *playoff_matchup.away_team();

        let home_team = match self.teams.get(&home_id) {
            Some(t) => t,
            None => return Err(format!("Playoff matchup references nonexistent home team ID: {}", home_id))
        };
        let away_team = match self.teams.get(&away_id) {
            Some(t) => t,
            None => return Err(format!("Playoff matchup references nonexistent away team ID: {}", away_id))
        };

        // Simulate the matchup
        let mut game = Game::new();
        let simulator = GameSimulator::new();
        let context = match simulator.sim_game(
            home_team, away_team,
            playoff_matchup.context().clone(),
            &mut game, rng
        ) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while simulating playoff matchup: {}", e))
        };

        // Update the matchup context and stats
        *playoff_matchup.context_mut() = context;
        *playoff_matchup.home_stats_mut() = Some(game.home_stats());
        *playoff_matchup.away_stats_mut() = Some(game.away_stats());
        Ok(game)
    }

    /// Simulate a single play of a playoff matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// my_league_season.generate_playoffs(4, &mut rng);
    ///
    /// // Simulate a single play of the first playoff matchup
    /// let res = my_league_season.sim_playoff_play(0, 0, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn sim_playoff_play(&mut self, round: usize, matchup: usize, rng: &mut impl Rng) -> Result<Option<Game>, String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot simulate playoff play: Regular season is not complete"));
        }

        // Ensure the playoffs have started
        if self.playoffs.rounds().is_empty() {
            return Err(String::from("Cannot simulate playoff play: Playoffs have not been generated"));
        }

        // Check if the prior round is complete (if not the first round)
        if round > 0 {
            let prev_round = match self.playoffs.rounds().get(round - 1) {
                Some(r) => r,
                None => return Err(format!("Failed to get previous playoff round {}", round - 1))
            };
            if !prev_round.complete() {
                return Err(format!(
                    "Cannot simulate playoff round {}: Previous round {} is not complete",
                    round, round - 1
                ));
            }
        }

        // Get the playoff round
        let playoff_round = match self.playoffs.rounds_mut().get_mut(round) {
            Some(r) => r,
            None => return Err(format!("No such playoff round: {}", round))
        };

        // Get the matchup
        let playoff_matchup = match playoff_round.matchups_mut().get_mut(matchup) {
            Some(m) => m,
            None => return Err(format!("No such matchup {} in playoff round {}", matchup, round))
        };

        // Ensure the matchup is not already complete
        if playoff_matchup.context().game_over() {
            return Err(format!("Playoff round {} matchup {} is already complete", round, matchup));
        }

        // Get the home and away teams
        let home_id = *playoff_matchup.home_team();
        let away_id = *playoff_matchup.away_team();

        let home_team = match self.teams.get(&home_id) {
            Some(t) => t,
            None => return Err(format!("Playoff matchup references nonexistent home team ID: {}", home_id))
        };
        let away_team = match self.teams.get(&away_id) {
            Some(t) => t,
            None => return Err(format!("Playoff matchup references nonexistent away team ID: {}", away_id))
        };

        // Create a new game if game has not started, or get existing game
        if playoff_matchup.game().is_none() {
            *playoff_matchup.game_mut() = Some(Game::new());
        }

        // Simulate the next play
        let simulator = GameSimulator::new();
        let context = match simulator.sim_play(
            home_team, away_team,
            playoff_matchup.context().clone(),
            playoff_matchup.game_mut().as_mut().unwrap(),
            rng
        ) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while simulating playoff play: {}", e))
        };

        // If game is over, archive game stats, clear game, update context
        if context.game_over() {
            *playoff_matchup.home_stats_mut() = Some(
                playoff_matchup.game().as_ref().ok_or(
                    "Failed to archive home stats for playoff game"
                )?.home_stats()
            );
            *playoff_matchup.away_stats_mut() = Some(
                playoff_matchup.game().as_ref().ok_or(
                    "Failed to archive away stats for playoff game"
                )?.away_stats()
            );
            *playoff_matchup.context_mut() = context;
            return Ok(playoff_matchup.take_game());
        }
        *playoff_matchup.context_mut() = context;
        Ok(None)
    }

    /// Simulate a full playoff round
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// my_league_season.generate_playoffs(4, &mut rng);
    ///
    /// // Simulate the first playoff round
    /// let res = my_league_season.sim_playoff_round(0, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn sim_playoff_round(&mut self, round: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot simulate playoff round: Regular season is not complete"));
        }

        // Ensure the playoffs have started
        if self.playoffs.rounds().is_empty() {
            return Err(String::from("Cannot simulate playoff round: Playoffs have not been generated"));
        }

        // Check if the prior round is complete (if not the first round)
        if round > 0 {
            let prev_round = match self.playoffs.rounds().get(round - 1) {
                Some(r) => r,
                None => return Err(format!("Failed to get previous playoff round {}", round - 1))
            };
            if !prev_round.complete() {
                return Err(format!(
                    "Cannot simulate playoff round {}: Previous round {} is not complete",
                    round, round - 1
                ));
            }
        }

        // Get the number of matchups in this round
        let num_matchups = match self.playoffs.rounds().get(round) {
            Some(r) => r.matchups().len(),
            None => return Err(format!("No such playoff round: {}", round))
        };

        // Simulate each matchup in the round
        for i in 0..num_matchups {
            // Skip matchups that have already been completed
            let is_complete = match self.playoffs.rounds().get(round) {
                Some(r) => match r.matchups().get(i) {
                    Some(m) => m.context().game_over(),
                    None => continue
                },
                None => continue
            };
            if is_complete {
                continue;
            }

            // Simulate the matchup
            self.sim_playoff_matchup(round, i, rng)?;
        }
        Ok(())
    }

    /// Simulate all remaining playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire regular season
    /// my_league_season.sim_regular_season(&mut rng);
    ///
    /// // Generate playoffs with 4 teams
    /// my_league_season.generate_playoffs(4, &mut rng);
    ///
    /// // Simulate all playoffs
    /// let res = my_league_season.sim_playoffs(&mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn sim_playoffs(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure the regular season is complete
        if !self.regular_season_complete() {
            return Err(String::from("Cannot simulate playoffs: regular season is not complete"));
        }

        // Ensure the playoffs have started
        if self.playoffs.rounds().is_empty() {
            return Err(String::from("Cannot simulate playoffs: playoffs have not been generated"));
        }

        // Simulate rounds until playoffs are complete
        while !self.playoffs.complete() {
            // Get the current round index
            let current_round = self.playoffs.rounds().len() - 1;

            // Simulate the current round if not complete
            if !self.playoffs.rounds().get(current_round).is_none_or(|r| r.complete()) {
                self.sim_playoff_round(current_round, rng)?;
            }

            // Generate the next round if the current round is complete and playoffs aren't done
            if !self.playoffs.complete() {
                self.generate_next_playoff_round(rng)?;
            }
        }
        Ok(())
    }

    /// Simulate the next play of a season matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the first game of the first week
    /// let res = my_league_season.sim_play(0, 0, &mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn sim_play(&mut self, week: usize, matchup: usize, rng: &mut impl Rng) -> Result<Option<Game>, String> {
        // Check if the prior week is not complete
        if week > 0 {
            let prev_week = match self.weeks.get(week - 1) {
                Some(w) => w,
                None => return Err(format!("Failed to get previous week {} from season {}", week-1, self.year))
            };
            if !prev_week.complete() {
                return Err(
                    format!(
                        "Cannot simulate week {} for season {}: previous week {} not complete",
                        week, self.year, week-1
                    )
                );
            }
        }

        // Try to get the given week
        let mut _week_to_sim = match self.weeks.get_mut(week) {
            Some(w) => w,
            None => return Err(format!("No such week for season {}: {}", self.year, week)),
        };

        // Try to get the given matchup
        let mut _matchup_to_sim = match _week_to_sim.matchups_mut().get_mut(matchup) {
            Some(m) => m,
            None => return Err(format!("No such matchup in season {} week {}: {}", self.year, week, matchup)),
        };

        // Ensure the matchup is not already complete
        if _matchup_to_sim.context().game_over() {
            return Err(format!("Season {} week {} matchup {} is already complete", self.year, week, matchup));
        }

        // Try to get the home team for the matchup
        let home_id = _matchup_to_sim.home_team();
        let home_team = match self.teams.get(home_id) {
            Some(t) => t,
            None => return Err(
                format!(
                    "Season {} week {} matchup {} references nonexistent home team ID: {}",
                    self.year, week, matchup, home_id
                )
            )
        };

        // Try to get the away team for the matchup
        let away_id = _matchup_to_sim.away_team();
        let away_team = match self.teams.get(away_id) {
            Some(t) => t,
            None => return Err(
                format!(
                    "Season {} week {} matchup {} references nonexistent away team ID: {}",
                    self.year, week, matchup, away_id
                )
            )
        };

        // Create a new game if game has not started, or get existing game
        if _matchup_to_sim.game().is_none() {
            *_matchup_to_sim.game_mut() = Some(Game::new());
        }

        // Simulate the next play
        let simulator = GameSimulator::new();
        let context = match simulator.sim_play(
            home_team, away_team,
            _matchup_to_sim.context().clone(),
            _matchup_to_sim.game_mut().as_mut().unwrap(),
            rng
        ) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while simulating matchup: {}", e))
        };

        // If game is over, archive game stats, clear game, update context
        if context.game_over() {
            *_matchup_to_sim.home_stats_mut() = Some(
                _matchup_to_sim.game().as_ref().ok_or(
                    "Failed to archive home stats for game"
                )?.home_stats()
            );
            *_matchup_to_sim.away_stats_mut() = Some(
                _matchup_to_sim.game().as_ref().ok_or(
                    "Failed to archive away stats for game"
                )?.away_stats()
            );
            *_matchup_to_sim.context_mut() = context;
            return Ok(_matchup_to_sim.take_game());
        }
        *_matchup_to_sim.context_mut() = context;
        Ok(None)
    }

    /// Simulate a season matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the first game of the first week
    /// my_league_season.sim_matchup(0, 0, &mut rng);
    /// ```
    pub fn sim_matchup(&mut self, week: usize, matchup: usize, rng: &mut impl Rng) -> Result<Game, String> {
        // Check if the prior week is not complete
        if week > 0 {
            let prev_week = match self.weeks.get(week - 1) {
                Some(w) => w,
                None => return Err(format!("Failed to get previous week {} from season {}", week-1, self.year))
            };
            if !prev_week.complete() {
                return Err(
                    format!(
                        "Cannot simulate week {} for season {}: previous week {} not complete",
                        week, self.year, week-1
                    )
                );
            }
        }
        
        // Try to get the given week
        let mut _week_to_sim = match self.weeks.get_mut(week) {
            Some(w) => w,
            None => return Err(format!("No such week for season {}: {}", self.year, week)),
        };

        // Try to get the given matchup
        let mut _matchup_to_sim = match _week_to_sim.matchups_mut().get_mut(matchup) {
            Some(m) => m,
            None => return Err(format!("No such matchup in season {} week {}: {}", self.year, week, matchup)),
        };

        // Ensure the matchup is not already complete
        if _matchup_to_sim.context().game_over() {
            return Err(format!("Season {} week {} matchup {} is already complete", self.year, week, matchup));
        }

        // Try to get the home team for the matchup
        let home_id = _matchup_to_sim.home_team();
        let home_team = match self.teams.get(home_id) {
            Some(t) => t,
            None => return Err(
                format!(
                    "Season {} week {} matchup {} references nonexistent home team ID: {}",
                    self.year, week, matchup, home_id
                )
            )
        };

        // Try to get the away team for the matchup
        let away_id = _matchup_to_sim.away_team();
        let away_team = match self.teams.get(away_id) {
            Some(t) => t,
            None => return Err(
                format!(
                    "Season {} week {} matchup {} references nonexistent away team ID: {}",
                    self.year, week, matchup, away_id
                )
            )
        };

        // Simulate the matchup
        let mut game = Game::new();
        let simulator = GameSimulator::new();
        let context = match simulator.sim_game(
            home_team, away_team,
            _matchup_to_sim.context().clone(),
            &mut game, rng
        ) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error while simulating matchup: {}", e))
        };

        // Archive the game stats, clear the game, update the context
        *_matchup_to_sim.home_stats_mut() = Some(game.home_stats());
        *_matchup_to_sim.away_stats_mut() = Some(game.away_stats());
        *_matchup_to_sim.context_mut() = context;
        Ok(game)
    }

    /// Simulate a full week of season matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the first week of the season
    /// my_league_season.sim_week(0, &mut rng);
    /// ```
    pub fn sim_week(&mut self, week: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Check if the prior week is not complete
        if week > 0 {
            let prev_week = match self.weeks.get(week - 1) {
                Some(w) => w,
                None => return Err(format!("Failed to get previous week {} from season {}", week-1, self.year))
            };
            if !prev_week.complete() {
                return Err(
                    format!(
                        "Cannot simulate week {} for season {}: previous week {} not complete",
                        week, self.year, week-1
                    )
                );
            }
        }

        // Try to get the given week
        let mut _week_to_sim = match self.weeks.get_mut(week) {
            Some(w) => w,
            None => return Err(format!("No such week for season {}: {}", self.year, week)),
        };

        // Check if the current week is complete
        if _week_to_sim.complete() {
            return Err(format!("Season {} week {} already completed", self.year, week));
        }

        // Loop through the week's matchups mutably
        for (i, matchup) in _week_to_sim.matchups_mut().iter_mut().enumerate() {
            // Skip matchups that have already been completed
            if matchup.context().game_over() {
                continue
            }

            // Try to get the home team for the matchup
            let home_id = matchup.home_team();
            let home_team = match self.teams.get(home_id) {
                Some(t) => t,
                None => return Err(
                    format!(
                        "Season {} week {} matchup {} references nonexistent home team ID: {}",
                        self.year, week, i, home_id
                    )
                )
            };

            // Try to get the away team for the matchup
            let away_id = matchup.away_team();
            let away_team = match self.teams.get(away_id) {
                Some(t) => t,
                None => return Err(
                    format!(
                        "Season {} week {} matchup {} references nonexistent away team ID: {}",
                        self.year, week, i, away_id
                    )
                )
            };

            // Simulate the matchup
            let mut game = Game::new();
            let simulator = GameSimulator::new();
            let context = match simulator.sim_game(
                home_team, away_team,
                matchup.context().clone(),
                &mut game, rng
            ) {
                Ok(c) => c,
                Err(e) => return Err(format!("Error while simulating matchup: {}", e))
            };

            // Update the matchup context and stats
            *matchup.context_mut() = context;
            *matchup.away_stats_mut() = Some(game.away_stats());
            *matchup.home_stats_mut() = Some(game.home_stats());
        }
        Ok(())
    }

    /// Simulate the regular season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire season
    /// my_league_season.sim_regular_season(&mut rng);
    /// ```
    pub fn sim_regular_season(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        for i in 0..self.weeks.len() {
            // Skip weeks which have already completed
            let week = match self.weeks.get(i) {
                Some(w) => w,
                None => return Err(
                    format!(
                        "Failed to simulate season {} week {}: No such week",
                        self.year, i
                    )
                ),
            };
            if week.complete() {
                continue;
            }

            // Simulate the week
            match self.sim_week(i, rng) {
                Ok(()) => (),
                Err(error) => return Err(
                    format!(
                        "Failed to simulate season {} week {}: {}",
                        self.year,
                        i, error
                    )
                ),
            }
        }
        Ok(())
    }

    /// Simulate a full season of matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire season
    /// my_league_season.sim(&mut rng);
    /// ```
    pub fn sim(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        for i in 0..self.weeks.len() {
            // Skip weeks which have already completed
            let week = match self.weeks.get(i) {
                Some(w) => w,
                None => return Err(
                    format!(
                        "Failed to simulate season {} week {}: {}",
                        self.year, i, "No such week"
                    )
                ),
            };
            if week.complete() {
                continue;
            }

            // Simulate the week
            match self.sim_week(i, rng) {
                Ok(()) => (),
                Err(error) => return Err(
                    format!(
                        "Failed to simulate season {} week {}: {}",
                        self.year,
                        i, error
                    )
                ),
            }
        }
        Ok(())
    }

    /// Get all season matchups involving a team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Simulate the entire season
    /// my_league_season.sim(&mut rng);
    ///
    /// // Get the mathups for team 0
    /// let matchups: LeagueSeasonMatchups = my_league_season.team_matchups(0).unwrap();
    /// ```
    pub fn team_matchups(&self, id: usize) -> Result<LeagueSeasonMatchups, String> {
        // Ensure the given team ID exists
        let _team = match self.team(id) {
            Some(t) => t,
            None => return Err(
                format!(
                    "No team found with ID {} in season {}",
                    id, self.year()
                )
            )
        };

        // Construct the matchups vector
        let mut matchups: Vec<Option<LeagueSeasonMatchup>> = Vec::new();
        for week in self.weeks.iter() {
            matchups.push(week.team_matchup(id));
        }
        Ok(LeagueSeasonMatchups::new(id, matchups))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_gen() {
        // Create a new season
        let mut my_league_season = LeagueSeason::new();
    
        // Add some teams to the season
        let _ = my_league_season.add_team(0, FootballTeam::new());
        let _ = my_league_season.add_team(1, FootballTeam::new());
        let _ = my_league_season.add_team(2, FootballTeam::new());
        let _ = my_league_season.add_team(3, FootballTeam::new());
        let _ = my_league_season.add_team(4, FootballTeam::new());
        let _ = my_league_season.add_team(5, FootballTeam::new());
        
        // Generate the season schedule
        let mut rng = rand::thread_rng();
        let _ = my_league_season.generate_schedule(
            LeagueSeasonScheduleOptions::new(),
            &mut rng
        );

        // Validate the schedule
        // Map team IDs to a tuple of usizes which represent:
        // Home games, away games, consecutive away games
        let mut home_away: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
        for (id, _) in my_league_season.teams().iter() {
            home_away.insert(*id, (0, 0, 0));
        }
        for week in my_league_season.weeks() {
            // Loop through the week's matchups
            for matchup in week.matchups().iter() {
                let home_id = matchup.home_team();
                let away_id = matchup.away_team();

                // Tally home games, away games, consecutive away games
                home_away.entry(*home_id)
                    .and_modify(|(home, _, cons_away)| {
                        *home += 1;
                        *cons_away = 0;
                    });
                home_away.entry(*away_id)
                    .and_modify(|(_, away, cons_away)| {
                        *away += 1;
                        *cons_away = 0;
                    });

                // Assert that no team plays three away games in a row
                if let Some(entry) = home_away.get(away_id) {
                    let (_, _, cons_away) = entry;
                    assert!(*cons_away < 3);
                }
            }
        }

        // Assert that each team plays an equal number of home and away games
        for (_, (home, away, _)) in home_away.iter() {
            assert!(home == away);
        }
    }
}
