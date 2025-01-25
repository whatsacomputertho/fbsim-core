#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::team::{DEFAULT_TEAM_NAME};

/// # `BoxScore` struct
///
/// A `BoxScore` represents the result of a football game
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct BoxScore {
    home_team: String,
    home_score: i32,
    away_team: String,
    away_score: i32
}

impl BoxScore {
    /// Constructor for the `BoxScore` struct in which each score
    /// is defaulted to 0_i32, and each team name is defaulted to
    /// the default team name.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    ///
    /// let my_score = BoxScore::new();
    /// ```
    pub fn new() -> BoxScore {
        BoxScore{
            home_team: String::from(DEFAULT_TEAM_NAME),
            home_score: 0_i32,
            away_team: String::from(DEFAULT_TEAM_NAME),
            away_score: 0_i32
        }
    }

    /// Constructor for the `BoxScore` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    ///
    /// let my_score = BoxScore::from_properties(
    ///     "My Team A",
    ///     24_i32,
    ///     "My Team B",
    ///     17_i32
    /// );
    /// ```
    pub fn from_properties(home_team: &str, home_score: i32, away_team: &str, away_score: i32) -> Result<BoxScore, String> {
        // Ensure home and away scores are in range [0, max)
        if !home_score >= 0_i32 {
            return Err(
                format!(
                    "Home score not in range [0, max): {}",
                    home_score
                )
            )
        }
        if !away_score >= 0_i32 {
            return Err(
                format!(
                    "Away score not in range [0, max): {}",
                    away_score
                )
            )
        }
        Ok(
            BoxScore{
                home_team: String::from(home_team),
                home_score: home_score,
                away_team: String::from(away_team),
                away_score: away_score
            }
        )
    }

    /// Getter for the home score property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    ///
    /// let my_score = BoxScore::from_properties(
    ///     "My Team A",
    ///     24_i32,
    ///     "My Team B",
    ///     17_i32
    /// ).unwrap();
    /// let home_score = my_score.home_score();
    /// println!("{}", home_score); // 24
    /// ```
    pub fn home_score(&self) -> i32 {
        self.home_score
    }

    /// Getter for the away score property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    ///
    /// let my_score = BoxScore::from_properties(
    ///     "My Team A",
    ///     24_i32,
    ///     "My Team B",
    ///     17_i32
    /// ).unwrap();
    /// let away_score = my_score.away_score();
    /// println!("{}", away_score); // 17
    /// ```
    pub fn away_score(&self) -> i32 {
        self.away_score
    }
}

impl std::fmt::Display for BoxScore {
    /// Format a `BoxScore` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    ///
    /// let my_box_score = BoxScore::new();
    /// println!("{}", my_box_score);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let score_str = format!(
            "{} {} - {} {}",
            self.home_team,
            self.home_score,
            self.away_team,
            self.away_score
        );
        f.write_str(&score_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_score_from_properties() {
        // Test home score OOB
        let result_a = BoxScore::from_properties("Test Team A", -3, "Test Team B", 24);
        let expected_a: Result<BoxScore, String> = Err(
            String::from("Home score not in range [0, max): -3")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test away score OOB
        let result_b = BoxScore::from_properties("Test Team A", 17, "Test Team B", -17);
        let expected_b: Result<BoxScore, String> = Err(
            String::from("Away score not in range [0, max): -17")
        );
        assert_eq!(
            result_b,
            expected_b
        );

        // Test both scores in range
        let result_c = BoxScore::from_properties("Test Team A", 17, "Test Team B", 24);
        let expected_c: Result<BoxScore, String> = Ok(
            BoxScore{
                home_team: String::from("Test Team A"),
                home_score: 17,
                away_team: String::from("Test Team B"),
                away_score: 24
            }
        );
        assert_eq!(
            result_c,
            expected_c
        );
    }
}
