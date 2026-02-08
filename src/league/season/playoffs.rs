#![doc = include_str!("../../../docs/league/season/playoffs.md")]
pub mod picture;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use rand::Rng;
use serde::{Serialize, Deserialize, Deserializer};
use std::collections::{BTreeMap, HashSet};

use crate::game::matchup::FootballMatchupResult;
use crate::league::matchup::LeagueTeamRecord;
use crate::league::season::week::LeagueSeasonWeek;
use crate::league::season::matchup::LeagueSeasonMatchup;

/// Maximum allowed length for a playoff team short name
const MAX_PLAYOFF_TEAM_SHORT_NAME_LEN: usize = 4;

/// # `PlayoffTeamRaw` struct
///
/// A freshly deserialized `PlayoffTeam` prior to validation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct PlayoffTeamRaw {
    pub seed: usize,
    pub short_name: String,
}

impl PlayoffTeamRaw {
    /// Validate the raw playoff team
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeamRaw;
    ///
    /// let raw = PlayoffTeamRaw { seed: 1, short_name: "TM".to_string() };
    /// assert!(raw.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.short_name.len() > MAX_PLAYOFF_TEAM_SHORT_NAME_LEN {
            return Err(format!(
                "Playoff team short name '{}' exceeds maximum length of {} characters",
                self.short_name, MAX_PLAYOFF_TEAM_SHORT_NAME_LEN
            ));
        }
        Ok(())
    }
}

impl TryFrom<PlayoffTeamRaw> for PlayoffTeam {
    type Error = String;

    fn try_from(raw: PlayoffTeamRaw) -> Result<Self, Self::Error> {
        raw.validate()?;
        Ok(PlayoffTeam {
            seed: raw.seed,
            short_name: raw.short_name,
        })
    }
}

/// # `PlayoffTeamsRaw` struct
///
/// A freshly deserialized `PlayoffTeams` prior to validation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct PlayoffTeamsRaw {
    pub teams: BTreeMap<usize, BTreeMap<usize, PlayoffTeam>>,
}

