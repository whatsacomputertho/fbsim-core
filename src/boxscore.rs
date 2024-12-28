use rand::Rng;
use rand_distr::{Normal, Distribution};

// Home score generator model weights
const H_MEAN_INTERCEPT: f64 = 23.14578315_f64;
const H_MEAN_COEF: f64 = 10.9716991_f64;
const H_STD_INTERCEPT: f64 = 7.64006156_f64;
const H_STD_COEF_1: f64 = 5.72612946_f64;
const H_STD_COEF_2: f64 = -4.29283414_f64;

// Away score generator model weights
const A_MEAN_INTERCEPT: f64 = 22.14952374_f64;
const A_MEAN_COEF: f64 = 8.92113289_f64;
const A_STD_INTERCEPT: f64 = 6.47638621_f64;
const A_STD_COEF_1: f64 = 8.00861267_f64;
const A_STD_COEF_2: f64 = -5.589282_f64;

/// # `BoxScoreGenerator` struct
///
/// A `BoxScoreGenerator` generates an american football box score
/// given the normalized skill differential (in range [0, 1]) of the
/// home offense and the away defense, and vice versa, the away
/// offense and the home defense.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BoxScoreGenerator {
    home_off_away_def_norm_diff: f64,
    away_off_home_def_norm_diff: f64
}

impl BoxScoreGenerator {
    /// Constructor for the `BoxScoreGenerator` struct in which each
    /// skill differential is defaulted to 0.5_f64,
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScoreGenerator;
    ///
    /// let my_box_score_gen = BoxScoreGenerator::new();
    /// ```
    pub fn new() -> BoxScoreGenerator {
        BoxScoreGenerator{
            home_off_away_def_norm_diff: 0.5_f64,
            away_off_home_def_norm_diff: 0.5_f64
        }
    }

    /// Constructor for the `BoxScoreGenerator` struct in which its
    /// struct properties are given as arguments.  Here, the first
    /// argument is the normalized differential between the home
    /// offensive skill and the away defensive skill.  The second
    /// argument is then the normalized skill differential between
    /// the away offense and the home defense.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScoreGenerator;
    ///
    /// let my_home_off_away_def_norm_diff = 0.75_f64;
    /// let my_away_off_home_def_norm_diff = 0.33_f64;
    /// let my_box_score_gen = BoxScoreGenerator::from_properties(
    ///     my_home_off_away_def_norm_diff,
    ///     my_away_off_home_def_norm_diff
    /// );
    /// ```
    pub fn from_properties(ha_norm_diff: f64, ah_norm_diff: f64) -> Result<BoxScoreGenerator, String> {
        // Ensure normalized differentials are in range [0, 1]
        if !(ha_norm_diff >= 0.0_f64 && ha_norm_diff <= 1.0_f64) {
            return Err(
                format!(
                    "Home offense / away defense noralized skill differential not in range [0, 1]: {}",
                    ha_norm_diff
                )
            )
        }
        if !(ah_norm_diff >= 0.0_f64 && ah_norm_diff <= 1.0_f64) {
            return Err(
                format!(
                    "Away offense / home defense noralized skill differential not in range [0, 1]: {}",
                    ah_norm_diff
                )
            )
        }

        // If so then return the result wrapping the instantiated struct
        Ok(
            BoxScoreGenerator{
                home_off_away_def_norm_diff: ha_norm_diff,
                away_off_home_def_norm_diff: ah_norm_diff
            }
        )
    }

    /// Set the home offense to away defense normalized skill differential.
    /// Returns a result describing whether the mutation succeeded.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScoreGenerator;
    /// 
    /// let mut my_box_score_gen = BoxScoreGenerator::new();
    /// my_box_score_gen.set_home_off_away_def_norm_diff(0.75_f64).unwrap();
    /// ```
    pub fn set_home_off_away_def_norm_diff(&mut self, ha_norm_diff: f64) -> Result<(), String> {
        // Ensure normalized differential is in range [0, 1]
        if !(ha_norm_diff >= 0.0_f64 && ha_norm_diff <= 1.0_f64) {
            return Err(
                format!(
                    "Home offense / away defense noralized skill differential not in range [0, 1]: {}",
                    ha_norm_diff
                )
            )
        }

        // If so, set the normalized differential
        self.home_off_away_def_norm_diff = ha_norm_diff;
        Ok(())
    }

