#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

use crate::league::season::week::LeagueSeasonWeek;
use crate::league::season::matchup::LeagueSeasonMatchup;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonPlayoffs {
    teams: BTreeMap<usize, (usize, String)>,
    rounds: Vec<LeagueSeasonWeek>
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

        // If the last round contains one matchup, the playoffs are complete
        if let Some(last_round) = self.rounds.last() {
            if last_round.matchups().len() == 1 {
                return true;
            }
        }
        false
    }

    /// Add a team to the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let res = my_playoffs.add_team(0, "ME");
    /// assert!(res.is_ok());
    /// ```
    pub fn add_team(&mut self, team: usize, name: &str) -> Result<(), String> {
        // Ensure the playoffs have not already started
        if self.started() {
            return Err(String::from("Playoffs have already started, cannot add new team"));
        }
        
        // Calculate the next seed and add the team
        let seed = self.teams.len() + 1;
        self.teams.insert(seed, (team, String::from(name)));
        Ok(())
    }

    /// Get the number of teams that will appear in the first round
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME");
    /// let _ = my_playoffs.add_team(1, "YOU");
    /// let _ = my_playoffs.add_team(2, "THEM");
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
    /// let _ = my_playoffs.add_team(0, "ME");
    /// let _ = my_playoffs.add_team(1, "YOU");
    /// let _ = my_playoffs.add_team(2, "THEM");
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
    /// let _ = my_playoffs.add_team(0, "ME");
    /// let _ = my_playoffs.add_team(1, "YOU");
    /// let _ = my_playoffs.add_team(2, "THEM");
    ///
    /// // Get the number of wild card teams
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

        // Match up the wild card teams against one another
        let mut week = LeagueSeasonWeek::new();
        for i in 0..wild_card_matchups {
            // Get the home and away IDs
            let home_seed = byes + i;
            let away_seed = num_teams - i;
            let (home_id, home_name) = match self.teams.get(&home_seed) {
                Some(t) => t,
                None => return Err(format!("No team found with seed {}", home_seed))
            };
            let (away_id, away_name) = match self.teams.get(&away_seed) {
                Some(t) => t,
                None => return Err(format!("No team found with seed {}", away_seed))
            };

            // Create the matchup and add to the week
            let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
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

            // Match up the first round teams against one another
            let first_round_matchups = num_teams.checked_div(2).ok_or(
                String::from("Failed to calculate first round matchups")
            )?;
            let mut week = LeagueSeasonWeek::new();
            for i in 0..first_round_matchups {
                // Get the home and away IDs
                let home_seed = i + 1;
                let away_seed = num_teams - i;
                let (home_id, home_name) = match self.teams.get(&home_seed) {
                    Some(t) => t,
                    None => return Err(format!("No team found with seed {}", home_seed))
                };
                let (away_id, away_name) = match self.teams.get(&away_seed) {
                    Some(t) => t,
                    None => return Err(format!("No team found with seed {}", away_seed))
                };

                // Create the matchup and add to the week
                let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
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

            // Get the winner of each wild card matchup and number of byes
            let winners: Vec<usize> = round.matchups().iter().map(
                |x| x.winner().unwrap()
            ).collect();
            let num_winners = winners.len();
            let byes = self.byes()?;

            // Populate the week with matchups
            let mut week = LeagueSeasonWeek::new();
            if num_winners >= byes {
                // Match up winners of middle-ranked matchups with byes
                for i in 0..byes {
                    let bye_seed = i + 1;
                    let winner_index = num_winners - bye_seed;
                    let winner_seed = match winners.get(winner_index) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let (home_id, home_name) = match self.teams.get(&bye_seed) {
                        Some(t) => t,
                        None => return Err(format!("No team found with seed {}", bye_seed))
                    };
                    let (away_id, away_name) = match self.teams.get(winner_seed) {
                        Some(t) => t,
                        None => return Err(format!("No team found with seed {}", winner_seed))
                    };

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
                    week.matchups_mut().push(matchup);
                }

                // Match up winners of higher/lower ranked matchups with each other
                let diff_winners = num_winners - byes;
                let diff_winner_matchups = diff_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate first round matchups")
                )?;
                for i in 0..diff_winner_matchups {
                    let t1_seed = match winners.get(i) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", i))
                    };
                    let t2_index = diff_winners - i + 1;
                    let t2_seed = match winners.get(diff_winners - i + 1) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", t2_index))
                    };
                    let (home_id, home_name) = if t1_seed > t2_seed {
                        match self.teams.get(t1_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t1_seed))
                        }
                    } else {
                        match self.teams.get(t2_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t2_seed))
                        }
                    };
                    let (away_id, away_name) = if t1_seed > t2_seed {
                        match self.teams.get(t2_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t2_seed))
                        }
                    } else {
                        match self.teams.get(t1_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t1_seed))
                        }
                    };

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
                    week.matchups_mut().push(matchup);
                }
            } else {
                // Match up highest-ranked byes against winners
                for i in 0..num_winners {
                    let bye_seed = i + 1;
                    let winner_index = num_winners - bye_seed;
                    let winner_seed = match winners.get(winner_index) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", winner_index))
                    };
                    let (home_id, home_name) = match self.teams.get(&bye_seed) {
                        Some(t) => t,
                        None => return Err(format!("No team found with seed {}", bye_seed))
                    };
                    let (away_id, away_name) = match self.teams.get(winner_seed) {
                        Some(t) => t,
                        None => return Err(format!("No team found with seed {}", winner_seed))
                    };

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
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
                    let (home_id, home_name) = match self.teams.get(&t1_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t1_seed))
                        };
                    let (away_id, away_name) = match self.teams.get(&t2_seed) {
                        Some(t) => t,
                        None => return Err(format!("No team found with seed {}", t2_seed))
                    };

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
                    week.matchups_mut().push(matchup);
                }
            }
            self.rounds.push(week);
            Ok(())
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

    /// Generate the next round of the playoffs
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs;
    ///
    /// // Instantiate playoffs and add teams
    /// let mut my_playoffs = LeagueSeasonPlayoffs::new();
    /// let _ = my_playoffs.add_team(0, "ME");
    /// let _ = my_playoffs.add_team(1, "YOU");
    /// let _ = my_playoffs.add_team(2, "THEM");
    ///
    /// // Get the number of wild card teams
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
                // Get winners of previous round and ensure more than one
                let round = match self.rounds.last() {
                    Some(r) => r,
                    None => return Err(String::from("Wild card round not found"))
                };
                let winners: Vec<usize> = round.matchups().iter().map(
                    |x| x.winner().unwrap()
                ).collect();
                let num_winners = winners.len();
                if num_winners <= 1 {
                    return Err(format!("Cannot generate next round, only {} teams remain", num_winners));
                }
                let next_round_matchups = num_winners.checked_div(2).ok_or(
                    String::from("Failed to calculate next round matchups")
                )?;

                // Match up winners of previous round against each other
                let mut week = LeagueSeasonWeek::new();
                for i in 0..next_round_matchups {
                    let t1_index = i * 2;
                    let t1_seed = match winners.get(t1_index) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", t1_index))
                    };
                    let t2_seed = match winners.get(t1_index + 1) {
                        Some(s) => s,
                        None => return Err(format!("No winner found at index {}", t1_index + 1))
                    };

                    // Get the home and away teams
                    let (home_id, home_name) = if t1_seed > t2_seed {
                        match self.teams.get(t1_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t1_seed))
                        }
                    } else {
                        match self.teams.get(t2_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t2_seed))
                        }
                    };
                    let (away_id, away_name) = if t1_seed > t2_seed {
                        match self.teams.get(t2_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t2_seed))
                        }
                    } else {
                        match self.teams.get(t1_seed) {
                            Some(t) => t,
                            None => return Err(format!("No team found with seed {}", t1_seed))
                        }
                    };

                    // Create the matchup and add to the week
                    let matchup = LeagueSeasonMatchup::new(*home_id, *away_id, home_name, away_name, rng);
                    week.matchups_mut().push(matchup);
                }
                self.rounds.push(week);
                Ok(())
            }
        }
    }
}