impl PlayoffTeamsRaw {
    /// Validate the raw playoff teams
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeamsRaw;
    /// use std::collections::BTreeMap;
    ///
    /// let raw = PlayoffTeamsRaw { teams: BTreeMap::new() };
    /// assert!(raw.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // Check for duplicate team IDs across conferences
        let mut seen = HashSet::new();
        for (conf_id, conference_teams) in &self.teams {
            for &team_id in conference_teams.keys() {
                if !seen.insert(team_id) {
                    return Err(format!(
                        "Duplicate team ID {} found across conferences in playoffs (detected in conference {})",
                        team_id, conf_id
                    ));
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<PlayoffTeamsRaw> for PlayoffTeams {
    type Error = String;

    fn try_from(raw: PlayoffTeamsRaw) -> Result<Self, Self::Error> {
        raw.validate()?;
        Ok(PlayoffTeams {
            teams: raw.teams,
        })
    }
}

/// # `LeagueSeasonPlayoffsRaw` struct
///
/// A freshly deserialized `LeagueSeasonPlayoffs` prior to validation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonPlayoffsRaw {
    pub teams: PlayoffTeams,
    pub conference_brackets: BTreeMap<usize, Vec<LeagueSeasonWeek>>,
    #[serde(default)]
    pub winners_bracket: Vec<LeagueSeasonWeek>,
}

impl LeagueSeasonPlayoffsRaw {
    /// Validate the raw playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffsRaw;
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    /// use std::collections::BTreeMap;
    ///
    /// let raw = LeagueSeasonPlayoffsRaw {
    ///     teams: PlayoffTeams::new(),
    ///     conference_brackets: BTreeMap::new(),
    ///     winners_bracket: Vec::new(),
    /// };
    /// assert!(raw.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // Collect all valid team IDs from the playoff teams
        let valid_team_ids: HashSet<usize> = self.teams.iter().collect();

        // Validate all matchups in conference brackets reference valid team IDs
        for (conf_id, rounds) in &self.conference_brackets {
            for (round_index, round) in rounds.iter().enumerate() {
                for (matchup_index, matchup) in round.matchups().iter().enumerate() {
                    let home_id = *matchup.home_team();
                    let away_id = *matchup.away_team();
                    if !valid_team_ids.contains(&home_id) {
                        return Err(format!(
                            "Conference {} bracket round {} matchup {} references nonexistent home team ID: {}",
                            conf_id, round_index, matchup_index, home_id
                        ));
                    }
                    if !valid_team_ids.contains(&away_id) {
                        return Err(format!(
                            "Conference {} bracket round {} matchup {} references nonexistent away team ID: {}",
                            conf_id, round_index, matchup_index, away_id
                        ));
                    }
                }
            }
        }

        // Validate all matchups in the winners bracket reference valid team IDs
        for (round_index, round) in self.winners_bracket.iter().enumerate() {
            for (matchup_index, matchup) in round.matchups().iter().enumerate() {
                let home_id = *matchup.home_team();
                let away_id = *matchup.away_team();
                if !valid_team_ids.contains(&home_id) {
                    return Err(format!(
                        "Winners bracket round {} matchup {} references nonexistent home team ID: {}",
                        round_index, matchup_index, home_id
                    ));
                }
                if !valid_team_ids.contains(&away_id) {
                    return Err(format!(
                        "Winners bracket round {} matchup {} references nonexistent away team ID: {}",
                        round_index, matchup_index, away_id
                    ));
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<LeagueSeasonPlayoffsRaw> for LeagueSeasonPlayoffs {
    type Error = String;

    fn try_from(raw: LeagueSeasonPlayoffsRaw) -> Result<Self, Self::Error> {
        raw.validate()?;
        Ok(LeagueSeasonPlayoffs {
            teams: raw.teams,
            conference_brackets: raw.conference_brackets,
            winners_bracket: raw.winners_bracket,
        })
    }
}

/// # `PlayoffTeam` struct
///
/// Represents a single team's playoff entry with its seed and short name.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize)]
pub struct PlayoffTeam {
    seed: usize,
    short_name: String,
}

impl<'de> Deserialize<'de> for PlayoffTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = PlayoffTeamRaw::deserialize(deserializer)?;
        PlayoffTeam::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl PlayoffTeam {
    /// Get the team's playoff seed
    pub fn seed(&self) -> usize {
        self.seed
    }

    /// Get the team's short name
    pub fn short_name(&self) -> &str {
        &self.short_name
    }
}

/// # `PlayoffTeams` struct
///
/// A collection of teams participating in the playoffs, organized by conference.
/// Conference ID 0 is used for non-conference playoffs.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize)]
pub struct PlayoffTeams {
    /// conference_id -> team_id -> PlayoffTeam
    teams: BTreeMap<usize, BTreeMap<usize, PlayoffTeam>>,
}

impl PlayoffTeams {
    /// Create a new empty PlayoffTeams collection
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let my_teams = PlayoffTeams::new();
    /// ```
    pub fn new() -> PlayoffTeams {
        PlayoffTeams::default()
    }

    /// Add a team to the playoffs
    ///
    /// Teams are expected to be added in seed order within each conference.
    /// Seed is calculated based on number of teams already in the conference bracket.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let res = my_teams.add(
    ///     0,          // Team ID
    ///     "TM",       // Short name
    ///     0           // Conference ID
    /// );
    /// assert!(res.is_ok());
    /// ```
    pub fn add(&mut self, team_id: usize, short_name: &str, conference: usize) -> Result<(), String> {
        let conference_teams = self.teams.entry(conference).or_default();
        if conference_teams.contains_key(&team_id) {
            return Err(format!("Team {} is already in conference {}", team_id, conference));
        }

        let seed = conference_teams.len() + 1;
        conference_teams.insert(team_id, PlayoffTeam {
            seed,
            short_name: short_name.to_string(),
        });
        Ok(())
    }

    /// Get a team by ID (searches all conferences)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "TM", 0);
    ///
    /// let existing_team = my_teams.get(0);
    /// assert!(existing_team.is_some());
    /// assert!(existing_team.unwrap().seed() == 1);
    ///
    /// let nonexistent_team = my_teams.get(1);
    /// assert!(nonexistent_team.is_none());
    /// ```
    pub fn get(&self, team_id: usize) -> Option<&PlayoffTeam> {
        for conference_teams in self.teams.values() {
            if let Some(team) = conference_teams.get(&team_id) {
                return Some(team);
            }
        }
        None
    }

    /// Get all teams in a specific conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// let _ = my_teams.add(1, "B", 0);
    /// let _ = my_teams.add(2, "C", 1);
    /// let _ = my_teams.add(3, "D", 1);
    ///
    /// let existing_conference = my_teams.get_conference(1);
    /// assert!(existing_conference.is_some());
    /// assert_eq!(existing_conference.unwrap().len(), 2);
    ///
    /// let nonexistent_conference = my_teams.get_conference(2);
    /// assert!(nonexistent_conference.is_none());
    /// ```
    pub fn get_conference(&self, conference: usize) -> Option<&BTreeMap<usize, PlayoffTeam>> {
        self.teams.get(&conference)
    }

    /// Check if a team is in the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// assert!(my_teams.contains(0));  // Existing team
    /// assert!(!my_teams.contains(1)); // Nonexistent team
    /// ```
    pub fn contains(&self, team_id: usize) -> bool {
        self.get(team_id).is_some()
    }

    /// Get the total number of teams across all conferences
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// assert!(my_teams.len() == 1);
    /// ```
    pub fn len(&self) -> usize {
        self.teams.values().map(|c| c.len()).sum()
    }

    /// Check if there are no teams
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// assert!(my_teams.is_empty());
    ///
    /// // Add a team which should cause the playoffs to become nonempty
    /// let _ = my_teams.add(0, "A", 0);
    /// assert!(!my_teams.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.teams.is_empty() || self.teams.values().all(|c| c.is_empty())
    }

    /// Get the number of conferences
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// let _ = my_teams.add(1, "B", 0);
    /// let _ = my_teams.add(2, "C", 1);
    /// let _ = my_teams.add(3, "D", 1);
    /// assert!(my_teams.num_conferences() == 2);
    /// ```
    pub fn num_conferences(&self) -> usize {
        self.teams.len()
    }

    /// Iterate over conference IDs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// let _ = my_teams.add(1, "B", 0);
    /// let _ = my_teams.add(2, "C", 1);
    /// let _ = my_teams.add(3, "D", 1);
    /// for (i, conference) in my_teams.conferences().enumerate() {
    ///     assert!(i == *conference); // Note conference IDs are borrowed
    /// }
    /// ```
    pub fn conferences(&self) -> impl Iterator<Item = &usize> {
        self.teams.keys()
    }

    /// Iterate over all teams across all conferences
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    /// let _ = my_teams.add(1, "B", 0);
    /// let _ = my_teams.add(2, "C", 1);
    /// let _ = my_teams.add(3, "D", 1);
    /// let team_ids: Vec<usize> = my_teams.iter().collect();
    /// assert_eq!(team_ids, vec![0, 1, 2, 3]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.teams.values().flat_map(|c| c.keys().copied())
    }

    /// Get teams in a conference sorted by seed
    ///
    /// Returns a vector of `(team_id, &PlayoffTeam)` pairs sorted by seed.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(3, "A", 0);
    /// let _ = my_teams.add(1, "B", 0);
    /// let by_seed = my_teams.conference_teams_by_seed(0);
    /// assert_eq!(by_seed.len(), 2);
    /// assert_eq!(by_seed[0].0, 3); // team_id 3 is seed 1
    /// assert_eq!(by_seed[1].0, 1); // team_id 1 is seed 2
    /// ```
    pub fn conference_teams_by_seed(&self, conference: usize) -> Vec<(usize, &PlayoffTeam)> {
        if let Some(conference_teams) = self.teams.get(&conference) {
            let mut teams: Vec<(usize, &PlayoffTeam)> = conference_teams
                .iter()
                .map(|(&team_id, team)| (team_id, team))
                .collect();
            teams.sort_by_key(|(_, team)| team.seed);
            teams
        } else {
            Vec::new()
        }
    }

    /// Get a team by seed within a specific conference
    ///
    /// Returns a `(team_id, &PlayoffTeam)` pair if found.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::PlayoffTeams;
    ///
    /// let mut my_teams = PlayoffTeams::new();
    /// let _ = my_teams.add(0, "A", 0);
    ///
    /// let existing_team = my_teams.get_by_seed(
    ///     0,  // Conference ID
    ///     1   // Team seed
    /// );
    /// assert!(existing_team.is_some());
    /// let (team_id, team) = existing_team.unwrap();
    /// assert!(team_id == 0);
    /// assert!(team.seed() == 1);
    ///
    /// let nonexistent_team = my_teams.get_by_seed(0, 2);
    /// assert!(nonexistent_team.is_none());
    /// ```
    pub fn get_by_seed(&self, conference: usize, seed: usize) -> Option<(usize, &PlayoffTeam)> {
        self.teams
            .get(&conference)?
            .iter()
            .find(|(_, team)| team.seed == seed)
            .map(|(&team_id, team)| (team_id, team))
    }
}

impl<'de> Deserialize<'de> for PlayoffTeams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = PlayoffTeamsRaw::deserialize(deserializer)?;
        PlayoffTeams::try_from(raw).map_err(serde::de::Error::custom)
    }
}

/// # `LeagueSeasonPlayoffs` struct
///
/// A `LeagueSeasonPlayoffs` represents football season playoffs.
///
/// Rounds are organized by conference bracket ID. Single-conference (non-conference)
/// playoffs use bracket ID 0. Multi-conference playoffs have one bracket per
/// conference, plus a `winners_bracket` for the championship between conference winners.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[cfg_attr(feature = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize)]
pub struct LeagueSeasonPlayoffs {
    /// Teams participating in the playoffs
    teams: PlayoffTeams,
    /// Conference bracket ID -> rounds for that bracket.
    /// Single-conference playoffs use bracket ID 0.
    conference_brackets: BTreeMap<usize, Vec<LeagueSeasonWeek>>,
    /// Winners bracket for championship game(s) between conference champions.
    /// Only used in multi-conference playoffs.
    #[serde(default)]
    winners_bracket: Vec<LeagueSeasonWeek>,
}

impl<'de> Deserialize<'de> for LeagueSeasonPlayoffs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = LeagueSeasonPlayoffsRaw::deserialize(deserializer)?;
        LeagueSeasonPlayoffs::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl LeagueSeasonPlayoffs {
    /// Initialize a new LeagueSeasonPlayoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// ```
    pub fn new() -> LeagueSeasonPlayoffs {
        LeagueSeasonPlayoffs::default()
    }

    /// Borrow the winners bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// let winners_bracket = my_playoffs.winners_bracket();
    /// ```
    pub fn winners_bracket(&self) -> &Vec<LeagueSeasonWeek> {
        &self.winners_bracket
    }

    /// Mutably borrow the winners bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let winners_bracket = my_playoffs.winners_bracket_mut();
    /// ```
    pub fn winners_bracket_mut(&mut self) -> &mut Vec<LeagueSeasonWeek> {
        &mut self.winners_bracket
    }

    /// Borrow the playoffs' conference brackets
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// let conference_brackets = my_playoffs.conference_brackets();
    /// ```
    pub fn conference_brackets(&self) -> &BTreeMap<usize, Vec<LeagueSeasonWeek>> {
        &self.conference_brackets
    }

    /// Mutably borrow the playoffs' conference brackets
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let conference_brackets = my_playoffs.conference_brackets_mut();
    /// ```
    pub fn conference_brackets_mut(&mut self) -> &mut BTreeMap<usize, Vec<LeagueSeasonWeek>> {
        &mut self.conference_brackets
    }

    /// Get the rounds for a specific conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// let conference_bracket = my_playoffs.conference_bracket(0);
    /// assert!(conference_bracket.is_none());
    /// ```
    pub fn conference_bracket(&self, conference: usize) -> Option<&Vec<LeagueSeasonWeek>> {
        self.conference_brackets.get(&conference)
    }

    /// Mutably get the rounds for a specific conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let conference_bracket = my_playoffs.conference_bracket_mut(0);
    /// assert!(conference_bracket.is_none());
    /// ```
    pub fn conference_bracket_mut(&mut self, conference: usize) -> Option<&mut Vec<LeagueSeasonWeek>> {
        self.conference_brackets.get_mut(&conference)
    }

    /// Get the number of teams in the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(my_playoffs.num_teams() == 0);
    /// ```
    pub fn num_teams(&self) -> usize {
        self.teams.len()
    }

    /// Check if a team is in the playoffs given its ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.team_in_playoffs(0));
    /// ```
    pub fn team_in_playoffs(&self, team_id: usize) -> bool {
        self.teams.contains(team_id)
    }

    /// Find the seed for a given team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Create playoffs and add a team
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    ///
    /// // Get that team's seed
    /// let seed = my_playoffs.team_seed(0);
    /// assert!(seed.is_ok());
    /// assert!(seed.unwrap() == 1);
    /// ```
    pub fn team_seed(&self, team_id: usize) -> Result<usize, String> {
        self.teams
            .get(team_id)
            .map(|team| team.seed())
            .ok_or_else(|| format!("Team {} not in playoffs", team_id))
    }

    /// Find the conference for a given team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Create playoffs and add a team
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", Some(0));
    ///
    /// // Get that team's conference
    /// let seed = my_playoffs.team_conference(0);
    /// assert!(seed.is_ok());
    /// assert!(seed.unwrap() == 0);
    /// ```
    pub fn team_conference(&self, team_id: usize) -> Result<usize, String> {
        if !self.teams.contains(team_id) {
            return Err(
                format!(
                    "Team {} does not exist in playoffs",
                    team_id
                )
            )
        }
        for conference in self.teams.conferences() {
            if let Some(c) = self.teams.get_conference(*conference) {
                for id in c.keys() {
                    if *id == team_id {
                        return Ok(*conference);
                    }
                }
            }
        }
        Err(
            format!(
                "Team {} exists but was not found in any conference",
                team_id
            )
        )
    }

    /// Check if this is a conference-based playoff
    ///
    /// Returns true if teams are spread across multiple conferences.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.is_conference_playoff());
    /// ```
    pub fn is_conference_playoff(&self) -> bool {
        self.teams.num_conferences() > 1
    }

    /// Get the number of conferences in the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert_eq!(my_playoffs.num_conferences(), 0);
    /// ```
    pub fn num_conferences(&self) -> usize {
        self.teams.num_conferences()
    }

    /// Get the team IDs for a specific conference bracket sorted by seed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(1, "ME", Some(0));
    /// let teams = my_playoffs.conference_teams(0);
    /// assert_eq!(teams.len(), 1);
    /// ```
    pub fn conference_teams(&self, conf_index: usize) -> Vec<(usize, &PlayoffTeam)> {
        self.teams.conference_teams_by_seed(conf_index)
    }

    /// Determine whether a given conference bracket has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.conference_bracket_started(0));
    /// ```
    pub fn conference_bracket_started(&self, conference: usize) -> bool {
        if let Some(bracket) = self.conference_brackets.get(&conference) {
            // If any round has started, the playoffs have started
            for round in bracket {
                if round.started() {
                    return true;
                }
            }
        }
        false
    }

    /// Determine whether the conference brackets have sterted
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.conference_brackets_started());
    /// ```
    pub fn conference_brackets_started(&self) -> bool {
        for (_, bracket) in self.conference_brackets.iter() {
            // If any round has started, the playoffs have started
            for round in bracket {
                if round.started() {
                    return true;
                }
            }
        }
        false
    }

    /// Determine whether a given conference bracket is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.conference_bracket_complete(0));
    /// ```
    pub fn conference_bracket_complete(&self, conference: usize) -> bool {
        if let Some(bracket) = self.conference_brackets.get(&conference) {
            // If no rounds generated yet, it hasn't completed
            if bracket.is_empty() {
                return false;
            }
            for round in bracket {
                // If any round hasn't started or hasn't finished yet, it hasn't completed
                if !(round.started() && round.complete()) {
                    return false;
                }

                // This conference playoff bracket is complete when only 1 team remains
                let conference = match self.teams.get_conference(conference) {
                    Some(c) => c,
                    None => return false,
                };
                let mut eliminated = 0;
                for round in bracket {
                    eliminated += round.matchups().len();
                }
                if conference.len() != eliminated + 1 {
                    return false;
                }
            }
            return true;
        }
        
        // If no round was found, it hasn't completed
        false
    }

    /// Determine whether the conference brackets are all complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.conference_brackets_complete());
    /// ```
    pub fn conference_brackets_complete(&self) -> bool {
        // If conference brackets have not started yet, they haven't completed
        if !self.conference_brackets_started() {
            return false;
        }

        // Check if each conference bracket is complete, count eliminated teams
        for (i, bracket) in self.conference_brackets.iter() {
            // If any round is not yet complete, the playoffs are not complete
            for round in bracket {
                if !round.complete() {
                    return false;
                }
            }

            // This conference playoff bracket is complete when only 1 team remains
            let conference = match self.teams.get_conference(*i) {
                Some(c) => c,
                None => return false
            };
            let mut eliminated = 0;
            for round in bracket {
                eliminated += round.matchups().len();
            }
            if conference.len() != eliminated + 1 {
                return false;
            }
        }
        true
    }

    /// Get the team ID of the conference champion for a specific conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(my_playoffs.conference_champion(0).is_none());
    /// ```
    pub fn conference_champion(&self, conference: usize) -> Option<usize> {
        // Check if the conference bracket is complete
        if !self.conference_bracket_complete(conference) {
            return None;
        }

        // Get the conference champion
        if let Some(bracket) = self.conference_brackets.get(&conference) {
            if let Some(round) = bracket.last() {
                if let Some(matchup) = round.matchups().first() {
                    return matchup.winner();
                }
            }
        }
        None
    }

    /// Determine whether the winners bracket has started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.winners_bracket_started());
    /// ```
    pub fn winners_bracket_started(&self) -> bool {
        if !self.winners_bracket.is_empty() {
            for round in self.winners_bracket.iter() {
                if round.started() {
                    return true;
                }
            }
        }
        false
    }

    /// Determine whether the winners bracket is complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.winners_bracket_complete());
    /// ```
    pub fn winners_bracket_complete(&self) -> bool {
        // If no winners bracket hasn't started yet, it hasn't completed
        if !self.winners_bracket_started() {
            return false;
        }

        // Get the final round of the winners bracket
        if let Some(final_round) = self.winners_bracket.last() {
            // If more than one matchup, it hasn't completed
            let matchups = final_round.matchups().len();
            if matchups > 1 {
                return false;
            }

            // If one matchup and round, but more than 2 conferences, it hasn't completed
            let conferences = self.num_conferences();
            if self.winners_bracket.len() == 1 && matchups == 1 && conferences > 2 {
                return false;
            }

            // If we reach this point, the one matchup left is the championship
            if let Some(final_matchup) = final_round.matchups().first() {
                return final_matchup.context().game_over();
            }
        }
        false
    }

    /// Determine whether the playoffs have started
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.started());
    /// ```
    pub fn started(&self) -> bool {
        self.conference_brackets_started()
    }

    /// Determine whether the playoffs are complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(!my_playoffs.complete());
    /// ```
    pub fn complete(&self) -> bool {
        // If no conference brackets generated yet, they haven't started
        // If conference brackets are not complete yet, they haven't completed
        if self.conference_brackets.is_empty() || !self.conference_brackets_complete() {
            return false;
        }

        // If conference playoff, check if winners bracket is complete
        // Non-conference playoff, if this point is reached they are complete
        if self.is_conference_playoff() {
            self.winners_bracket_complete()
        } else {
            true
        }
    }

    /// Add a team to the playoffs
    ///
    /// If `conference` is `None`, the team is added to the default conference (0).
    /// Teams should be added in seed order within each conference.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// // Non-conference playoff
    /// let res = my_playoffs.add_team(0, "ME", None);
    /// assert!(res.is_ok());
    ///
    /// // Conference playoff
    /// let mut conf_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = conf_playoffs.add_team(0, "YOU", Some(0));
    /// let _ = conf_playoffs.add_team(1, "THEM", Some(1));
    /// ```
    pub fn add_team(&mut self, team_id: usize, name: &str, conference: Option<usize>) -> Result<(), String> {
        // Ensure the playoffs have not already started
        if self.started() {
            return Err(String::from("Playoffs have already started, cannot add new team"));
        }

        let conf = conference.unwrap_or(0);
        self.teams.add(team_id, name, conf)
    }

    /// Helper method to calculate the number of first round teams
    fn num_first_round_teams(&self, num_teams: usize) -> Result<usize, String> {
        if num_teams < 2 {
            return Err(
                format!(
                    "Playoffs must contain at least 2 teams per conference, got {}",
                    num_teams
                )
            )
        }
        let next_power = num_teams.checked_next_power_of_two().ok_or(
            String::from("Failed to calculate first round conference matchups")
        )?;
        if next_power == num_teams {
            Ok(next_power)
        } else {
            next_power.checked_div(2).ok_or(
                String::from("Failed to calculate first round conference matchups")
            )
        }
    }

    /// Get the number of teams that will appear in the first round of a given
    /// conference bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    /// let _ = my_playoffs.add_team(1, "YOU", None);
    /// let _ = my_playoffs.add_team(2, "THEM", None);
    ///
    /// // Get the number of first round teams
    /// let first_round_teams = my_playoffs.first_round_teams(None);
    /// assert!(first_round_teams.is_ok());
    /// assert!(first_round_teams.unwrap() == 2);
    /// ```
    pub fn first_round_teams(&self, conference: Option<usize>) -> Result<usize, String> {
        let conf = conference.unwrap_or_default();
        let teams_per_conference = self.conference_teams(conf).len();
        self.num_first_round_teams(teams_per_conference)
    }

    /// Get the number of teams that will appear in the first round of the
    /// winners bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "A", Some(0));
    /// let _ = my_playoffs.add_team(1, "B", Some(0));
    /// let _ = my_playoffs.add_team(2, "C", Some(1));
    /// let _ = my_playoffs.add_team(3, "D", Some(1));
    /// let _ = my_playoffs.add_team(4, "E", Some(2));
    /// let _ = my_playoffs.add_team(5, "F", Some(2));
    ///
    /// // Get the number of first round teams
    /// let first_round_teams = my_playoffs.first_round_winners();
    /// assert!(first_round_teams.is_ok());
    /// assert!(first_round_teams.unwrap() == 2);
    /// ```
    pub fn first_round_winners(&self) -> Result<usize, String> {
        let conferences = self.num_conferences();
        match conferences {
            0 => Err(String::from("No conferences yet to determine winners bracket")),
            1 => Ok(0),
            _ => self.num_first_round_teams(conferences)
        }
    }

    /// Helper method to calculate the number of wild card teams
    fn num_wild_card_teams(&self, num_teams: usize, first_round_teams: usize) -> Result<usize, String> {
        if num_teams == 1 && first_round_teams == 0 {
            return Ok(0);
        }
        let k = num_teams.checked_sub(first_round_teams).ok_or(
            String::from("Failed to calculate wild cards")
        )?;
        Ok(2 * k)
    }

    /// Get the number of teams that will appear in the wild card round of the
    /// conference round
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    /// let _ = my_playoffs.add_team(1, "YOU", None);
    /// let _ = my_playoffs.add_team(2, "THEM", None);
    ///
    /// // Get the number of wild card teams
    /// let wild_cards = my_playoffs.wild_cards(None);
    /// assert!(wild_cards.is_ok());
    /// assert!(wild_cards.unwrap() == 2);
    /// ```
    pub fn wild_cards(&self, conference: Option<usize>) -> Result<usize, String> {
        let conf = conference.unwrap_or_default();
        let num_teams = self.conference_teams(conf).len();
        let first_round_teams = self.first_round_teams(conference)?;
        self.num_wild_card_teams(num_teams, first_round_teams)
    }

    /// Get the number of teams that will appear in the wild card round of the
    /// winners bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "A", Some(0));
    /// let _ = my_playoffs.add_team(1, "B", Some(0));
    /// let _ = my_playoffs.add_team(2, "C", Some(1));
    /// let _ = my_playoffs.add_team(3, "D", Some(1));
    /// let _ = my_playoffs.add_team(4, "E", Some(2));
    /// let _ = my_playoffs.add_team(5, "F", Some(2));
    ///
    /// // Get the number of first round teams
    /// let wild_card_teams = my_playoffs.wild_card_winners();
    /// assert!(wild_card_teams.is_ok());
    /// assert!(wild_card_teams.unwrap() == 2);
    /// ```
    pub fn wild_card_winners(&self) -> Result<usize, String> {
        let num_teams = self.num_conferences();
        let first_round_teams = self.first_round_winners()?;
        self.num_wild_card_teams(num_teams, first_round_teams)
    }

    /// Helper method to calculate the number of teams that will have a bye
    fn num_bye_teams(&self, num_teams: usize, first_round_teams: usize) -> Result<usize, String> {
        if num_teams == 0 && first_round_teams == 1 {
            return Ok(0);
        }
        let k = num_teams.checked_sub(first_round_teams).ok_or(
            String::from("Failed to calculate byes")
        )?;
        let byes = num_teams.checked_sub(2 * k).ok_or(
            String::from("Failed to calculate byes")
        )?;
        Ok(byes)
    }

    /// Get the number of teams that will have a bye in a given conference
    /// bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    /// let _ = my_playoffs.add_team(1, "YOU", None);
    /// let _ = my_playoffs.add_team(2, "THEM", None);
    ///
    /// // Get the number of byes
    /// let byes = my_playoffs.byes(None);
    /// assert!(byes.is_ok());
    /// assert!(byes.unwrap() == 1);
    /// ```
    pub fn byes(&self, conference: Option<usize>) -> Result<usize, String> {
        let conf = conference.unwrap_or_default();
        let num_teams = self.conference_teams(conf).len();
        let first_round_teams = self.first_round_teams(conference)?;
        self.num_bye_teams(num_teams, first_round_teams)
    }

    /// Get the number of teams that will have a bye in the winners bracket
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "A", Some(0));
    /// let _ = my_playoffs.add_team(1, "B", Some(0));
    /// let _ = my_playoffs.add_team(2, "C", Some(1));
    /// let _ = my_playoffs.add_team(3, "D", Some(1));
    /// let _ = my_playoffs.add_team(4, "E", Some(2));
    /// let _ = my_playoffs.add_team(5, "F", Some(2));
    ///
    /// // Get the number of winners bracket first round byes
    /// let winners_bracket_byes = my_playoffs.winners_bracket_byes();
    /// assert!(winners_bracket_byes.is_ok());
    /// assert!(winners_bracket_byes.unwrap() == 1);
    /// ```
    pub fn winners_bracket_byes(&self) -> Result<usize, String> {
        let num_teams = self.num_conferences();
        let first_round_teams = self.first_round_winners()?;
        self.num_bye_teams(num_teams, first_round_teams)
    }

