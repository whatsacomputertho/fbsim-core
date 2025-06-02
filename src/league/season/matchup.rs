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

    /// Borrow the home team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let home_id = my_matchup.home_team();
    /// ```
    pub fn home_team(&self) -> &usize {
        &self.home_team
    }

    /// Borrow the away team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let away_id = my_matchup.away_team();
    /// ```
    pub fn away_team(&self) -> &usize {
        &self.away_team
    }

    /// Borrow the home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let home_score = my_matchup.home_score();
    /// ```
    pub fn home_score(&self) -> &usize {
        &self.home_score
    }

    /// Mutably borrow the home score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut home_score = my_matchup.home_score_mut();
    /// ```
    pub fn home_score_mut(&mut self) -> &mut usize {
        &mut self.home_score
    }

    /// Borrow the away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let away_score = my_matchup.away_score();
    /// ```
    pub fn away_score(&self) -> &usize {
        &self.away_score
    }

    /// Mutably borrow the away score
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut away_score = my_matchup.away_score_mut();
    /// ```
    pub fn away_score_mut(&mut self) -> &mut usize {
        &mut self.away_score
    }

    /// Borrow the complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let complete = my_matchup.complete();
    /// ```
    pub fn complete(&self) -> &bool {
        &self.complete
    }

    /// Mutably borrow the complete property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
    ///
    /// let mut my_matchup = LeagueSeasonMatchup::new(0, 1);
    /// let mut complete = my_matchup.complete_mut();
    /// ```
    pub fn complete_mut(&mut self) -> &mut bool {
        &mut self.complete
    }
}
