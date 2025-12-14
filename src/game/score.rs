pub mod freq;

use lazy_static::lazy_static;
use rand::Rng;
use rand_distr::{Normal, Distribution, Bernoulli};
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use statrs::distribution::Categorical;

use crate::game::score::freq::ScoreFrequencyLookup;
use crate::team::{DEFAULT_TEAM_NAME};

// Home score simulator model weights
const H_MEAN_COEF: f64 = 23.14578315_f64;
const H_MEAN_INTERCEPT: f64 = 10.9716991_f64;
const H_STD_INTERCEPT: f64 = 7.64006156_f64;
const H_STD_COEF_1: f64 = 5.72612946_f64;
const H_STD_COEF_2: f64 = -4.29283414_f64;

// Away score simulator model weights
const A_MEAN_COEF: f64 = 22.14952374_f64;
const A_MEAN_INTERCEPT: f64 = 8.92113289_f64;
const A_STD_INTERCEPT: f64 = 6.47638621_f64;
const A_STD_COEF_1: f64 = 8.00861267_f64;
const A_STD_COEF_2: f64 = -5.589282_f64;

// Tie probability model weights
const P_TIE_COEF: f64 = -0.00752297_f64;
const P_TIE_INTERCEPT: f64 = 0.01055039_f64;
const P_TIE_BASE: f64 = 0.036_f64;

// Score frequency distribution
lazy_static!{
    static ref SCORE_FREQ_LUT: ScoreFrequencyLookup = {
        let mut tmp_lut = ScoreFrequencyLookup::new();
        tmp_lut.create();
        tmp_lut
    };
}

/// # `ScoreSimulatable` trait
///
/// A `ScoreSimulatable` can return an offense and defense overall, which
/// are the two values needed to generate the final score of a game
pub trait ScoreSimulatable {
    fn name(&self) -> &str { DEFAULT_TEAM_NAME }
    fn defense_overall(&self) -> u32 { 50_u32 }
    fn offense_overall(&self) -> u32 { 50_u32 }
}

/// # `FinalScore` struct
///
/// A `FinalScore` represents the final score result of a football game
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FinalScore {
    home_team: String,
    home_score: i32,
    away_team: String,
    away_score: i32
}

impl FinalScore {
    /// Constructor for the `FinalScore` struct in which each score
    /// is defaulted to 0_i32, and each team name is defaulted to
    /// the default team name.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::FinalScore;
    ///
    /// let my_score = FinalScore::new();
    /// ```
    pub fn new() -> FinalScore {
        FinalScore{
            home_team: String::from(DEFAULT_TEAM_NAME),
            home_score: 0_i32,
            away_team: String::from(DEFAULT_TEAM_NAME),
            away_score: 0_i32
        }
    }