    /// Helper method for generating a conference's wild card matchups
    fn gen_conference_wild_card_round(&mut self, conference: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure conditions are valid to generate conference wild card round
        let num_teams = self.conference_teams(conference).len();
        if num_teams < 2 {
            return Err(
                format!(
                    "Conference {} bracket must contain at least 2 teams, got {}",
                    conference,
                    num_teams
                )
            )
        }
        if num_teams.is_power_of_two() {
            return Err(
                format!(
                    "No wild card round for conference {} bracket with a power of 2 teams: {}",
                    conference,
                    num_teams
                )
            )
        }
        if self.conference_brackets_started() {
            return Err(
                String::from(
                    "Cannot re-generate wild card rounds, conference playoffs already started"
                )
            )
        }

        // Clear conference bracket if it already exists
        self.conference_brackets.insert(conference, Vec::new());

        // Get the number of wild card teams and byes
        let byes = self.byes(Some(conference))?;
        let wild_cards = self.wild_cards(Some(conference))?;
        let wild_card_matchups = wild_cards.checked_div(2).ok_or(
            String::from("Failed to calculate wild card matchups")
        )?;

        // Match up the wild card teams against one another (using default conference 0)
        let mut week = LeagueSeasonWeek::new();
        for i in 0..wild_card_matchups {
            // Get the home and away teams by seed
            let home_seed = byes + i + 1;
            let away_seed = num_teams - i;
            let (home_team_id, home_team) = self.teams.get_by_seed(conference, home_seed)
                .ok_or_else(|| format!(
                    "No team found in conference {} with seed {}",
                    conference,
                    home_seed
                ))?;
            let (away_team_id, away_team) = self.teams.get_by_seed(conference, away_seed)
                .ok_or_else(|| format!(
                    "No team found in conference {} with seed {}",
                    conference,
                    away_seed
                ))?;

            // Create the matchup and add to the week
            let matchup = LeagueSeasonMatchup::new(
                home_team_id,
                away_team_id,
                home_team.short_name(),
                away_team.short_name(),
                rng
            );
            week.matchups_mut().push(matchup);
        }

        // Add the week to the conference bracket and return
        self.conference_brackets.entry(conference).or_default().push(week);
        Ok(())
    }

