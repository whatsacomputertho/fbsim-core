use crate::boxscore::BoxScore;
use crate::gen::BoxScoreGenerator;
use crate::team::FootballTeam;

use rand::Rng;

/// `BoxScoreSimulator` struct
///
/// The `BoxScoreSimulator` struct simulates a game by simply
/// generating a box score result
pub struct BoxScoreSimulator;

impl BoxScoreSimulator {
    /// Constructor for the `BoxScoreSimulator` struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::sim::BoxScoreSimulator;
    ///
    /// let my_sim = BoxScoreSimulator::new();
    /// ```
    pub fn new() -> BoxScoreSimulator {
        BoxScoreSimulator{}
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
    /// let score = sim.sim(&home, &away, &mut rng);
    /// println!("{}", score);
    /// ```
    pub fn sim(&self, home_team: &FootballTeam, away_team: &FootballTeam, rng: &mut impl Rng) -> BoxScore {
        // Calculate the normalized skill differentials
        let ha_norm_diff: f64 = ((home_team.offense_overall() - away_team.defense_overall() + 100) / 200) as f64;
        let ah_norm_diff: f64 = ((away_team.offense_overall() - home_team.defense_overall() + 100) / 200) as f64;

        // Generate the box score
        let box_score_gen: BoxScoreGenerator = BoxScoreGenerator::from_properties(
            ha_norm_diff,
            ah_norm_diff
        ).unwrap();
        let (home_score, away_score): (i32, i32) = box_score_gen.gen(rng);

        // Instantiate as a BoxScore and return
        let box_score: BoxScore = BoxScore::from_properties(
            home_team.name(),
            home_score,
            away_team.name(),
            away_score
        ).unwrap();
        box_score
    }
}