    /// Constructor for the `FinalScore` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::FinalScore;
    ///
    /// let my_score = FinalScore::from_properties(
    ///     "My Team A",
    ///     24_i32,
    ///     "My Team B",
    ///     17_i32
    /// );
    /// ```
    pub fn from_properties(home_team: &str, home_score: i32, away_team: &str, away_score: i32) -> Result<FinalScore, String> {
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
            FinalScore{
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
    /// use fbsim_core::game::score::FinalScore;
    ///
    /// let my_score = FinalScore::from_properties(
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
    /// use fbsim_core::game::score::FinalScore;
    ///
    /// let my_score = FinalScore::from_properties(
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

impl std::fmt::Display for FinalScore {
    /// Format a `FinalScore` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::score::FinalScore;
    ///
    /// let my_final_score = FinalScore::new();
    /// println!("{}", my_final_score);
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

/// # `FinalScoreSimulator` struct
///
/// A `FinalScoreSimulator` generates an american football final score
/// given the normalized skill differential (in range [0, 1]) of the
/// home offense and the away defense, and vice versa, the away
/// offense and the home defense.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct FinalScoreSimulator;

impl FinalScoreSimulator {
    /// Constructor for the `FinalScoreSimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::FinalScoreSimulator;
    ///
    /// let my_sim = FinalScoreSimulator::new();
    /// ```
    pub fn new() -> FinalScoreSimulator {
        FinalScoreSimulator{}
    }

    /// Gets the mean score parameter for the score generation
    fn get_mean_score(&self, norm_diff: f64, home: bool) -> f64 {
        // Get the mean score parameter
        if home {
            return H_MEAN_INTERCEPT + (H_MEAN_COEF * norm_diff)
        } else {
            return A_MEAN_INTERCEPT + (A_MEAN_COEF * norm_diff)
        }
    }

    /// Gets the score standard deviation parameter for the score
    /// generation
    fn get_std_score(&self, norm_diff: f64, home: bool) -> f64 {
        // Get the std score parameter
        if home {
            H_STD_INTERCEPT + (H_STD_COEF_1 * norm_diff) + (H_STD_COEF_2 * norm_diff.powi(2))
        } else {
            A_STD_INTERCEPT + (A_STD_COEF_1 * norm_diff) + (A_STD_COEF_2 * norm_diff.powi(2))
        }
    }

    /// Gets the normal distribution parameters for the score generation
    fn get_normal_params(&self, norm_diff: f64, home: bool) -> (f64, f64) {
        // Get the normal distribution parameters
        (self.get_mean_score(norm_diff, home), self.get_std_score(norm_diff, home))
    }

    /// Gets the probability of a tie for the given skill differential
    fn get_p_tie(&self, norm_diff: f64) -> f64 {
        return P_TIE_INTERCEPT + (P_TIE_COEF * norm_diff)
    }

    /// Gets the probability of a re-sim in the event of a tie in order to
    /// achieve the desired tie probability in the end
    fn get_p_resim(&self, p_tie: f64) -> f64 {
        return (p_tie - P_TIE_BASE) / (P_TIE_BASE.powi(2) - P_TIE_BASE)
    }

    /// Generates the away score only
    fn gen_away_score(&self, norm_diff: f64, rng: &mut impl Rng) -> i32 {
        // Create and sample a normal distribution for the score
        let (mean, std): (f64, f64) = self.get_normal_params(norm_diff, false);
        let away_dist = Normal::new(mean, std).unwrap();
        let away_score_float = away_dist.sample(rng);

        // Round to nearest integer and return
        let away_score = if away_score_float < 0_f64 {
            0_i32
        } else {
            away_score_float.round() as i32
        };
        return away_score
    }

    /// Generates the home score only
    fn gen_home_score(&self, norm_diff: f64, rng: &mut impl Rng) -> i32 {
        // Create and sample a normal distribution for the score
        let (mean, std) = self.get_normal_params(norm_diff, true);
        let home_dist = Normal::new(mean, std).unwrap();
        let home_score_float = home_dist.sample(rng);

        // Round to nearest integer, ensure positive and return
        let home_score = if home_score_float < 0_f64 {
            0_i32
        } else {
            home_score_float.round() as i32
        };
        return home_score
    }

    /// Generates the home and away scores, returns as a 2-tuple
    /// in which the first value is the home score, and the second
    /// value is the away score
    fn gen_score(&self, ha_norm_diff: f64, ah_norm_diff: f64, rng: &mut impl Rng) -> Result<(i32, i32), String> {
         // Ensure normalized differentials are in range [0, 1]
         if !(ha_norm_diff >= 0.0_f64 && ha_norm_diff <= 1.0_f64) {
            return Err(
                format!(
                    "Home offense / away defense normalized skill differential not in range [0, 1]: {}",
                    ha_norm_diff
                )
            )
        }
        if !(ah_norm_diff >= 0.0_f64 && ah_norm_diff <= 1.0_f64) {
           return Err(
               format!(
                   "Away offense / home defense normalized skill differential not in range [0, 1]: {}",
                   ah_norm_diff
               )
           )
        }

        // Generate the home and away scores
        Ok((self.gen_home_score(ha_norm_diff, rng), self.gen_away_score(ah_norm_diff, rng)))
    }

    /// Filters the final score by score frequency.  The score's nearest
    /// neighbors and their frequency are retrieved to construct a probability
    /// mass function for a categorical distribution.  That distribution is
    /// then sampled for the real score.
    fn filter_score(&self, score: i32, rng: &mut impl Rng) -> i32 {
        // If the score is 0, just return 0 as 1 is impossible
        if score == 0 {
            return 0
        }

        // Get the nearest neighbors of the score
        let low = SCORE_FREQ_LUT.frequency(score - 1).unwrap();
        let mid = SCORE_FREQ_LUT.frequency(score).unwrap();
        let high = SCORE_FREQ_LUT.frequency(score + 1).unwrap();
        
        // Construct a categorical distribution
        let dist = Categorical::new(&[low as f64, mid as f64, high as f64]).unwrap();
        let score_adjustment_r: f64 = dist.sample(rng);
        let score_adjustment = (score_adjustment_r as i32) - 1_i32;
        let adj_score = score + score_adjustment;
        adj_score
    }

    /// Simulates a game by generating a final score result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::score::{FinalScore, FinalScoreSimulator};
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let home = FootballTeam::new();
    /// let away = FootballTeam::new();
    /// let sim = FinalScoreSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let score = sim.sim(&home, &away, &mut rng).unwrap();
    /// println!("{}", score);
    /// ```
    pub fn sim(&self, home_team: &impl ScoreSimulatable, away_team: &impl ScoreSimulatable, rng: &mut impl Rng) -> Result<FinalScore, String> {
        // Calculate the normalized skill differentials
        let ha_norm_diff: f64 = (home_team.offense_overall() as i32 - away_team.defense_overall() as i32 + 100_i32) as f64 / 200_f64;
        let ah_norm_diff: f64 = (away_team.offense_overall() as i32 - home_team.defense_overall() as i32 + 100_i32) as f64 / 200_f64;

        // Generate the final score, return error if error is encountered
        let (home_score, away_score): (i32, i32) = match self.gen_score(ha_norm_diff, ah_norm_diff, rng) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        // Filter the final score by score frequency
        let adj_home_score = self.filter_score(home_score, rng);
        let adj_away_score = self.filter_score(away_score, rng);

        // Instantiate as a FinalScore
        let final_score: FinalScore = FinalScore::from_properties(
            home_team.name(),
            adj_home_score,
            away_team.name(),
            adj_away_score
        ).unwrap();

        // If not a tie, then return as-is
        if adj_home_score != adj_away_score {
            return Ok(final_score)
        }

        // If a tie is achieved after filtering, re-sim based on the skill
        // differentials and their associated tie probability.  Start by
        // calculating the average of the two skill differentials
        let avg_norm_diff: f64 = (ha_norm_diff + ah_norm_diff) / 2_f64;

        // Get the probability of a tie for the average skill differential.
        // Use it to get the required probability of a re-sim to achieve the
        // observed tie probability in the end
        let p_tie: f64 = self.get_p_tie(avg_norm_diff);
        let p_res: f64 = self.get_p_resim(p_tie);

        // Sample a bernoulli distribution of p_res to determine whether
        // to re-sim or not
        let dst_res: Bernoulli = Bernoulli::new(p_res).unwrap();
        let res: bool = dst_res.sample(rng);

        // Re-sim and re-filter if needed
        if res {
            // Generate the final score, return error if error is encountered
            let (home_score_2, away_score_2): (i32, i32) = match self.gen_score(ha_norm_diff, ah_norm_diff, rng) {
                Ok(v) => v,
                Err(e) => return Err(e)
            };

            // Filter the final score by score frequency
            let adj_home_score_2 = self.filter_score(home_score_2, rng);
            let adj_away_score_2 = self.filter_score(away_score_2, rng);

            // Instantiate as a FinalScore and return
            let final_score_2: FinalScore = FinalScore::from_properties(
                home_team.name(),
                adj_home_score_2,
                away_team.name(),
                adj_away_score_2
            ).unwrap();
            return Ok(final_score_2)
        }

        return Ok(final_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_final_score_from_properties() {
        // Test home score OOB
        let result_a = FinalScore::from_properties("Test Team A", -3, "Test Team B", 24);
        let expected_a: Result<FinalScore, String> = Err(
            String::from("Home score not in range [0, max): -3")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test away score OOB
        let result_b = FinalScore::from_properties("Test Team A", 17, "Test Team B", -17);
        let expected_b: Result<FinalScore, String> = Err(
            String::from("Away score not in range [0, max): -17")
        );
        assert_eq!(
            result_b,
            expected_b
        );

        // Test both scores in range
        let result_c = FinalScore::from_properties("Test Team A", 17, "Test Team B", 24);
        let expected_c: Result<FinalScore, String> = Ok(
            FinalScore{
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