    /// Helper method for generating the winners bracket's wild card matchups
    fn gen_winners_wild_card_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure conditions are valid to generate conference wild card round
        let num_teams = self.num_conferences();
        if num_teams < 2 {
            return Err(
                format!(
                    "Must contain at least 2 conferences for a winners bracket, got {}",
                    num_teams
                )
            )
        }
        if num_teams.is_power_of_two() {
            return Err(
                format!(
                    "No wild card round for winners bracket with a power of 2 teams: {}",
                    num_teams
                )
            )
        }
        if !self.conference_brackets_complete() {
            return Err(
                String::from(
                    "Cannot generate winners bracket, conference playoffs not complete"
                )
            )
        }
        if self.winners_bracket_started() {
            return Err(
                String::from(
                    "Cannot re-generate winners bracket, already started"
                )
            )
        }

        // Clear winners bracket if it already exists
        self.winners_bracket = Vec::new();

        // Get the number of wild card teams and byes
        let byes = self.winners_bracket_byes()?;
        let wild_cards = self.wild_card_winners()?;
        let wild_card_matchups = wild_cards.checked_div(2).ok_or(
            String::from(
                "Failed to calculate wild card matchups for winners bracket"
            )
        )?;

        // Match up the wild card teams against one another
        let mut week = LeagueSeasonWeek::new();
        for i in 0..wild_card_matchups {
            // TODO: Eventually we should have the winners bracket byes be
            // determined by each conference champion's performance in the
            // regular season and playoffs

            // Get the conference champions by conference ID
            let home_conf_id = byes + i;
            let away_conf_id = num_teams - (i + 1);
            let home_team_id = self.conference_champion(home_conf_id)
                .ok_or_else(|| format!(
                    "No conference champion found for conference {}",
                    home_conf_id
                ))?;
            let home_team = self.teams.get(home_team_id).unwrap();
            let away_team_id = self.conference_champion(away_conf_id)
                .ok_or_else(|| format!(
                    "No conference champion found for conference {}",
                    away_conf_id
                ))?;
            let away_team = self.teams.get(away_team_id).unwrap();

            // Create the matchup and add to the week
            let matchup = LeagueSeasonMatchup::new(
                home_team_id,
                away_team_id,
                home_team.short_name(),
                away_team.short_name(),
                rng
            );
            week.matchups_mut().push(matchup);
        }

