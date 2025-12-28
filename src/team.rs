#![doc = include_str!("../docs/team.md")]
pub mod coach;
pub mod defense;
pub mod offense;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};

use crate::game::play::PlaySimulatable;
use crate::game::score::ScoreSimulatable;
use crate::team::coach::FootballTeamCoach;
use crate::team::defense::FootballTeamDefense;
use crate::team::offense::FootballTeamOffense;

pub const DEFAULT_TEAM_NAME: &str = "Null Island Defaults";
pub const DEFAULT_TEAM_SHORT_NAME: &str = "NULL";

/// # `FootballTeamRaw` struct
///
/// A `FootballTeamRaw` is a `FootballTeam` before its properties have been
/// validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamRaw {
    name: String,
    short_name: String,
    coach: FootballTeamCoach,
    defense: FootballTeamDefense,
    offense: FootballTeamOffense
}

impl FootballTeamRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure the team name is no longer than 64 characters
        if self.name.len() > 64 {
            return Err(
                format!(
                    "Team name is longer than 64 characters: {}",
                    self.name
                )
            )
        }

        // Ensure the team acronym is no longer than 4 characters
        if self.short_name.len() > 4 {
            return Err(
                format!(
                    "Team short name is longer than 4 characters: {}",
                    self.short_name
                )
            )
        }
        Ok(())
    }
}

/// # `FootballTeam` struct
///
/// A `FootballTeam` represents a football team
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize)]
pub struct FootballTeam {
    name: String,
    short_name: String,
    coach: FootballTeamCoach,
    defense: FootballTeamDefense,
    offense: FootballTeamOffense
}

impl TryFrom<FootballTeamRaw> for FootballTeam {
    type Error = String;

    fn try_from(item: FootballTeamRaw) -> Result<Self, Self::Error> {
        // Validate the raw team
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            FootballTeam{
                name: item.name,
                short_name: item.short_name,
                coach: item.coach,
                offense: item.offense,
                defense: item.defense
            }
        )
    }
}

impl<'de> Deserialize<'de> for FootballTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = FootballTeamRaw::deserialize(deserializer)?;
        FootballTeam::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl PlaySimulatable for FootballTeam {
    /// Borrow the team's coach
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::PlaySimulatable;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let my_coach = my_team.coach();
    /// ```
    fn coach(&self) -> &FootballTeamCoach {
        &self.coach
    }

    /// Borrow the team's offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::PlaySimulatable;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let my_offense = my_team.offense();
    /// ```
    fn offense(&self) -> &FootballTeamOffense {
        &self.offense
    }

    /// Borrow the team's defense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::PlaySimulatable;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let my_defense = my_team.defense();
    /// ```
    fn defense(&self) -> &FootballTeamDefense {
        &self.defense
    }
}

impl ScoreSimulatable for FootballTeam {
    /// Get the football team's name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let name = my_team.name();
    /// ```
    fn name(&self) -> &str {
        &self.name
    }

    /// Get the overall of the defense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::ScoreSimulatable;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::from_overalls("My Team", "TEAM", 25, 75).unwrap();
    /// let defense_overall = my_team.defense_overall();
    /// assert!(defense_overall == 75);
    /// ```
    fn defense_overall(&self) -> u32 {
        self.defense.overall()
    }

    /// Get the overall of the offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::ScoreSimulatable;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::from_overalls("My Team", "TEAM", 25, 75).unwrap();
    /// let offense_overall = my_team.offense_overall();
    /// assert!(offense_overall == 25);
    /// ```
    fn offense_overall(&self) -> u32 {
        self.offense.overall()
    }
}

impl FootballTeam {
    /// Constructor for the `FootballTeam` struct in which each
    /// overall is defaulted to 50_i32, and the name is defaulted
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// ```
    pub fn new() -> FootballTeam {
        FootballTeam{
            name: String::from(DEFAULT_TEAM_NAME),
            short_name: String::from(DEFAULT_TEAM_SHORT_NAME),
            coach: FootballTeamCoach::new(),
            offense: FootballTeamOffense::new(),
            defense: FootballTeamDefense::new()
        }
    }

    /// Constructor for the `FootballTeam` struct in which an offense and defense
    /// are constructed given their overalls
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::from_overalls("My Team", "TEAM", 25, 75);
    /// ```
    pub fn from_overalls(name: &str, short_name: &str, offense_overall: u32, defense_overall: u32) -> Result<FootballTeam, String> {
        let offense = FootballTeamOffense::from_overall(offense_overall)?;
        let defense = FootballTeamDefense::from_overall(defense_overall)?;
        Ok(
            FootballTeam{
                name: String::from(name),
                short_name: String::from(short_name),
                coach: FootballTeamCoach::new(),
                offense: offense,
                defense: defense
            }
        )
    }

    /// Constructor for the `FootballTeam` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::team::coach::FootballTeamCoach;
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// use fbsim_core::team::defense::FootballTeamDefense;
    ///
    /// let my_coach = FootballTeamCoach::new();
    /// let my_defense = FootballTeamDefense::new();
    /// let my_offense = FootballTeamOffense::new();
    /// let my_team = FootballTeam::from_properties("My Team", "TEAM", my_coach, my_offense, my_defense);
    /// ```
    pub fn from_properties(name: &str, short_name: &str, coach: FootballTeamCoach, offense: FootballTeamOffense, defense: FootballTeamDefense) -> FootballTeam {
        FootballTeam{
            name: String::from(name),
            short_name: String::from(short_name),
            coach: coach,
            offense: offense,
            defense: defense
        }
    }

    /// Get the football team's name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let name = my_team.name();
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the football team's name mutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let mut my_team = FootballTeam::new();
    /// let mut name = my_team.name_mut();
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Borrow the football team's short name / acronym
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let short_name = my_team.short_name();
    /// ```
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Borrow the football team's short name / acronym mutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let mut my_team = FootballTeam::new();
    /// let mut short_name = my_team.short_name_mut();
    /// ```
    pub fn short_name_mut(&mut self) -> &mut String {
        &mut self.short_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_football_team_from_properties() {
        // Test offense overall OOB high
        let result_a = FootballTeam::from_overalls("Test Team", "TEST", 200, 50);
        let expected_a: Result<FootballTeam, String> = Err(
            String::from("Passing attribute is out of range [0, 100]: 200")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test defense overall OOB high
        let result_b = FootballTeam::from_overalls("Test Team", "TEST", 50, 150);
        let expected_b: Result<FootballTeam, String> = Err(
            String::from("Blitzing attribute is out of range [0, 100]: 150")
        );
        assert_eq!(
            result_b,
            expected_b
        );
    }
}
