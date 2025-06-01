use serde::{Serialize, Deserialize};

/// # `LeagueSeasonMatchup` struct
///
/// A `LeagueSeasonMatchup` represents a matchup during a week of a football season
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonMatchup {
    home_team: usize,
    away_team: usize,
    home_score: usize,
    away_score: usize,
    complete: bool
}

impl LeagueSeasonMatchup {
    /// Constructor for the LeagueSeasonMatchup struct in which the home and
    /// away team IDs are given, and the score & completion status is zeroed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// ```
    pub fn new(home_team: usize, away_team: usize) -> LeagueSeasonMatchup {
        LeagueSeasonMatchup {
            home_team: home_team,
            away_team: away_team,
            home_score: 0,
            away_score: 0,
            complete: false
        }
    }
}
