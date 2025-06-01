pub mod matchup;
pub mod team;
pub mod week;

use std::collections::BTreeMap;

use crate::league::season::team::LeagueSeasonTeam;
use crate::league::season::week::LeagueSeasonWeek;
use crate::league::season::matchup::LeagueSeasonMatchup;
use crate::sim::BoxScoreSimulator;
use crate::team::FootballTeam;

use chrono::Datelike;
use rand::Rng;
use serde::{Serialize, Deserialize, Deserializer};

/// # `LeagueSeasonRaw` struct
///
/// A `LeagueSeasonRaw` represents a freshly deserialized `LeagueSeason` prior
/// to any validation of the type having been completed.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonRaw {
    pub year: usize,
    pub teams: BTreeMap<usize, LeagueSeasonTeam>,
    pub weeks: Vec<LeagueSeasonWeek>
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
        LeagueSeasonRaw{
            year: chrono::Utc::now().year() as usize,
            teams: BTreeMap::new(),
            weeks: Vec::new()
        }
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
        if self.weeks.len() == 0 {
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

    /// Determine based on the matchups whether the season has completed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeasonRaw;
    ///
    /// let raw_league_season = LeagueSeasonRaw::new();
    /// let started = raw_league_season.complete();
    /// ```
    pub fn complete(&self) -> bool {
        // If no season weeks, then the season isn't complete
        if self.weeks.len() == 0 {
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
            if num_teams % 2 != 0 {
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
        // TODO: Add validation checking whether later weeks have been simulated before earlier weeks
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
                found_ids.push(home_id.clone());
                found_ids.push(away_id.clone());
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
    weeks: Vec<LeagueSeasonWeek>
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
                weeks: item.weeks
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
            weeks: Vec::new()
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
    /// let my_season_team = LeagueSeasonTeam::new();
    /// my_league_season.add_team(0, my_season_team);
    /// ```
    pub fn add_team(&mut self, id: usize, team: LeagueSeasonTeam) -> Result<(), String> {
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
    /// let my_season_team = LeagueSeasonTeam::new();
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
    /// let mut my_season_team = LeagueSeasonTeam::new();
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
        if self.weeks.len() == 0 {
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

    /// Determine based on the matchups whether the season has completed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// let my_league_season = LeagueSeason::new();
    /// let complete = my_league_season.complete();
    /// ```
    pub fn complete(&self) -> bool {
        // If no season weeks, then the season isn't complete
        if self.weeks.len() == 0 {
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

    /// Generate a schedule for the season.  The generated schedule is a round
    /// robin schedule in which each team plays an equal number of home and
    /// away games, and in which each team plays each other twice.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, LeagueSeasonTeam::new());
    /// my_league_season.add_team(1, LeagueSeasonTeam::new());
    /// my_league_season.add_team(2, LeagueSeasonTeam::new());
    /// my_league_season.add_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// my_league_season.generate_schedule();
    /// ```
    pub fn generate_schedule(&mut self) -> Result<(), String> {
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
        if num_teams % 2 != 0 {
            return Err(
                format!(
                    "Odd number of teams, cannot generate a schedule: {}",
                    num_teams
                )
            )
        }

        // Check to make sure the season has not already started
        if self.started() {
            return Err(
                "Season has already started, cannot re-generate schedule".to_string()
            )
        }

        // If the schedule is already non-empty then empty it before re-gen
        if self.weeks.len() > 0 {
            self.weeks.clear()
        }

        // TODO: Generate a random permutation of the season team IDs
        // TODO: Optionally accept a seed to control the schedule permutation
        let mut team_ids: Vec<usize> = self.teams.keys().cloned().collect();
        
        // TODO: Make number of weeks configurable
        let num_weeks = num_teams * 2;

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
                let matchup = LeagueSeasonMatchup::new(*home_id, *away_id);
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
        Ok(())
    }

    /// Simulate a season matchup
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, LeagueSeasonTeam::new());
    /// my_league_season.add_team(1, LeagueSeasonTeam::new());
    /// my_league_season.add_team(2, LeagueSeasonTeam::new());
    /// my_league_season.add_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// my_league_season.generate_schedule();
    ///
    /// // Simulate the first game of the first week
    /// let mut rng = rand::thread_rng();
    /// my_league_season.sim_matchup(0, 0, &mut rng);
    /// ```
    pub fn sim_matchup(&mut self, week: usize, matchup: usize, rng: &mut impl Rng) -> Result<(), String> {
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
        if *_matchup_to_sim.complete() {
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
        let home_sim_team = FootballTeam::from(home_team.clone());
        let away_sim_team = FootballTeam::from(away_team.clone());
        let simulator = BoxScoreSimulator::new();
        let box_score = match simulator.sim(&home_sim_team, &away_sim_team, rng) {
            Ok(score) => score,
            Err(error) => return Err(
                format!(
                    "Error while simulating matchup: {}",
                    error
                )
            )
        };

        // Update the status of the matchup
        *_matchup_to_sim.home_score_mut() = box_score.home_score() as usize;
        *_matchup_to_sim.away_score_mut() = box_score.away_score() as usize;
        *_matchup_to_sim.complete_mut() = true;
        Ok(())
    }

    /// Simulate a full week of season matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, LeagueSeasonTeam::new());
    /// my_league_season.add_team(1, LeagueSeasonTeam::new());
    /// my_league_season.add_team(2, LeagueSeasonTeam::new());
    /// my_league_season.add_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// my_league_season.generate_schedule();
    ///
    /// // Simulate the first week of the season
    /// let mut rng = rand::thread_rng();
    /// my_league_season.sim_week(0, &mut rng);
    /// ```
    pub fn sim_week(&mut self, week: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Try to get the given week
        let mut _week_to_sim = match self.weeks.get_mut(week) {
            Some(w) => w,
            None => return Err(format!("No such week for season {}: {}", self.year, week)),
        };

        // Loop through the week's matchups mutably
        for (i, matchup) in _week_to_sim.matchups_mut().iter_mut().enumerate() {
            // Skip matchups that have already been completed
            if *matchup.complete() {
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
            let home_sim_team = FootballTeam::from(home_team.clone());
            let away_sim_team = FootballTeam::from(away_team.clone());
            let simulator = BoxScoreSimulator::new();
            let box_score = match simulator.sim(&home_sim_team, &away_sim_team, rng) {
                Ok(score) => score,
                Err(error) => return Err(
                    format!(
                        "Error while simulating matchup: {}",
                        error
                    )
                )
            };

            // Update the status of the matchup
            *matchup.home_score_mut() = box_score.home_score() as usize;
            *matchup.away_score_mut() = box_score.away_score() as usize;
            *matchup.complete_mut() = true;
        }
        Ok(())
    }

    /// Simulate a full season of matchups
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Create a new season
    /// let mut my_league_season = LeagueSeason::new();
    ///
    /// // Add 4 teams to the season
    /// my_league_season.add_team(0, LeagueSeasonTeam::new());
    /// my_league_season.add_team(1, LeagueSeasonTeam::new());
    /// my_league_season.add_team(2, LeagueSeasonTeam::new());
    /// my_league_season.add_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// my_league_season.generate_schedule();
    ///
    /// // Simulate the entire season
    /// let mut rng = rand::thread_rng();
    /// my_league_season.sim(&mut rng);
    /// ```
    pub fn sim(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        for i in 0..self.weeks.len() {
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
}
