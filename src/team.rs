use serde::{Serialize, Deserialize};

pub const DEFAULT_TEAM_NAME: &str = "Null Island Defaults";

/// # `FootballTeam` struct
///
/// A `FootballTeam` represents a football team
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeam {
    name: String,
    offense_overall: i32,
    defense_overall: i32
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
            offense_overall: 50_i32,
            defense_overall: 50_i32
        }
    }

    /// Constructor for the `FootballTeam` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// ```
    pub fn from_properties(name: &str, offense_overall: i32, defense_overall: i32) -> Result<FootballTeam, String> {
        // Ensure offense and defense overall are in range [0, 100]
        if !(offense_overall >= 0_i32 && offense_overall <= 100_i32) {
            return Err(
                format!(
                    "Offense overall not in range [0, 100]: {}",
                    offense_overall
                )
            )
        }
        if !(defense_overall >= 0_i32 && defense_overall <= 100_i32) {
            return Err(
                format!(
                    "Defense overall not in range [0, 100]: {}",
                    defense_overall
                )
            )
        }
        Ok(
            FootballTeam{
                name: String::from(name),
                offense_overall: offense_overall,
                defense_overall: defense_overall
            }
        )
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

    /// Get the football team's offense overall
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let offense = my_team.offense_overall();
    /// ```
    pub fn offense_overall(&self) -> &i32 {
        &self.offense_overall
    }

    /// Get the football team's offense overall mutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let mut my_team = FootballTeam::new();
    /// let mut offense = my_team.offense_overall_mut();
    /// ```
    pub fn offense_overall_mut(&mut self) -> &mut i32 {
        &mut self.offense_overall
    }

    /// Get the football team's defense overall
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let my_team = FootballTeam::new();
    /// let offense = my_team.defense_overall();
    /// ```
    pub fn defense_overall(&self) -> &i32 {
        &self.defense_overall
    }

    /// Get the football team's defense overall mutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let mut my_team = FootballTeam::new();
    /// let mut offense = my_team.defense_overall_mut();
    /// ```
    pub fn defense_overall_mut(&mut self) -> &mut i32 {
        &mut self.defense_overall
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_football_team_from_properties() {
        // Test offense overall OOB low
        let result_a = FootballTeam::from_properties("Test Team", -20, 50);
        let expected_a: Result<FootballTeam, String> = Err(
            String::from("Offense overall not in range [0, 100]: -20")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test offense overall OOB high
        let result_b = FootballTeam::from_properties("Test Team", 200, 50);
        let expected_b: Result<FootballTeam, String> = Err(
            String::from("Offense overall not in range [0, 100]: 200")
        );
        assert_eq!(
            result_b,
            expected_b
        );

        // Test defense overall OOB low
        let result_c = FootballTeam::from_properties("Test Team", 50, -100);
        let expected_c: Result<FootballTeam, String> = Err(
            String::from("Defense overall not in range [0, 100]: -100")
        );
        assert_eq!(
            result_c,
            expected_c
        );

        // Test defense overall OOB low
        let result_d = FootballTeam::from_properties("Test Team", 50, 150);
        let expected_d: Result<FootballTeam, String> = Err(
            String::from("Defense overall not in range [0, 100]: 150")
        );
        assert_eq!(
            result_d,
            expected_d
        );

        // Test both values in range
        let result_e = FootballTeam::from_properties("Test Team", 50, 50);
        let expected_e: Result<FootballTeam, String> = Ok(FootballTeam{
            name: String::from("Test Team"),
            offense_overall: 50,
            defense_overall: 50
        });
        assert_eq!(
            result_e,
            expected_e
        );
    }
}
