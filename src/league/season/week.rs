use serde::{Serialize, Deserialize};

use crate::league::season::matchup::LeagueSeasonMatchup;

/// # `LeagueSeasonWeek` struct
///
/// A `LeagueSeasonWeek` represents a week of a football season
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonWeek {
    matchups: Vec<LeagueSeasonMatchup>
}
