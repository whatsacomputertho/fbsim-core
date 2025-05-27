use serde::{Serialize, Deserialize};

/// # `LeagueSeasonTeam` struct
///
/// A `LeagueSeasonTeam` represents a team during a season of a football leauge
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonTeam {
    name: String,
    logo: String,
    offense_overall: usize,
    defense_overall: usize
}

impl LeagueSeasonTeam {
    /// Constructor for the `LeagueSeasonTeam` struct in which the league team
    /// reference is given.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// ```
    pub fn new(name: String, logo: String, offense_overall: usize, defense_overall: usize) -> LeagueSeasonTeam {
        LeagueSeasonTeam{
            name: name, 
            logo: logo,
            offense_overall: offense_overall,
            defense_overall: defense_overall
        }
    }

    /// Borrow the season team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let team_name = my_season_team.name();
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Mutably borrow the season team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut team_name = my_season_team.name_mut();
    /// ```
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Borrow the season team logo
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let team_logo = my_season_team.logo();
    /// ```
    pub fn logo(&self) -> &String {
        &self.logo
    }

    /// Mutably borrow the season team logo
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut team_logo = my_season_team.logo_mut();
    /// ```
    pub fn logo_mut(&mut self) -> &mut String {
        &mut self.logo
    }

    /// Borrow the season team offensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let offense_overall = my_season_team.offense_overall();
    /// ```
    pub fn offense_overall(&self) -> &usize {
        &self.offense_overall
    }

    /// Mutably borrow the season team offensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut offense_overall = my_season_team.offense_overall_mut();
    /// ```
    pub fn offense_overall_mut(&mut self) -> &mut usize {
        &mut self.offense_overall
    }

    /// Borrow the season team defensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let defense_overall = my_season_team.defense_overall();
    /// ```
    pub fn defense_overall(&self) -> &usize {
        &self.defense_overall
    }

    /// Mutably borrow the season team defensive overall value
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let mut my_season_team = LeagueSeasonTeam::new("My Team".to_string(), "".to_string(), 50, 50);
    /// let mut defense_overall = my_season_team.defense_overall_mut();
    /// ```
    pub fn defense_overall_mut(&mut self) -> &mut usize {
        &mut self.defense_overall
    }
}
