//! League conference and division structures for organizing teams

use std::collections::BTreeMap;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// # `LeagueDivision` struct
///
/// A `LeagueDivision` represents a division within a conference, containing
/// a collection of team IDs. The division ID is implicit as the key in the
/// parent conference's `divisions` BTreeMap.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueDivision {
    name: String,
    teams: Vec<usize>,
}

impl Default for LeagueDivision {
    /// Default constructor for the LeagueDivision struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let division = LeagueDivision::default();
    /// ```
    fn default() -> Self {
        LeagueDivision {
            name: String::new(),
            teams: Vec::new(),
        }
    }
}

impl LeagueDivision {
    /// Constructor for the LeagueDivision struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let division = LeagueDivision::new();
    /// ```
    pub fn new() -> LeagueDivision {
        LeagueDivision::default()
    }

    /// Constructor with name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let division = LeagueDivision::with_name("East");
    /// ```
    pub fn with_name(name: &str) -> LeagueDivision {
        LeagueDivision {
            name: name.to_string(),
            teams: Vec::new(),
        }
    }

    /// Borrow the division name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let division = LeagueDivision::with_name("East");
    /// assert_eq!(division.name(), "East");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Mutably borrow the division name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let mut division = LeagueDivision::new();
    /// *division.name_mut() = "West".to_string();
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Borrow the teams in the division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let division = LeagueDivision::new();
    /// let teams = division.teams();
    /// ```
    pub fn teams(&self) -> &Vec<usize> {
        &self.teams
    }

    /// Mutably borrow the teams in the division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let mut division = LeagueDivision::new();
    /// division.teams_mut().push(0);
    /// ```
    pub fn teams_mut(&mut self) -> &mut Vec<usize> {
        &mut self.teams
    }

    /// Add a team to the division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// division.add_team(1);
    /// assert_eq!(division.teams().len(), 2);
    /// ```
    pub fn add_team(&mut self, team_id: usize) {
        self.teams.push(team_id);
    }

    /// Check if a team is in this division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// assert!(division.contains_team(0));
    /// assert!(!division.contains_team(1));
    /// ```
    pub fn contains_team(&self, team_id: usize) -> bool {
        self.teams.contains(&team_id)
    }

    /// Get the number of teams in the division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivision;
    ///
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// division.add_team(1);
    /// assert_eq!(division.num_teams(), 2);
    /// ```
    pub fn num_teams(&self) -> usize {
        self.teams.len()
    }
}

/// # `LeagueConference` struct
///
/// A `LeagueConference` represents a conference containing multiple divisions.
/// The conference ID is implicit as the index in the parent season's
/// `conferences` Vec.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueConference {
    name: String,
    divisions: BTreeMap<usize, LeagueDivision>,
}

impl Default for LeagueConference {
    /// Default constructor for the LeagueConference struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let conference = LeagueConference::default();
    /// ```
    fn default() -> Self {
        LeagueConference {
            name: String::new(),
            divisions: BTreeMap::new(),
        }
    }
}

impl LeagueConference {
    /// Constructor for the LeagueConference struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let conference = LeagueConference::new();
    /// ```
    pub fn new() -> LeagueConference {
        LeagueConference::default()
    }

    /// Constructor with name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let conference = LeagueConference::with_name("AFC");
    /// ```
    pub fn with_name(name: &str) -> LeagueConference {
        LeagueConference {
            name: name.to_string(),
            divisions: BTreeMap::new(),
        }
    }

    /// Borrow the conference name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let conference = LeagueConference::with_name("AFC");
    /// assert_eq!(conference.name(), "AFC");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Mutably borrow the conference name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let mut conference = LeagueConference::new();
    /// *conference.name_mut() = "NFC".to_string();
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Borrow the divisions in the conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let conference = LeagueConference::new();
    /// let divisions = conference.divisions();
    /// ```
    pub fn divisions(&self) -> &BTreeMap<usize, LeagueDivision> {
        &self.divisions
    }

    /// Mutably borrow the divisions in the conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueConference;
    ///
    /// let mut conference = LeagueConference::new();
    /// let divisions = conference.divisions_mut();
    /// ```
    pub fn divisions_mut(&mut self) -> &mut BTreeMap<usize, LeagueDivision> {
        &mut self.divisions
    }

    /// Add a division to the conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// let division = LeagueDivision::with_name("East");
    /// conference.add_division(0, division);
    /// ```
    pub fn add_division(&mut self, id: usize, division: LeagueDivision) {
        self.divisions.insert(id, division);
    }

    /// Get a division by ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// conference.add_division(0, LeagueDivision::with_name("East"));
    /// let division = conference.division(0);
    /// assert!(division.is_some());
    /// ```
    pub fn division(&self, id: usize) -> Option<&LeagueDivision> {
        self.divisions.get(&id)
    }

