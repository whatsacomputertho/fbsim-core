pub mod coach;
pub mod defense;
pub mod offense;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::game::play::PlaySimulatable;
use crate::game::score::ScoreSimulatable;
use crate::team::coach::FootballTeamCoach;
use crate::team::defense::FootballTeamDefense;
use crate::team::offense::FootballTeamOffense;

pub const DEFAULT_TEAM_NAME: &str = "Null Island Defaults";

/// # `FootballTeam` struct
///
/// A `FootballTeam` represents a football team
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeam {
    name: String,
    coach: FootballTeamCoach,
    defense: FootballTeamDefense,
    offense: FootballTeamOffense
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
    /// let my_team = FootballTeam::from_overalls("My Team", 25, 75).unwrap();
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
    /// let my_team = FootballTeam::from_overalls("My Team", 25, 75).unwrap();
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
    /// let my_team = FootballTeam::from_overalls("My Team", 25, 75);
    /// ```
    pub fn from_overalls(name: &str, offense_overall: u32, defense_overall: u32) -> Result<FootballTeam, String> {
        let offense = match FootballTeamOffense::from_overall(offense_overall) {
            Ok(o) => o,
            Err(msg) => return Err(msg)
        };
        let defense = match FootballTeamDefense::from_overall(defense_overall) {
            Ok(d) => d,
            Err(msg) => return Err(msg)
        };
        Ok(
            FootballTeam{
                name: String::from(name),
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
    /// let my_team = FootballTeam::from_properties("My Team", my_coach, my_offense, my_defense);
    /// ```
    pub fn from_properties(name: &str, coach: FootballTeamCoach, offense: FootballTeamOffense, defense: FootballTeamDefense) -> FootballTeam {
        FootballTeam{
            name: String::from(name),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_football_team_from_properties() {
        // Test offense overall OOB high
        let result_a = FootballTeam::from_overalls("Test Team", 200, 50);
        let expected_a: Result<FootballTeam, String> = Err(
            String::from("Overall not in range [0, 100]: 200")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test defense overall OOB high
        let result_b = FootballTeam::from_overalls("Test Team", 50, 150);
        let expected_b: Result<FootballTeam, String> = Err(
            String::from("Overall not in range [0, 100]: 150")
        );
        assert_eq!(
            result_b,
            expected_b
        );
    }
}
