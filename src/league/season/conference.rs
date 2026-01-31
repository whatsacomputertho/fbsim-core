#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};
use std::collections::HashSet;

/// Maximum allowed length for a division name
const MAX_DIVISION_NAME_LEN: usize = 64;

/// Maximum allowed length for a conference name
const MAX_CONFERENCE_NAME_LEN: usize = 64;

/// # `LeagueDivisionRaw` struct
///
/// A freshly deserialized `LeagueDivision` prior to validation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueDivisionRaw {
    pub name: String,
    pub teams: Vec<usize>,
}

impl LeagueDivisionRaw {
    /// Validate the raw division
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::LeagueDivisionRaw;
    ///
    /// let raw = LeagueDivisionRaw { name: "East".to_string(), teams: vec![0, 1] };
    /// assert!(raw.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // Validate name length
        if self.name.len() > MAX_DIVISION_NAME_LEN {
            return Err(format!(
                "Division name '{}' exceeds maximum length of {} characters",
                self.name, MAX_DIVISION_NAME_LEN
            ));
        }

        // Check for duplicate team IDs
        let mut seen = HashSet::new();
        for &team_id in &self.teams {
            if !seen.insert(team_id) {
                return Err(format!(
                    "Duplicate team ID {} in division '{}'",
                    team_id, self.name
                ));
            }
        }

        Ok(())
    }
}

impl TryFrom<LeagueDivisionRaw> for LeagueDivision {
    type Error = String;

    fn try_from(raw: LeagueDivisionRaw) -> Result<Self, Self::Error> {
        raw.validate()?;
        Ok(LeagueDivision {
            name: raw.name,
            teams: raw.teams,
        })
    }
}

/// # `LeagueConferenceRaw` struct
///
/// A freshly deserialized `LeagueConference` prior to validation.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueConferenceRaw {
    pub name: String,
    pub divisions: Vec<LeagueDivision>,
}

impl LeagueConferenceRaw {
    /// Validate the raw conference
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConferenceRaw, LeagueDivision};
    ///
    /// let mut div = LeagueDivision::with_name("East");
    /// div.add_team(0);
    /// div.add_team(1);
    /// let raw = LeagueConferenceRaw {
    ///     name: "AFC".to_string(),
    ///     divisions: vec![div],
    /// };
    /// assert!(raw.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        // Validate name length
        if self.name.len() > MAX_CONFERENCE_NAME_LEN {
            return Err(format!(
                "Conference name '{}' exceeds maximum length of {} characters",
                self.name, MAX_CONFERENCE_NAME_LEN
            ));
        }

        // Check for duplicate team IDs across divisions
        let mut seen = HashSet::new();
        for division in &self.divisions {
            for &team_id in division.teams() {
                if !seen.insert(team_id) {
                    return Err(format!(
                        "Duplicate team ID {} across divisions in conference '{}'",
                        team_id, self.name
                    ));
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<LeagueConferenceRaw> for LeagueConference {
    type Error = String;

    fn try_from(raw: LeagueConferenceRaw) -> Result<Self, Self::Error> {
        raw.validate()?;
        Ok(LeagueConference {
            name: raw.name,
            divisions: raw.divisions,
        })
    }
}

/// # `LeagueDivision` struct
///
/// A `LeagueDivision` represents a division within a conference, containing
/// a collection of team IDs. The division ID is implicit as the key in the
/// parent conference's `divisions` BTreeMap.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
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

impl<'de> Deserialize<'de> for LeagueDivision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = LeagueDivisionRaw::deserialize(deserializer)?;
        LeagueDivision::try_from(raw).map_err(serde::de::Error::custom)
    }
}

/// # `LeagueConference` struct
///
/// A `LeagueConference` represents a conference containing multiple divisions.
/// The conference ID is implicit as the index in the parent season's
/// `conferences` Vec.
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct LeagueConference {
    name: String,
    divisions: Vec<LeagueDivision>,
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
            divisions: Vec::new(),
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
            divisions: Vec::new(),
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
    pub fn divisions(&self) -> &Vec<LeagueDivision> {
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
    pub fn divisions_mut(&mut self) -> &mut Vec<LeagueDivision> {
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
    /// conference.add_division(division);
    /// ```
    pub fn add_division(&mut self, division: LeagueDivision) {
        self.divisions.push(division);
    }

    /// Get a division by ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// conference.add_division(LeagueDivision::with_name("East"));
    /// let division = conference.division(0);
    /// assert!(division.is_some());
    /// ```
    pub fn division(&self, id: usize) -> Option<&LeagueDivision> {
        self.divisions.get(id)
    }

    /// Get a mutable division by ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::conference::{LeagueConference, LeagueDivision};
    ///
    /// let mut conference = LeagueConference::new();
    /// conference.add_division(LeagueDivision::with_name("East"));
    /// let division = conference.division_mut(0);
    /// assert!(division.is_some());
    /// ```
    pub fn division_mut(&mut self, id: usize) -> Option<&mut LeagueDivision> {
        self.divisions.get_mut(id)
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
    /// conference.add_division(east);
    /// conference.add_division(west);
    ///
    /// let teams = conference.all_teams();
    /// assert_eq!(teams.len(), 4);
    /// ```
    pub fn all_teams(&self) -> Vec<usize> {
        self.divisions
            .iter()
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
    /// conference.add_division(division);
    ///
    /// assert!(conference.contains_team(0));
    /// assert!(!conference.contains_team(1));
    /// ```
    pub fn contains_team(&self, team_id: usize) -> bool {
        self.divisions.iter().any(|d| d.contains_team(team_id))
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
    /// conference.add_division(division);
    ///
    /// assert_eq!(conference.team_division(0), Some(0));
    /// assert_eq!(conference.team_division(1), None);
    /// ```
    pub fn team_division(&self, team_id: usize) -> Option<usize> {
        for (div_id, division) in self.divisions.iter().enumerate() {
            if division.contains_team(team_id) {
                return Some(div_id);
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
    /// conference.add_division(LeagueDivision::new());
    /// conference.add_division(LeagueDivision::new());
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
    /// conference.add_division(division);
    /// assert_eq!(conference.num_teams(), 2);
    /// ```
    pub fn num_teams(&self) -> usize {
        self.divisions.iter().map(|d| d.num_teams()).sum()
    }
}

impl<'de> Deserialize<'de> for LeagueConference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = LeagueConferenceRaw::deserialize(deserializer)?;
        LeagueConference::try_from(raw).map_err(serde::de::Error::custom)
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

        conference.add_division(east);
        conference.add_division(west);

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

        conference.add_division(east);
        conference.add_division(west);

        assert_eq!(conference.team_division(0), Some(0));
        assert_eq!(conference.team_division(1), Some(0));
        assert_eq!(conference.team_division(2), Some(1));
        assert_eq!(conference.team_division(3), Some(1));
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

        conference.add_division(div1);
        conference.add_division(div2);

        let teams = conference.all_teams();
        assert_eq!(teams.len(), 4);
        assert!(teams.contains(&0));
        assert!(teams.contains(&1));
        assert!(teams.contains(&2));
        assert!(teams.contains(&3));
    }
}