    /// Get a mutable division by ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// conference.add_division(0, LeagueDivision::with_name("East"));
    /// let division = conference.division_mut(0);
    /// assert!(division.is_some());
    /// ```
    pub fn division_mut(&mut self, id: usize) -> Option<&mut LeagueDivision> {
        self.divisions.get_mut(&id)
    }

    /// Get all team IDs in this conference (across all divisions)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// let mut east = LeagueDivision::with_name("East");
    /// east.add_team(0);
    /// east.add_team(1);
    /// let mut west = LeagueDivision::with_name("West");
    /// west.add_team(2);
    /// west.add_team(3);
    /// conference.add_division(0, east);
    /// conference.add_division(1, west);
    ///
    /// let teams = conference.all_teams();
    /// assert_eq!(teams.len(), 4);
    /// ```
    pub fn all_teams(&self) -> Vec<usize> {
        self.divisions
            .values()
            .flat_map(|d| d.teams().iter().cloned())
            .collect()
    }

    /// Check if a team is in this conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// conference.add_division(0, division);
    ///
    /// assert!(conference.contains_team(0));
    /// assert!(!conference.contains_team(1));
    /// ```
    pub fn contains_team(&self, team_id: usize) -> bool {
        self.divisions.values().any(|d| d.contains_team(team_id))
    }

    /// Find which division a team is in (returns division ID)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// conference.add_division(5, division);
    ///
    /// assert_eq!(conference.team_division(0), Some(5));
    /// assert_eq!(conference.team_division(1), None);
    /// ```
    pub fn team_division(&self, team_id: usize) -> Option<usize> {
        for (div_id, division) in &self.divisions {
            if division.contains_team(team_id) {
                return Some(*div_id);
            }
        }
        None
    }

    /// Get the number of divisions in the conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// conference.add_division(0, LeagueDivision::new());
    /// conference.add_division(1, LeagueDivision::new());
    /// assert_eq!(conference.num_divisions(), 2);
    /// ```
    pub fn num_divisions(&self) -> usize {
        self.divisions.len()
    }

    /// Get the total number of teams in the conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// let mut division = LeagueDivision::new();
    /// division.add_team(0);
    /// division.add_team(1);
    /// conference.add_division(0, division);
    /// assert_eq!(conference.num_teams(), 2);
    /// ```
    pub fn num_teams(&self) -> usize {
        self.divisions.values().map(|d| d.num_teams()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_creation() {
        let mut division = LeagueDivision::with_name("East");
        division.add_team(0);
        division.add_team(1);
        division.add_team(2);
        division.add_team(3);

        assert_eq!(division.name(), "East");
        assert_eq!(division.num_teams(), 4);
        assert!(division.contains_team(0));
        assert!(division.contains_team(3));
        assert!(!division.contains_team(4));
    }

    #[test]
    fn test_conference_creation() {
        let mut conference = LeagueConference::with_name("AFC");

        let mut east = LeagueDivision::with_name("East");
        east.add_team(0);
        east.add_team(1);

        let mut west = LeagueDivision::with_name("West");
        west.add_team(2);
        west.add_team(3);

        conference.add_division(0, east);
        conference.add_division(1, west);

        assert_eq!(conference.name(), "AFC");
        assert_eq!(conference.num_divisions(), 2);
        assert_eq!(conference.num_teams(), 4);
        assert!(conference.contains_team(0));
        assert!(conference.contains_team(3));
        assert!(!conference.contains_team(4));
    }

    #[test]
    fn test_team_division_lookup() {
        let mut conference = LeagueConference::new();

        let mut east = LeagueDivision::new();
        east.add_team(0);
        east.add_team(1);

        let mut west = LeagueDivision::new();
        west.add_team(2);
        west.add_team(3);

        conference.add_division(10, east);
        conference.add_division(20, west);

        assert_eq!(conference.team_division(0), Some(10));
        assert_eq!(conference.team_division(1), Some(10));
        assert_eq!(conference.team_division(2), Some(20));
        assert_eq!(conference.team_division(3), Some(20));
        assert_eq!(conference.team_division(4), None);
    }

    #[test]
    fn test_all_teams() {
        let mut conference = LeagueConference::new();

        let mut div1 = LeagueDivision::new();
        div1.add_team(0);
        div1.add_team(1);

        let mut div2 = LeagueDivision::new();
        div2.add_team(2);
        div2.add_team(3);

        conference.add_division(0, div1);
        conference.add_division(1, div2);

        let teams = conference.all_teams();
        assert_eq!(teams.len(), 4);
        assert!(teams.contains(&0));
        assert!(teams.contains(&1));
        assert!(teams.contains(&2));
        assert!(teams.contains(&3));
    }
}
