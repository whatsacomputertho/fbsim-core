pub mod picture;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

use crate::game::matchup::FootballMatchupResult;
use crate::league::matchup::LeagueTeamRecord;
use crate::league::season::week::LeagueSeasonWeek;
use crate::league::season::matchup::LeagueSeasonMatchup;

/// # `PlayoffTeam` struct
///
/// Represents a team participating in the playoffs
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct PlayoffTeam {
    id: usize,
    name: String,
    seed: usize,
    conference: Option<usize>,
}

impl PlayoffTeam {
    /// Create a new PlayoffTeam
    pub fn new(id: usize, name: String, seed: usize, conference: Option<usize>) -> Self {
        Self { id, name, seed, conference }
    }

    /// Get the team ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the team name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the team's playoff seed
    pub fn seed(&self) -> usize {
        self.seed
    }

    /// Get the team's conference (if conference-based playoffs)
    pub fn conference(&self) -> Option<usize> {
        self.conference
    }
}

/// # `PlayoffTeams` struct
///
/// A collection of teams participating in the playoffs, organized by conference.
/// Conference ID 0 is used for non-conference playoffs.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct PlayoffTeams {
    /// conference_id -> team_id -> PlayoffTeam
    inner: BTreeMap<usize, BTreeMap<usize, PlayoffTeam>>,
}

impl PlayoffTeams {
    /// Create a new empty PlayoffTeams collection
    pub fn new() -> Self {
        Self { inner: BTreeMap::new() }
    }

    /// Add a team to the playoffs
    ///
    /// Teams are expected to be added in seed order within each conference.
    /// The seed is automatically calculated based on the number of teams already in that conference.
    pub fn add(&mut self, team_id: usize, name: &str, conference: usize) -> Result<(), String> {
        let conference_teams = self.inner.entry(conference).or_default();

        if conference_teams.contains_key(&team_id) {
            return Err(format!("Team {} is already in conference {}", team_id, conference));
        }

        let seed = conference_teams.len() + 1;
        let playoff_team = PlayoffTeam::new(team_id, String::from(name), seed, Some(conference));
        conference_teams.insert(team_id, playoff_team);
        Ok(())
    }

    /// Get a team by ID (searches all conferences)
    pub fn get(&self, team_id: usize) -> Option<&PlayoffTeam> {
        for conference_teams in self.inner.values() {
            if let Some(team) = conference_teams.get(&team_id) {
                return Some(team);
            }
        }
        None
    }

    /// Get all teams in a specific conference
    pub fn get_conference(&self, conference: usize) -> Option<&BTreeMap<usize, PlayoffTeam>> {
        self.inner.get(&conference)
    }

    /// Check if a team is in the playoffs
    pub fn contains(&self, team_id: usize) -> bool {
        self.get(team_id).is_some()
    }

    /// Get the total number of teams across all conferences
    pub fn len(&self) -> usize {
        self.inner.values().map(|c| c.len()).sum()
    }

    /// Check if there are no teams
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty() || self.inner.values().all(|c| c.is_empty())
    }

    /// Get the number of conferences
    pub fn num_conferences(&self) -> usize {
        self.inner.len()
    }

    /// Iterate over conference IDs
    pub fn conferences(&self) -> impl Iterator<Item = &usize> {
        self.inner.keys()
    }

    /// Iterate over all teams across all conferences
    pub fn iter(&self) -> impl Iterator<Item = &PlayoffTeam> {
        self.inner.values().flat_map(|c| c.values())
    }

    /// Get teams in a conference sorted by seed
    pub fn conference_teams_by_seed(&self, conference: usize) -> Vec<&PlayoffTeam> {
        if let Some(conference_teams) = self.inner.get(&conference) {
            let mut teams: Vec<&PlayoffTeam> = conference_teams.values().collect();
            teams.sort_by_key(|t| t.seed());
            teams
        } else {
            Vec::new()
        }
    }

    /// Get a team by seed within a specific conference
    pub fn get_by_seed(&self, conference: usize, seed: usize) -> Option<&PlayoffTeam> {
        self.inner
            .get(&conference)
            .and_then(|conf_teams| conf_teams.values().find(|t| t.seed() == seed))
    }
}

