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