    /// Set the away offense to home defense normalized skill differential.
    /// Returns a result describing whether the mutation succeeded.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScoreGenerator;
    /// 
    /// let mut my_box_score_gen = BoxScoreGenerator::new();
    /// my_box_score_gen.set_away_off_home_def_norm_diff(0.25_f64).unwrap();
    /// ```
    pub fn set_away_off_home_def_norm_diff(&mut self, ah_norm_diff: f64) -> Result<(), String> {
        // Ensure normalized differential is in range [0, 1]
        if !(ah_norm_diff >= 0.0_f64 && ah_norm_diff <= 1.0_f64) {
            return Err(
                format!(
                    "Away offense / home defense noralized skill differential not in range [0, 1]: {}",
                    ah_norm_diff
                )
            )
        }

        // If so, set the normalized differential
        self.away_off_home_def_norm_diff = ah_norm_diff;
        Ok(())
    }

    /// Gets the mean score parameter for the away generation
    fn get_mean_score(&self, home: bool) -> f64 {
        if home {
            H_MEAN_INTERCEPT + (H_MEAN_COEF * self.home_off_away_def_norm_diff)
        } else {
            A_MEAN_INTERCEPT + (A_MEAN_COEF * self.away_off_home_def_norm_diff)
        }
    }

    /// Gets the score standard deviation parameter for the score
    /// generation
    fn get_std_score(&self, home: bool) -> f64 {
        if home {
            H_STD_INTERCEPT + (H_STD_COEF_1 * self.home_off_away_def_norm_diff) + (H_STD_COEF_2 * self.home_off_away_def_norm_diff.powi(2))
        } else {
            A_STD_INTERCEPT + (A_STD_COEF_1 * self.away_off_home_def_norm_diff) + (A_STD_COEF_2 * self.away_off_home_def_norm_diff.powi(2))
        }
    }

    /// Gets the normal distribution parameters for the score generation
    fn get_normal_params(&self, home: bool) -> (f64, f64) {
        (self.get_mean_score(home), self.get_std_score(home))
    }