/// # `LeagueSeasonPlayoffs` struct
///
/// A `LeagueSeasonPlayoffs` represents football season playoffs
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonPlayoffs {
    /// Teams participating in the playoffs
    teams: PlayoffTeams,
    /// Playoff rounds (used for non-conference playoffs)
    rounds: Vec<LeagueSeasonWeek>,
    /// Conference playoff rounds: conference_index -> playoff rounds for that conference
    #[serde(default)]
    conference_rounds: BTreeMap<usize, Vec<LeagueSeasonWeek>>,
    /// Winners bracket for championship game(s) between conference champions
    /// Contains the championship game for 2-conference setup, or a tournament for >2 conferences
    #[serde(default)]
    winners_bracket: Vec<LeagueSeasonWeek>,
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
        // If no rounds generated yet, they haven't started
        if self.rounds.is_empty() {
            return false;
        }

        // If any rounds have started, the playoffs have started
        for round in self.rounds.iter() {
            if round.started() {
                return true;
            }
        }

        // If no rounds have started, the playoffs have not started
        false
    }

    /// Borrow the playoff rounds
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// let rounds = my_playoffs.rounds();
    /// ```
    pub fn rounds(&self) -> &Vec<LeagueSeasonWeek> {
        &self.rounds
    }

    /// Mutably borrow the playoff rounds
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let rounds = my_playoffs.rounds_mut();
    /// ```
    pub fn rounds_mut(&mut self) -> &mut Vec<LeagueSeasonWeek> {
        &mut self.rounds
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
        // If no rounds generated yet, they haven't started
        if self.rounds.is_empty() {
            return false;
        }

        // If any round is not yet complete, the playoffs are not complete
        for round in self.rounds.iter() {
            if !round.complete() {
                return false;
            }
        }

        // Count how many teams have been eliminated
        let mut eliminated = 0;
        for round in self.rounds.iter() {
            eliminated += round.matchups().len();
        }

        // Playoffs are complete when only 1 team remains
        self.teams.len() == eliminated + 1
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

    /// Get the number of teams that will appear in the first round
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
    /// let first_round_teams = my_playoffs.first_round_teams();
    /// assert!(first_round_teams.is_ok());
    /// assert!(first_round_teams.unwrap() == 2);
    /// ```
    pub fn first_round_teams(&self) -> Result<usize, String> {
        // Ensure there are enough teams (at least 2)
        let num_teams = self.teams.len();
        if num_teams < 2 {
            return Err(format!("Playoffs must contain at least 2 teams, got {}", num_teams))
        }

        // Calculate the number of first round matchups
        let next_power = num_teams.checked_next_power_of_two().ok_or(
            String::from("Failed to calculate first round matchups")
        )?;
        if next_power == num_teams {
            Ok(next_power)
        } else {
            next_power.checked_div(2).ok_or(
                String::from("Failed to calculate first round matchups")
            )
        }
    }

    /// Get the number of teams that will appear in the wild card round
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
    /// let wild_cards = my_playoffs.wild_cards();
    /// assert!(wild_cards.is_ok());
    /// assert!(wild_cards.unwrap() == 2);
    /// ```
    pub fn wild_cards(&self) -> Result<usize, String> {
        let num_teams = self.teams.len();
        let first_round_teams = self.first_round_teams()?;
        let k = num_teams.checked_sub(first_round_teams).ok_or(
            String::from("Failed to calculate wild cards")
        )?;
        Ok(2 * k)
    }

    /// Get the number of teams that will have a bye
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
    /// let byes = my_playoffs.byes();
    /// assert!(byes.is_ok());
    /// assert!(byes.unwrap() == 1);
    /// ```
    pub fn byes(&self) -> Result<usize, String> {
        let num_teams = self.teams.len();
        let first_round_teams = self.first_round_teams()?;
        let k = num_teams.checked_sub(first_round_teams).ok_or(
            String::from("Failed to calculate byes")
        )?;
        let byes = num_teams.checked_sub(2 * k).ok_or(
            String::from("Failed to calculate byes")
        )?;
        Ok(byes)
    }

    // Helper function for generating the wild card matchups
    fn gen_wild_card_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough teams (at least 2)
        let num_teams = self.teams.len();
        if num_teams < 2 {
            return Err(format!("Playoffs must contain at least 2 teams, got {}", num_teams))
        }

        // Ensure the number of teams is not a power of 2
        if num_teams.is_power_of_two() {
            return Err(format!("No wild card round for playoffs with a power of 2 teams: {}", num_teams))
        }

        // Ensure the playoffs have not yet started
        if self.started() {
            return Err(String::from("Cannot re-generate wild card round, playoffs already started"))
        }

        // If rounds exist already, clear them
        if !self.rounds.is_empty() {
            self.rounds = Vec::new();
        }

        // Get the number of wild card teams and byes
        let byes = self.byes()?;
        let wild_cards = self.wild_cards()?;
        let wild_card_matchups = wild_cards.checked_div(2).ok_or(
            String::from("Failed to calculate wild card matchups")
        )?;

        // Match up the wild card teams against one another (using default conference 0)
        let mut week = LeagueSeasonWeek::new();
        for i in 0..wild_card_matchups {
            // Get the home and away teams by seed
            let home_seed = byes + i + 1;
            let away_seed = num_teams - i;
            let home_team = self.teams.get_by_seed(0, home_seed)
                .ok_or_else(|| format!("No team found with seed {}", home_seed))?;
            let away_team = self.teams.get_by_seed(0, away_seed)
                .ok_or_else(|| format!("No team found with seed {}", away_seed))?;

            // Create the matchup and add to the week
            let matchup = LeagueSeasonMatchup::new(
                home_team.id(),
                away_team.id(),
                home_team.name(),
                away_team.name(),
                rng
            );
            week.matchups_mut().push(matchup);
        }

        // Add the week to the rounds and return
        self.rounds.push(week);
        Ok(())
    }

    // Helper function for generating the first round matchups
    fn gen_first_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough teams (at least 2)
        let num_teams = self.teams.len();
        if num_teams < 2 {
            return Err(format!("Playoffs must contain at least 2 teams, got {}", num_teams))
        }

        if num_teams.is_power_of_two() {
            // In this case, there is no wild-card round, this is a true first round
            // Ensure the playoffs have not yet started
            if self.started() {
                return Err(String::from("Cannot re-generate first round, playoffs already started"))
            }

            // If rounds exist already, clear them
            if !self.rounds.is_empty() {
                self.rounds = Vec::new();
            }

            // Match up the first round teams against one another (using default conference 0)
            let first_round_matchups = num_teams.checked_div(2).ok_or(
                String::from("Failed to calculate first round matchups")
            )?;
            let mut week = LeagueSeasonWeek::new();
            for i in 0..first_round_matchups {
                // Get the home and away teams by seed
                let home_seed = i + 1;
                let away_seed = num_teams - i;
                let home_team = self.teams.get_by_seed(0, home_seed)
                    .ok_or_else(|| format!("No team found with seed {}", home_seed))?;
                let away_team = self.teams.get_by_seed(0, away_seed)
                    .ok_or_else(|| format!("No team found with seed {}", away_seed))?;

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(
                    home_team.id(),
                    away_team.id(),
                    home_team.name(),
                    away_team.name(),
                    rng
                );
                week.matchups_mut().push(matchup);
            }

            // Add the week to the rounds and return
            self.rounds.push(week);
            Ok(())
        } else {
            // In this case, we need to determine the winners of the wild card round
            // Ensure only the wild card round exists
            let rounds = self.rounds.len();
            if rounds > 1 {
                return Err(format!("Expected only 1 round, found {}", rounds));
            }

            // Get the wild card round
            let round = match self.rounds.last() {
                Some(r) => r,
                None => return Err(String::from("Wild card round not found"))
            };

            // Get the seed of each wild card matchup winner and number of byes
            let winner_seeds: Vec<usize> = round.matchups().iter().map(
                |x| self.team_seed(x.winner().unwrap()).unwrap()
            ).collect();
            let num_winners = winner_seeds.len();
            let byes = self.byes()?;

            // Populate the week with matchups (using default conference 0)
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
                    let home_team = self.teams.get_by_seed(0, bye_seed)
                        .ok_or_else(|| format!("No team found with seed {}", bye_seed))?;
                    let away_team = self.teams.get_by_seed(0, winner_seed)
                        .ok_or_else(|| format!("No team found with seed {}", winner_seed))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team.id(),
                        away_team.id(),
                        home_team.name(),
                        away_team.name(),
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
                    let home_team = self.teams.get_by_seed(0, home_seed)
                        .ok_or_else(|| format!("No team found with seed {}", home_seed))?;
                    let away_team = self.teams.get_by_seed(0, away_seed)
                        .ok_or_else(|| format!("No team found with seed {}", away_seed))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team.id(),
                        away_team.id(),
                        home_team.name(),
                        away_team.name(),
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
                    let home_team = self.teams.get_by_seed(0, bye_seed)
                        .ok_or_else(|| format!("No team found with seed {}", bye_seed))?;
                    let away_team = self.teams.get_by_seed(0, winner_seed)
                        .ok_or_else(|| format!("No team found with seed {}", winner_seed))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team.id(),
                        away_team.id(),
                        home_team.name(),
                        away_team.name(),
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
                    let home_team = self.teams.get_by_seed(0, t1_seed)
                        .ok_or_else(|| format!("No team found with seed {}", t1_seed))?;
                    let away_team = self.teams.get_by_seed(0, t2_seed)
                        .ok_or_else(|| format!("No team found with seed {}", t2_seed))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team.id(),
                        away_team.id(),
                        home_team.name(),
                        away_team.name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
            }
            self.rounds.push(week);
            Ok(())
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

        // Need at least one round for a championship
        if self.rounds.is_empty() {
            return Ok(false);
        }

        // Not championship if more than 2 teams but only 1 round
        if self.rounds.len() == 1 && self.teams.len() > 2 {
            return Ok(false);
        }

        // Get the final round
        if let Some(final_round) = self.rounds.last() {
            // Championship is the round with exactly 1 matchup
            if final_round.matchups().len() == 1 {
                if let Some(final_matchup) = final_round.matchups().first() {
                    let team_in_championship = *final_matchup.home_team() == team_id ||
                           *final_matchup.away_team() == team_id;
                    return Ok(team_in_championship);
                }
            }
        }
        Ok(false)
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
        // If playoffs are not complete, there is no champion
        if !self.complete() {
            return None;
        }

        // Get the final round's single matchup and return the winner
        if let Some(final_round) = self.rounds.last() {
            if let Some(final_matchup) = final_round.matchups().first() {
                return final_matchup.winner();
            }
        }
        None
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

        // Calculate the team's playoff record
        for round in self.rounds.iter() {
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

    /// Generate the next round of the playoffs
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
    /// // Generate the next round of the playoffs
    /// let mut rng = rand::thread_rng();
    /// let res = my_playoffs.gen_next_round(&mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn gen_next_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Ensure there are enough teams (at least 2)
        let num_teams = self.teams.len();
        if num_teams < 2 {
            return Err(format!("Playoffs must contain at least 2 teams, got {}", num_teams))
        }

        let first_round_teams = self.first_round_teams()?;
        if self.rounds.is_empty() {
            // Wild card round or first round
            if first_round_teams != num_teams {
                self.gen_wild_card_round(rng)
            } else {
                self.gen_first_round(rng)
            }
        } else {
            // First round or later round
            if self.rounds.len() == 1 && first_round_teams != num_teams {
                self.gen_first_round(rng)
            } else {
                // Get seeds of winners from previous round and ensure more than one
                let round = match self.rounds.last() {
                    Some(r) => r,
                    None => return Err(String::from("Previous round not found"))
                };
                let winner_seeds: Vec<usize> = round.matchups().iter().map(
                    |x| self.team_seed(x.winner().unwrap()).unwrap()
                ).collect();
                let num_winners = winner_seeds.len();
                if num_winners <= 1 {
                    return Err(format!("Cannot generate next round, only {} teams remain", num_winners));
                }
                let next_round_matchups = num_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate next round matchups")
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
                    let home_team = self.teams.get_by_seed(0, home_seed)
                        .ok_or_else(|| format!("No team found with seed {}", home_seed))?;
                    let away_team = self.teams.get_by_seed(0, away_seed)
                        .ok_or_else(|| format!("No team found with seed {}", away_seed))?;

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(
                        home_team.id(),
                        away_team.id(),
                        home_team.name(),
                        away_team.name(),
                        rng
                    );
                    week.matchups_mut().push(matchup);
                }
                self.rounds.push(week);
                Ok(())
            }
        }
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

    /// Get teams in a specific conference, sorted by seed
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
    pub fn conference_teams(&self, conf_index: usize) -> Vec<&PlayoffTeam> {
        self.teams.conference_teams_by_seed(conf_index)
    }

    /// Generate the next round for conference-based playoffs
    ///
    /// This handles separate conference brackets until the championship round,
    /// then merges conference winners for the final game.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME", Some(0));
    /// let _ = my_playoffs.add_team(1, "YOU", Some(0));
    /// let _ = my_playoffs.add_team(2, "THEM", Some(1));
    /// let _ = my_playoffs.add_team(3, "US", Some(1));
    ///
    /// let mut rng = rand::thread_rng();
    /// let res = my_playoffs.gen_next_conference_round(&mut rng);
    /// assert!(res.is_ok());
    /// ```
    pub fn gen_next_conference_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        if !self.is_conference_playoff() {
            return Err("Not a conference-based playoff. Use gen_next_round instead.".to_string());
        }

        // Ensure we have at least 2 conferences
        if self.teams.num_conferences() < 2 {
            return Err("Conference playoffs require at least 2 conferences".to_string());
        }

        // Check if this is the championship round (one team remaining per conference)
        let conference_winners = self.get_conference_winners();
        if conference_winners.len() == self.teams.num_conferences()
            && conference_winners.iter().all(|(_, winner)| winner.is_some())
        {
            return self.gen_championship_round(rng);
        }

        // Collect conference indices first to avoid borrow issues
        let conf_indices: Vec<usize> = self.teams.conferences().copied().collect();

        // Generate next round for each conference separately
        let mut week = LeagueSeasonWeek::new();

        for conf_index in conf_indices {
            // Determine which teams are still active in this conference
            let active_teams = self.get_active_conference_teams(conf_index)?;

            if active_teams.len() <= 1 {
                // Conference bracket is complete
                continue;
            }

            // Generate matchups for this conference
            let matchups = self.generate_conference_matchups(&active_teams, rng)?;
            for matchup in matchups {
                week.matchups_mut().push(matchup);
            }
        }

        if week.matchups().is_empty() {
            return Err("No matchups to generate".to_string());
        }

        self.conference_rounds
            .entry(0) // TODO: Track rounds per conference properly
            .or_default()
            .push(week);
        Ok(())
    }

    /// Get teams still active in a conference bracket
    fn get_active_conference_teams(&self, conf_index: usize) -> Result<Vec<&PlayoffTeam>, String> {
        let conference_teams = self.teams.conference_teams_by_seed(conf_index);
        if conference_teams.is_empty() {
            return Err(format!("Conference {} not found", conf_index));
        }

        // Check all rounds (both regular and conference rounds) for eliminated teams
        let all_rounds: Vec<&LeagueSeasonWeek> = self.rounds.iter()
            .chain(self.conference_rounds.values().flatten())
            .collect();

        if all_rounds.is_empty() {
            // No rounds played yet, all teams active
            return Ok(conference_teams);
        }

        // Find teams that have lost in previous rounds
        let mut eliminated: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for round in all_rounds {
            for matchup in round.matchups() {
                if matchup.context().game_over() {
                    if let Some(winner_id) = matchup.winner() {
                        // The loser is eliminated
                        let loser_id = if *matchup.home_team() == winner_id {
                            *matchup.away_team()
                        } else {
                            *matchup.home_team()
                        };
                        eliminated.insert(loser_id);
                    }
                }
            }
        }

        // Return active teams (not eliminated)
        let active: Vec<&PlayoffTeam> = conference_teams
            .into_iter()
            .filter(|team| !eliminated.contains(&team.id()))
            .collect();
        Ok(active)
    }

    /// Generate matchups for a conference round
    fn generate_conference_matchups(
        &self,
        active_teams: &[&PlayoffTeam],
        rng: &mut impl Rng,
    ) -> Result<Vec<LeagueSeasonMatchup>, String> {
        let num_teams = active_teams.len();
        if num_teams < 2 {
            return Err("Not enough teams for matchups".to_string());
        }

        let num_matchups = num_teams / 2;
        let mut matchups = Vec::new();

        // Sort by seed
        let mut sorted_teams: Vec<&PlayoffTeam> = active_teams.to_vec();
        sorted_teams.sort_by_key(|t| t.seed());

        // Pair highest seed with lowest seed
        for i in 0..num_matchups {
            let home_team = sorted_teams[i];
            let away_team = sorted_teams[num_teams - 1 - i];

            // Lower seed gets home field
            let (final_home, final_away) = if home_team.seed() < away_team.seed() {
                (home_team, away_team)
            } else {
                (away_team, home_team)
            };

            let matchup = LeagueSeasonMatchup::new(
                final_home.id(),
                final_away.id(),
                final_home.name(),
                final_away.name(),
                rng,
            );
            matchups.push(matchup);
        }
        Ok(matchups)
    }

    /// Get the conference winners (if determined)
    fn get_conference_winners(&self) -> Vec<(usize, Option<&PlayoffTeam>)> {
        let mut winners = Vec::new();
        for conf_index in self.teams.conferences() {
            match self.get_active_conference_teams(*conf_index) {
                Ok(active) if active.len() == 1 => {
                    winners.push((*conf_index, Some(active[0])));
                }
                _ => {
                    winners.push((*conf_index, None));
                }
            }
        }
        winners
    }

    /// Generate the championship round (conference winners face off)
    fn gen_championship_round(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        let winners = self.get_conference_winners();

        // Collect actual winners
        let mut final_teams: Vec<(usize, usize, String)> = Vec::new();
        for (conf_index, winner) in winners {
            if let Some(team) = winner {
                final_teams.push((conf_index, team.id(), team.name().to_string()));
            }
        }

        if final_teams.len() < 2 {
            return Err(format!(
                "Championship requires at least 2 conference winners, got {}",
                final_teams.len()
            ));
        }

        // For 2 conferences, create a single championship game
        // For >2 conferences, create a winners bracket tournament
        let mut week = LeagueSeasonWeek::new();

        if final_teams.len() == 2 {
            // Standard 2-conference championship
            final_teams.sort_by_key(|(conf_index, _, _)| *conf_index);
            let (_, home_id, home_name) = &final_teams[0];
            let (_, away_id, away_name) = &final_teams[1];

            let matchup = LeagueSeasonMatchup::new(
                *home_id,
                *away_id,
                home_name,
                away_name,
                rng,
            );
            week.matchups_mut().push(matchup);
            self.winners_bracket.push(week);
        } else {
            // Multi-conference tournament: pair up winners
            final_teams.sort_by_key(|(conf_index, _, _)| *conf_index);
            let num_matchups = final_teams.len() / 2;
            for i in 0..num_matchups {
                let (_, home_id, home_name) = &final_teams[i];
                let (_, away_id, away_name) = &final_teams[final_teams.len() - 1 - i];

                let matchup = LeagueSeasonMatchup::new(
                    *home_id,
                    *away_id,
                    home_name,
                    away_name,
                    rng,
                );
                week.matchups_mut().push(matchup);
            }
            self.winners_bracket.push(week);
        }
        Ok(())
    }

    /// Get the conference champion for a specific conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let my_playoffs = LeagueSeasonPlayoffs::new();
    /// assert!(my_playoffs.conference_champion(0).is_none());
    /// ```
    pub fn conference_champion(&self, conf_index: usize) -> Option<usize> {
        let active = self.get_active_conference_teams(conf_index).ok()?;
        if active.len() == 1 {
            Some(active[0].id())
        } else {
            None
        }
    }

    /// Borrow the conference rounds
    pub fn conference_rounds(&self) -> &BTreeMap<usize, Vec<LeagueSeasonWeek>> {
        &self.conference_rounds
    }

    /// Borrow the winners bracket
    pub fn winners_bracket(&self) -> &Vec<LeagueSeasonWeek> {
        &self.winners_bracket
    }
}
