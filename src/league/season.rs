#![doc = include_str!("../../docs/league/season.md")]
pub mod matchup;
pub mod playoffs;
pub mod week;

use std::collections::BTreeMap;

use crate::team::FootballTeam;
use crate::league::matchup::LeagueTeamRecord;
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
    pub permute: Option<bool>
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
            permute: None
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

        // Generate the round-robin schedule using the season team IDs
        for week_index in 0..num_weeks {
            // Create a new league season week
            let mut week = LeagueSeasonWeek::new();

            // TODO: Implement the ability to have bye weeks
            let num_matchups = num_teams / 2;

            // Create matchups for each pair of teams
            for matchup_index in 0..num_matchups {
                // Match up 0 : (n/2), 1 : (n/2)+1, ...
                let home_id = match team_ids.get(matchup_index) {
                    Some(id) => id,
                    None => return Err(
                        format!(
                            "While generating week {} matchup {}, no such home team ID: {}",
                            week_index,
                            matchup_index,
                            matchup_index
                        )
                    ),
                };
                let away_id = match team_ids.get(num_matchups + matchup_index) {
                    Some(id) => id,
                    None => return Err(
                        format!(
                            "While generating week {} matchup {}, no such away team ID: {}",
                            week_index,
                            matchup_index,
                            num_matchups + matchup_index
                        )
                    ),
                };
                
                // Get the home & away short names
                let home_short_name = self.teams.get(home_id).unwrap().short_name();
                let away_short_name = self.teams.get(away_id).unwrap().short_name();

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_short_name, away_short_name, rng);
                week.matchups_mut().push(matchup);
            }

            // Add the week to the season
            self.weeks.push(week);

            // Round robin the team IDs vec for the next week of matchups
            // Alternate home & away each week, "adjusted round robin"
            // Guarantees each team plays equal number of home and away games
            team_ids.rotate_right(1); // Round robin, rotate all
            if week_index % 2 == 0 {
                team_ids.swap(0, 1); // Round robin, fix 
            } else {
                team_ids.swap(num_matchups, num_matchups + 1);
            }
            team_ids.rotate_right(num_matchups);
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
            self.playoffs.add_team(*team_id, short_name)?;
        }

        // Generate the first round (or wild card round if not a power of 2)
        self.playoffs.gen_next_round(rng)?;
        Ok(())
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