    /// Generates the away score only
    fn gen_away_score(&self, rng: &mut impl Rng) -> i32 {
        // Create and sample a normal distribution for the score
        let (mean, std) = self.get_normal_params(false);
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
    fn gen_home_score(&self, rng: &mut impl Rng) -> i32 {
        // Create and sample a normal distribution for the score
        let (mean, std) = self.get_normal_params(true);
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
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScoreGenerator;
    /// 
    /// let mut rng = rand::thread_rng();
    /// let my_box_score_gen = BoxScoreGenerator::new();
    /// let (home_score, away_score): (i32, i32) = my_box_score_gen.gen(&mut rng);
    /// ```
    pub fn gen(&self, rng: &mut impl Rng) -> (i32, i32) {
        (self.gen_home_score(rng), self.gen_away_score(rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_score_gen_from_properties() {
        // Test H/A diff OOB low
        let result_a = BoxScoreGenerator::from_properties(
            -1_f64,
            0.5_f64
        );
        let expected_a: Result<BoxScoreGenerator, String> = Err(
            String::from("Home offense / away defense noralized skill differential not in range [0, 1]: -1")
        );
        assert_eq!(
            result_a,
            expected_a
        );

        // Test H/A diff OOB high
        let result_b = BoxScoreGenerator::from_properties(
            13.37_f64,
            0.5_f64
        );
        let expected_b: Result<BoxScoreGenerator, String> = Err(
            String::from("Home offense / away defense noralized skill differential not in range [0, 1]: 13.37")
        );
        assert_eq!(
            result_b,
            expected_b
        );

        // Test A/H diff OOB low
        let result_c = BoxScoreGenerator::from_properties(
            0.5_f64,
            -3.3_f64
        );
        let expected_c: Result<BoxScoreGenerator, String> = Err(
            String::from("Away offense / home defense noralized skill differential not in range [0, 1]: -3.3")
        );
        assert_eq!(
            result_c,
            expected_c
        );

        // Test A/H diff OOB high
        let result_d = BoxScoreGenerator::from_properties(
            0.5_f64,
            4.4_f64
        );
        let expected_d: Result<BoxScoreGenerator, String> = Err(
            String::from("Away offense / home defense noralized skill differential not in range [0, 1]: 4.4")
        );
        assert_eq!(
            result_d,
            expected_d
        );

        // Test both diffs in valid range
        let result_e = BoxScoreGenerator::from_properties(
            0.0001_f64,
            0.9999_f64
        );
        let expected_e: Result<BoxScoreGenerator, String> = Ok(
            BoxScoreGenerator{
                home_off_away_def_norm_diff: 0.0001_f64,
                away_off_home_def_norm_diff: 0.9999_f64
            }
        );
        assert_eq!(
            result_e,
            expected_e
        );
    }

    #[test]
    fn test_box_score_gen_setters() {
        // Initialize a BoxScoreGenerator
        let mut box_score_gen = BoxScoreGenerator::new();

        // Test H/A diff OOB and in range
        let result_a = box_score_gen.set_home_off_away_def_norm_diff(-6.9_f64);
        let expected_a: Result<(), String> = Err(
            String::from("Home offense / away defense noralized skill differential not in range [0, 1]: -6.9")
        );
        assert_eq!(result_a, expected_a);
        let result_b = box_score_gen.set_home_off_away_def_norm_diff(7.7123456_f64);
        let expected_b: Result<(), String> = Err(
            String::from("Home offense / away defense noralized skill differential not in range [0, 1]: 7.7123456")
        );
        assert_eq!(result_b, expected_b);
        let result_c = box_score_gen.set_home_off_away_def_norm_diff(0.25_f64);
        let expected_c: Result<(), String> = Ok(());
        assert_eq!(result_c, expected_c);

        // Test A/H diff OOB and in range
        let result_d = box_score_gen.set_away_off_home_def_norm_diff(-5.567_f64);
        let expected_d: Result<(), String> = Err(
            String::from("Away offense / home defense noralized skill differential not in range [0, 1]: -5.567")
        );
        assert_eq!(result_d, expected_d);
        let result_e = box_score_gen.set_away_off_home_def_norm_diff(400.0001_f64);
        let expected_e: Result<(), String> = Err(
            String::from("Away offense / home defense noralized skill differential not in range [0, 1]: 400.0001")
        );
        assert_eq!(result_e, expected_e);
        let result_f = box_score_gen.set_away_off_home_def_norm_diff(0.25_f64);
        let expected_f: Result<(), String> = Ok(());
        assert_eq!(result_f, expected_f);
    }

    #[test]
    fn test_box_score_gen() {
        // Initialize a box score generator and an rng
        let mut rng = rand::thread_rng();
        let box_score_gen = BoxScoreGenerator::new();

        // Generate and validate a few box scores
        let (home_a, away_a) = box_score_gen.gen(&mut rng);
        let (home_b, away_b) = box_score_gen.gen(&mut rng);
        let (home_c, away_c) = box_score_gen.gen(&mut rng);
        let (home_d, away_d) = box_score_gen.gen(&mut rng);
        let (home_e, away_e) = box_score_gen.gen(&mut rng);
        assert!(home_a > 0);
        assert!(home_b > 0);
        assert!(home_c > 0);
        assert!(home_d > 0);
        assert!(home_e > 0);
        assert!(away_a > 0);
        assert!(away_b > 0);
        assert!(away_c > 0);
        assert!(away_d > 0);
        assert!(away_e > 0);
    }
}
