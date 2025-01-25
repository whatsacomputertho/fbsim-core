use rand::Rng;
use rand_distr::{Normal, Distribution};

use crate::boxscore::BoxScore;
use crate::team::FootballTeam;

// Home score simulator model weights
const H_MEAN_INTERCEPT: f64 = 23.14578315_f64;
const H_MEAN_COEF: f64 = 10.9716991_f64;
const H_STD_INTERCEPT: f64 = 7.64006156_f64;
const H_STD_COEF_1: f64 = 5.72612946_f64;
const H_STD_COEF_2: f64 = -4.29283414_f64;

// Away score simulator model weights
const A_MEAN_INTERCEPT: f64 = 22.14952374_f64;
const A_MEAN_COEF: f64 = 8.92113289_f64;
const A_STD_INTERCEPT: f64 = 6.47638621_f64;
const A_STD_COEF_1: f64 = 8.00861267_f64;
const A_STD_COEF_2: f64 = -5.589282_f64;

/// # `BoxScoreSimulator` struct
///
/// A `BoxScoreSimulator` generates an american football box score
/// given the normalized skill differential (in range [0, 1]) of the
/// home offense and the away defense, and vice versa, the away
/// offense and the home defense.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BoxScoreSimulator;

impl BoxScoreSimulator {
    /// Constructor for the `BoxScoreSimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::sim::BoxScoreSimulator;
    ///
    /// let my_box_score_gen = BoxScoreSimulator::new();
    /// ```
    pub fn new() -> BoxScoreSimulator {
        BoxScoreSimulator{}
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

    /// Simulates a game by generating a box score result
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::boxscore::BoxScore;
    /// use fbsim_core::sim::BoxScoreSimulator;
    /// use fbsim_core::team::FootballTeam;
    ///
    /// let home = FootballTeam::new();
    /// let away = FootballTeam::new();
    /// let sim = BoxScoreSimulator::new();
    /// let mut rng = rand::thread_rng();
    /// let score = sim.sim(&home, &away, &mut rng).unwrap();
    /// println!("{}", score);
    /// ```
    pub fn sim(&self, home_team: &FootballTeam, away_team: &FootballTeam, rng: &mut impl Rng) -> Result<BoxScore, String> {
        // Calculate the normalized skill differentials
        let ha_norm_diff: f64 = (home_team.offense_overall() - away_team.defense_overall() + 100) as f64 / 200_f64;
        let ah_norm_diff: f64 = (away_team.offense_overall() - home_team.defense_overall() + 100) as f64 / 200_f64;

        // Generate the box score, return error if error is encountered
        let (home_score, away_score): (i32, i32) = match self.gen_score(ha_norm_diff, ah_norm_diff, rng) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        // Instantiate as a BoxScore and return
        let box_score: BoxScore = BoxScore::from_properties(
            home_team.name(),
            home_score,
            away_team.name(),
            away_score
        ).unwrap();
        Ok(box_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_score_gen() {
        // Initialize a box score simulator and an rng
        let mut rng = rand::thread_rng();
        let sim = BoxScoreSimulator::new();

        // Initialize two football teams
        let home = FootballTeam::new();
        let away = FootballTeam::new();

        // Generate and validate a few box scores
        let score_a = sim.sim(&home, &away, &mut rng).unwrap();
        let score_b = sim.sim(&home, &away, &mut rng).unwrap();
        let score_c = sim.sim(&home, &away, &mut rng).unwrap();
        let score_d = sim.sim(&home, &away, &mut rng).unwrap();
        let score_e = sim.sim(&home, &away, &mut rng).unwrap();
        assert!(score_a.home_score() > 1);
        assert!(score_a.away_score() > 1);
        assert!(score_b.home_score() > 1);
        assert!(score_b.away_score() > 1);
        assert!(score_c.home_score() > 1);
        assert!(score_c.away_score() > 1);
        assert!(score_d.home_score() > 1);
        assert!(score_d.away_score() > 1);
        assert!(score_e.home_score() > 1);
        assert!(score_e.away_score() > 1);
    }
}
