use serde::{Serialize, Deserialize, Deserializer};

use crate::team::DEFAULT_TEAM_NAME;

/// # `LeagueSeasonTeamRaw` struct
///
/// A `LeagueSeasonTeamRaw` is a `LeagueSeasonTeam` before it has been
/// fully validated and deserialized
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueSeasonTeamRaw {
    name: String,
    logo: String,
    offense_overall: usize,
    defense_overall: usize
}

impl LeagueSeasonTeamRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure the offense and defense overall are in range
        if !self.offense_overall <= 100_usize {
            return Err(
                format!(
                    "Offense overall not in range [0, 100]: {}",
                    self.offense_overall
                )
            )
        }
        if !self.defense_overall <= 100_usize {
            return Err(
                format!(
                    "Defense overall not in range [0, 100]: {}",
                    self.defense_overall
                )
            )
        }
        Ok(())
    }
}

/// # `LeagueSeasonTeam` struct
///
/// A `LeagueSeasonTeam` represents a team during a season of a football leauge
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct LeagueSeasonTeam {
    name: String,
    logo: String,
    offense_overall: usize,
    defense_overall: usize
}

impl TryFrom<LeagueSeasonTeamRaw> for LeagueSeasonTeam {
    type Error = String;

    fn try_from(item: LeagueSeasonTeamRaw) -> Result<Self, Self::Error> {
        // Validate the raw season team
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            LeagueSeasonTeam{
                name: item.name,
                logo: item.logo,
                offense_overall: item.offense_overall,
                defense_overall: item.defense_overall
            }
        )
    }
}

impl<'de> Deserialize<'de> for LeagueSeasonTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = LeagueSeasonTeamRaw::deserialize(deserializer)?;
        LeagueSeasonTeam::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl LeagueSeasonTeam {
    /// Constructor for the `LeagueSeasonTeam` struct in which the properties
    /// are zeroed
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new();
    /// ```
    pub fn new() -> LeagueSeasonTeam {
        LeagueSeasonTeam{
            name: String::from(DEFAULT_TEAM_NAME), 
            logo: "".to_string(), // TODO: Default logo
            offense_overall: 50_usize,
            defense_overall: 50_usize
        }
    }

    /// Constructor for the `LeagueSeasonTeam` struct in which the properties
    /// are given and validated
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team: Result<LeagueSeasonTeam, String> = LeagueSeasonTeam::from_properties(
    ///     "My Team".to_string(),
    ///     "".to_string(),
    ///     50,
    ///     50
    /// );
    /// ```
    pub fn from_properties(name: String, logo: String, offense_overall: usize, defense_overall: usize) -> Result<LeagueSeasonTeam, String> {
        let raw = LeagueSeasonTeamRaw{
            name: name, 
            logo: logo,
            offense_overall: offense_overall,
            defense_overall: defense_overall
        };
        LeagueSeasonTeam::try_from(raw)
    }

    /// Borrow the season team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// let my_season_team = LeagueSeasonTeam::new();
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
    /// let mut my_season_team = LeagueSeasonTeam::new();
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
    /// let my_season_team = LeagueSeasonTeam::new();
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
    /// let mut my_season_team = LeagueSeasonTeam::new();
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
    /// let my_season_team = LeagueSeasonTeam::new();
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
    /// let mut my_season_team = LeagueSeasonTeam::new();
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
    /// let my_season_team = LeagueSeasonTeam::new();
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
    /// let mut my_season_team = LeagueSeasonTeam::new();
    /// let mut defense_overall = my_season_team.defense_overall_mut();
    /// ```
    pub fn defense_overall_mut(&mut self) -> &mut usize {
        &mut self.defense_overall
    }
}