        // Add the week to the conference bracket and return
        self.winners_bracket.push(week);
        Ok(())
    }

    /// Helper method for generating a conference's first round matchups
    fn gen_conference_first_round(&mut self, conference: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough teams (at least 2)
        let num_teams = self.conference_teams(conference).len();
        if num_teams < 2 {
            return Err(
                format!(
                    "Conference {} bracket must contain at least 2 teams, got {}",
                    conference,
                    num_teams
                )
            )
        }

        if num_teams.is_power_of_two() {
            // In this case, there is no wild-card round, this is a true first round
            // Ensure the playoffs have not yet started
            if self.conference_brackets_started() {
                return Err(
                    format!(
                        "Cannot re-generate first round, conference {} playoffs already started",
                        conference
                    )
                )
            }

            // Clear conference bracket if it already exists
            self.conference_brackets.insert(conference, Vec::new());

            // Match up the first round teams against one another
            let first_round_matchups = num_teams.checked_div(2).ok_or(
                String::from("Failed to calculate first round matchups")
            )?;
            let mut week = LeagueSeasonWeek::new();
            for i in 0..first_round_matchups {
                // Get the home and away teams by seed
                let home_seed = i + 1;
                let away_seed = num_teams - i;
                let (home_team_id, home_team) = self.teams.get_by_seed(conference, home_seed)
                    .ok_or_else(|| format!(
                        "No team found in conference {} with seed {}",
                        conference,
                        home_seed
                    ))?;
                let (away_team_id, away_team) = self.teams.get_by_seed(conference, away_seed)
                    .ok_or_else(|| format!(
                        "No team found in conference {} with seed {}",
                        conference,
                        away_seed
                    ))?;

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(
                    home_team_id,
                    away_team_id,
                    home_team.short_name(),
                    away_team.short_name(),
                    rng
                );
                week.matchups_mut().push(matchup);
            }

            // Add the week to the conference bracket and return
            self.conference_brackets.entry(conference).or_default().push(week);
            Ok(())
        } else {
            // In this case, we need to determine the winners of the wild card round
            // Ensure only the wild card round exists in the conference bracket
            let rounds = self.conference_brackets.get(&conference).map(|b| b.len()).unwrap_or(0);
            if rounds > 1 {
                return Err(
                    format!(
                        "Expected only 1 round for conference {}, found {}",
                        conference,
                        rounds
                    )
                );
            }

            // Get the seeds of the wild card winners from the conference bracket
            let round = match self.conference_brackets.get(&conference).and_then(|b| b.last()) {
                Some(r) => r,
                None => return Err(String::from("Wild card round not found"))
            };
            let winner_seeds: Vec<usize> = round.matchups().iter().map(
                |x| self.team_seed(x.winner().unwrap()).unwrap()
            ).collect();
            let num_winners = winner_seeds.len();
            let byes = self.byes(Some(conference))?;

            // Populate the round with matchups
            let mut week = LeagueSeasonWeek::new();
            if num_winners >= byes {
                // Match up winners of middle-ranked matchups with byes
                for i in 0..byes {
                    let bye_seed = i + 1;
                    let winner_index = num_winners - bye_seed;
                    let winner_seed = match winner_seeds.get(winner_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(conference, bye_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            bye_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(conference, winner_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            winner_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }

                // Match up winners of higher/lower ranked matchups with each other
                let diff_winners = num_winners - byes;
                let diff_winner_matchups = diff_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate first round matchups")
                )?;
                for i in 0..diff_winner_matchups {
                    let t1_seed = match winner_seeds.get(i) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", i))
                    };
                    let t2_index = diff_winners - i + 1;
                    let t2_seed = match winner_seeds.get(diff_winners - i + 1) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t2_index))
                    };
                    // Lower seed gets home field advantage
                    let (home_seed, away_seed) = if t1_seed < t2_seed {
                        (t1_seed, t2_seed)
                    } else {
                        (t2_seed, t1_seed)
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(conference, home_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            home_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(conference, away_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            away_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
            } else {
                // Match up highest-ranked byes against winners
                for i in 0..num_winners {
                    let bye_seed = i + 1;
                    let winner_index = num_winners - bye_seed;
                    let winner_seed = match winner_seeds.get(winner_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(conference, bye_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            bye_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(conference, winner_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            winner_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }

                // Match up lowest-ranked byes against each other
                let diff_winners = byes - num_winners;
                let diff_winner_matchups = diff_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate first round matchups")
                )?;
                for i in 0..diff_winner_matchups {
                    let t1_seed = num_winners + i + 1;
                    let t2_seed = byes - i;
                    let (home_team_id, home_team) = self.teams.get_by_seed(conference, t1_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            t1_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(conference, t2_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            t2_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
            }
            self.conference_brackets.entry(conference).or_default().push(week);
            Ok(())
        }
    }

    /// Helper method for generating the winners' bracket first round matchups
    fn gen_winners_first_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure conditions are valid to generate conference wild card round
        let num_teams = self.num_conferences();
        if num_teams < 2 {
            return Err(
                format!(
                    "Must contain at least 2 conferences for a winners bracket, got {}",
                    num_teams
                )
            )
        }
        if !self.conference_brackets_complete() {
            return Err(
                String::from(
                    "Cannot generate winners bracket, conference playoffs not complete"
                )
            )
        }

        if num_teams.is_power_of_two() {
            // In this case, there is no wild-card round, this is a true first round
            // Ensure the winners bracket has not yet started
            if self.winners_bracket_started() {
                return Err(
                    String::from(
                        "Cannot re-generate first round, winners bracket already started"
                    )
                )
            }

            // Clear the winners bracket if it already exists
            self.winners_bracket = Vec::new();

            // Match up the first round teams against one another
            let first_round_matchups = num_teams.checked_div(2).ok_or(
                String::from("Failed to calculate first round matchups")
            )?;
            let mut week = LeagueSeasonWeek::new();
            for i in 0..first_round_matchups {
                // Get the home and away teams by conference ID
                let home_conf_id = i;
                let away_conf_id = num_teams - (i + 1);
                let home_team_id = self.conference_champion(home_conf_id)
                    .ok_or_else(|| format!(
                        "No conference champion found for conference {}",
                        home_conf_id
                    ))?;
                let home_team = self.teams.get(home_team_id).unwrap();
                let away_team_id = self.conference_champion(away_conf_id)
                    .ok_or_else(|| format!(
                        "No conference champion found for conference {}",
                        away_conf_id
                    ))?;
                let away_team = self.teams.get(away_team_id).unwrap();

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(
                    home_team_id,
                    away_team_id,
                    home_team.short_name(),
                    away_team.short_name(),
                    rng
                );
                week.matchups_mut().push(matchup);
            }

            // Add the week to the conference bracket and return
            self.winners_bracket.push(week);
            Ok(())
        } else {
            // In this case, we need to determine the winners of the wild card round
            // Ensure only the wild card round exists in the conference bracket
            let rounds = self.winners_bracket.len();
            if rounds > 1 {
                return Err(
                    format!(
                        "Expected only 1 round in winners bracket, found {}",
                        rounds
                    )
                );
            }

            // Get the seeds of the wild card winners from the conference bracket
            let round = match self.winners_bracket.last() {
                Some(r) => r,
                None => return Err(String::from("Wild card round not found"))
            };
            let winner_ids: Vec<usize> = round.matchups().iter().map(
                |x| x.winner().unwrap()
            ).collect();
            let num_winners = winner_ids.len();
            let byes = self.winners_bracket_byes()?;

            // Populate the round with matchups
            let mut week = LeagueSeasonWeek::new();
            if num_winners >= byes {
                // Match up winners of middle-ranked matchups with byes
                for i in 0..byes {
                    let bye_conf_id = i;
                    let winner_index = num_winners - (i + 1);
                    let away_team_id = match winner_ids.get(winner_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let home_team_id = self.conference_champion(bye_conf_id)
                        .ok_or_else(|| format!(
                            "No conference champion found for conference {}",
                            bye_conf_id
                        ))?;
                    let home_team = self.teams.get(home_team_id).unwrap();
                    let away_team = self.teams.get(away_team_id)
                        .ok_or_else(|| format!(
                            "No team found with ID {}",
                            away_team_id
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }

                // Match up winners of higher/lower ranked matchups with each other
                let diff_winners = num_winners - byes;
                let diff_winner_matchups = diff_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate first round matchups")
                )?;
                for i in 0..diff_winner_matchups {
                    let t1_id = match winner_ids.get(i) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", i))
                    };
                    let t1_seed = self.team_seed(t1_id)?;
                    let t1_conference = self.team_conference(t1_id)?;
                    let t2_index = diff_winners - (i + 1);
                    let t2_id = match winner_ids.get(t2_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t2_index))
                    };
                    let t2_seed = self.team_seed(t2_id)?;
                    let t2_conference = self.team_conference(t2_id)?;
                    // Lower seed gets home field advantage
                    let (home_seed, home_conference, away_seed, away_conference) = if t1_seed < t2_seed {
                        (t1_seed, t1_conference, t2_seed, t2_conference)
                    } else {
                        (t2_seed, t2_conference, t1_seed, t1_conference)
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(home_conference, home_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            home_conference,
                            home_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(away_conference, away_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            away_conference,
                            away_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
            } else {
                // Match up highest-ranked byes against winners
                for i in 0..num_winners {
                    let bye_conf_id = i;
                    let winner_index = num_winners - (i + 1);
                    let away_team_id = match winner_ids.get(winner_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let home_team_id = self.conference_champion(bye_conf_id)
                        .ok_or_else(|| format!(
                            "No conference champion found for conference {}",
                            bye_conf_id
                        ))?;
                    let home_team = self.teams.get(home_team_id).unwrap();
                    let away_team = self.teams.get(away_team_id)
                        .ok_or_else(|| format!(
                            "No team found with ID {}",
                            away_team_id
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }

                // Match up lowest-ranked byes against each other
                let diff_winners = byes - num_winners;
                let diff_winner_matchups = diff_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate first round matchups")
                )?;
                for i in 0..diff_winner_matchups {
                    let t1_conference = num_winners + i;
                    let t1_id = self.conference_champion(t1_conference)
                        .ok_or_else(|| format!(
                            "No conference champion found for conference {}",
                            t1_conference
                        ))?;
                    let t1_seed = self.team_seed(t1_id)?;
                    let t2_conference = byes - (i + 1);
                    let t2_id = self.conference_champion(t2_conference)
                        .ok_or_else(|| format!(
                            "No conference champion found for conference {}",
                            t2_conference
                        ))?;
                    let t2_seed = self.team_seed(t2_id)?;
                    // Lower seed gets home field advantage
                    let (home_seed, home_conference, away_seed, away_conference) = if t1_seed < t2_seed {
                        (t1_seed, t1_conference, t2_seed, t2_conference)
                    } else {
                        (t2_seed, t2_conference, t1_seed, t1_conference)
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(home_conference, home_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            home_conference,
                            home_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(away_conference, away_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            away_conference,
                            away_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
            }
            self.winners_bracket.push(week);
            Ok(())
        }
    }

    /// Helper method for generating the next round of the conference playoffs
    fn gen_next_conference_round(&mut self, conference: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough teams in the conference (at least 2)
        let num_teams = self.conference_teams(conference).len();
        if num_teams < 2 {
            return Err(format!("Playoffs must contain at least 2 teams, got {}", num_teams))
        }
        let first_round_teams = self.first_round_teams(Some(conference))?;
        let bracket_len = self.conference_brackets.get(&conference).map(|b| b.len()).unwrap_or(0);
        if bracket_len == 0 {
            // Wild card round or first round
            if first_round_teams != num_teams {
                self.gen_conference_wild_card_round(conference, rng)
            } else {
                self.gen_conference_first_round(conference, rng)
            }
        } else {
            // First round or later round
            if bracket_len == 1 && first_round_teams != num_teams {
                self.gen_conference_first_round(conference, rng)
            } else {
                // Get seeds of winners from previous round and ensure more than one
                let round = match self.conference_brackets.get(&conference).and_then(|b| b.last()) {
                    Some(r) => r,
                    None => return Err(
                        format!(
                            "Previous round not found for conference {}",
                            conference
                        )
                    )
                };
                let winner_seeds: Vec<usize> = round.matchups().iter().map(
                    |x| self.team_seed(x.winner().unwrap()).unwrap()
                ).collect();
                let num_winners = winner_seeds.len();
                if num_winners <= 1 {
                    return Err(
                        format!(
                            "Cannot generate next round for conference {}, only {} teams remain",
                            conference,
                            num_winners
                        )
                    );
                }
                let next_round_matchups = num_winners.checked_div(2).ok_or(
                    format!(
                        "Failed to calculate next round matchups for conference {}",
                        conference
                    )
                )?;

                // Match up winners of previous round against each other (using default conference 0)
                let mut week = LeagueSeasonWeek::new();
                for i in 0..next_round_matchups {
                    let t1_index = i * 2;
                    let t1_seed = match winner_seeds.get(t1_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t1_index))
                    };
                    let t2_seed = match winner_seeds.get(t1_index + 1) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t1_index + 1))
                    };

                    // Get the home and away teams (lower seed gets home field)
                    let (home_seed, away_seed) = if t1_seed < t2_seed {
                        (t1_seed, t2_seed)
                    } else {
                        (t2_seed, t1_seed)
                    };
                    let (home_team_id, home_team) = self.teams.get_by_seed(conference, home_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            home_seed
                        ))?;
                    let (away_team_id, away_team) = self.teams.get_by_seed(conference, away_seed)
                        .ok_or_else(|| format!(
                            "No team found in conference {} with seed {}",
                            conference,
                            away_seed
                        ))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
                self.conference_brackets.entry(conference).or_default().push(week);
                Ok(())
            }
        }
    }

    /// Helper method for generating the next round for all conference brackets
    fn gen_next_conference_rounds(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        let conference_ids: Vec<usize> = self.teams.conferences().copied().collect();
        for conference in conference_ids {
            self.gen_next_conference_round(conference, rng)?;
        }
        Ok(())
    }

    /// Helper method for generating the next round of the winners bracket
    fn gen_next_winners_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough conference champions
        let num_teams = self.teams.num_conferences();
        if num_teams < 2 {
            return Err(
                format!(
                    "Winners bracket must contain at least 2 conference champions, got {}",
                    num_teams
                )
            )
        }

        // Ensure the conference playoffs are complete
        if !self.conference_brackets_complete() {
            return Err(
                String::from(
                    "Cannot generate winners bracket: Conference playoffs not complete"
                )
            )
        }

        let first_round_teams = self.first_round_winners()?;
        let bracket_len = self.winners_bracket.len();
        if bracket_len == 0 {
            // Wild card round or first round
            if first_round_teams != num_teams {
                self.gen_winners_wild_card_round(rng)
            } else {
                self.gen_winners_first_round(rng)
            }
        } else {
            // First round or later round
            if bracket_len == 1 && first_round_teams != num_teams {
                self.gen_winners_first_round(rng)
            } else {
                // Get seeds of winners from previous round and ensure more than one
                let round = match self.winners_bracket.last() {
                    Some(r) => r,
                    None => return Err(
                        String::from(
                            "Previous winners bracket round not found"
                        )
                    )
                };
                let winner_ids: Vec<usize> = round.matchups().iter().map(
                    |x| x.winner().unwrap()
                ).collect();
                let num_winners = winner_ids.len();
                if num_winners <= 1 {
                    return Err(
                        format!(
                            "Cannot generate next round for winners bracket, only {} teams remain",
                            num_winners
                        )
                    );
                }
                let next_round_matchups = num_winners.checked_div(2).ok_or(
                    String::from(
                        "Failed to calculate next round matchups for winners bracket"
                    )
                )?;

                // Match up winners of previous round against each other
                let mut week = LeagueSeasonWeek::new();
                for i in 0..next_round_matchups {
                    let t1_index = i * 2;
                    let t1_id = match winner_ids.get(t1_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t1_index))
                    };
                    let t1_seed = self.team_seed(t1_id)?;
                    let t2_index = t1_index + 1;
                    let t2_id = match winner_ids.get(t2_index) {
                        Some(s) => *s,
                        None => return Err(format!("No winner found at index {}", t2_index))
                    };
                    let t2_seed = self.team_seed(t2_id)?;
                    // Get the home and away teams (lower seed gets home field)
                    let (home_team_id, away_team_id) = if t1_seed < t2_seed {
                        (t1_id, t2_id)
                    } else {
                        (t2_id, t1_id)
                    };
                    let home_team = self.teams.get(home_team_id).unwrap();
                    let away_team = self.teams.get(away_team_id).unwrap();

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team_id,
                        away_team_id,
                        home_team.short_name(),
                        away_team.short_name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
                self.winners_bracket.push(week);
                Ok(())
            }
        }
    }

    /// Generate the next round of the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "A", Some(0));
    /// let _ = my_playoffs.add_team(1, "B", Some(0));
    /// let _ = my_playoffs.add_team(2, "C", Some(1));
    /// let _ = my_playoffs.add_team(3, "D", Some(1));
    /// let _ = my_playoffs.add_team(4, "E", Some(2));
    /// let _ = my_playoffs.add_team(5, "F", Some(2));
    ///
    /// // Get the number of winners bracket first round byes
    /// let mut rng = rand::thread_rng();
    /// let res = my_playoffs.gen_next_playoff_round(&mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn gen_next_playoff_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure playoffs are not already complete
        if self.complete() {
            return Err(
                String::from(
                    "Cannot generate next playoff round: Playoffs complete"
                )
            )
        }

        // Generate the next round of the playoffs
        if self.is_conference_playoff() {
            if self.conference_brackets_complete() {
                self.gen_next_winners_round(rng)
            } else {
                self.gen_next_conference_rounds(rng)
            }
        } else {
            self.gen_next_conference_rounds(rng)
        }
    }

    /// Gets the championship matchup if it exists
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Create playoffs and add a team
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// let championship = my_playoffs.championship();
    /// assert!(championship.is_none());
    /// ```
    pub fn championship(&self) -> Option<&LeagueSeasonMatchup> {
        if self.is_conference_playoff() {
            let conferences = self.teams.num_conferences();
            if !self.winners_bracket_started() || 
                self.winners_bracket.is_empty() ||
                (
                    self.winners_bracket.len() == 1 &&
                    conferences > 2
                ) {
                return None;
            }
            if let Some(final_round) = self.winners_bracket.last() {
                if final_round.matchups().len() != 1 {
                    return None;
                }
                return final_round.matchups().first();
            }
            None
        } else {
            let bracket = self.conference_brackets.get(&0)?;
            if bracket.is_empty() || (bracket.len() == 1 && self.teams.len() > 2) {
                return None;
            }
            if let Some(final_round) = bracket.last() {
                if final_round.matchups().len() != 1 {
                    return None;
                }
                return final_round.matchups().first();
            }
            None
        }
    }

    /// Check if a team made it to the championship
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Create playoffs and add a team
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    ///
    /// // Check if that team is in the championship
    /// let in_championship = my_playoffs.in_championship(0);
    /// assert!(in_championship.is_ok());
    /// assert!(!in_championship.unwrap());
    /// ```
    pub fn in_championship(&self, team_id: usize) -> Result<bool, String> {
        // Ensure the team ID exists in the playoffs
        if !self.team_in_playoffs(team_id) {
            return Err(format!("Team {} not in playoffs", team_id));
        }

        // Check if the team appeared in the championship matchup
        match self.championship() {
            Some(m) => Ok(*m.home_team() == team_id || *m.away_team() == team_id),
            None => Ok(false)
        }
    }

    /// Get the champion team ID if the playoffs are complete
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(my_playoffs.champion().is_none());
    /// ```
    pub fn champion(&self) -> Option<usize> {
        match self.championship() {
            Some(m) => m.winner(),
            None => None
        }
    }

    /// Compute a team's playoff record
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    ///
    /// // Create playoffs and add a team
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", None);
    ///
    /// // Get that team's record
    /// let record = my_playoffs.record(0);
    /// assert!(record.is_ok());
    /// assert!(record.unwrap() == LeagueTeamRecord::new());
    /// ```
    pub fn record(&self, team_id: usize) -> Result<LeagueTeamRecord, String> {
        // Ensure the team ID exists in the playoffs
        if !self.team_in_playoffs(team_id) {
            return Err(format!("Team {} not in playoffs", team_id));
        }
        let mut record = LeagueTeamRecord::new();

        // Calculate the team's playoff record across all brackets
        let all_rounds = self.conference_brackets.values().flatten()
            .chain(self.winners_bracket.iter());

        for round in all_rounds {
            for matchup in round.matchups().iter() {
                // Check if this team participated in the matchup
                if *matchup.home_team() != team_id && *matchup.away_team() != team_id {
                    continue;
                }

                // Get the result for this team
                if let Some(result) = matchup.result(team_id) {
                    match result {
                        FootballMatchupResult::Win => record.increment_wins(1),
                        FootballMatchupResult::Loss => record.increment_losses(1),
                        FootballMatchupResult::Tie => record.increment_ties(1),
                    }
                }
            }
        }
        Ok(record)
    }
}
