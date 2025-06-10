pub mod season;
pub mod team;

use crate::league::team::LeagueTeam;
use crate::league::season::LeagueSeason;
use crate::league::season::team::LeagueSeasonTeam;

use std::collections::BTreeMap;

use rand::Rng;
use serde::{Serialize, Deserialize, Deserializer};

/// # `LeagueRaw` struct
///
/// A `LeagueRaw` represents a league that is freshly deserialized from JSON
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct LeagueRaw {
    pub teams: BTreeMap<usize, LeagueTeam>,
    pub current_season: Option<LeagueSeason>,
    pub seasons: Vec<LeagueSeason>
}

impl LeagueRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure the IDs in the current season map to matching league team IDs
        match &self.current_season {
            Some(season) => {
                for (id, team) in season.teams().iter() {
                    if !self.teams.contains_key(&id) {
                        return Err(
                            format!(
                                "Season {} contains team {} with nonexistent ID: {}",
                                season.year(), team.name(), id
                            )
                        )
                    }
                }
            },
            None => ()
        }

        // Ensure the IDs in all past seasons map to matching league team IDs
        for season in self.seasons.iter() {
            for (id, team) in season.teams().iter() {
                if !self.teams.contains_key(&id) {
                    return Err(
                        format!(
                            "Season {} contains team {} with nonexistent ID: {}",
                            season.year(), team.name(), id
                        )
                    )
                }
            }
        }
        Ok(())
    }
}

/// # `League` struct
///
/// A `League` represents a football league. It contains a vector of teams in
/// the league as `LeagueTeam` objects. It also contains the season that is
/// currently in-progress, and it contains a vector of past seasons
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct League {
    teams: BTreeMap<usize, LeagueTeam>,
    current_season: Option<LeagueSeason>,
    seasons: Vec<LeagueSeason>
}

impl TryFrom<LeagueRaw> for League {
    type Error = String;

    fn try_from(item: LeagueRaw) -> Result<Self, Self::Error> {
        // Validate the raw league
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            League{
                teams: item.teams,
                current_season: item.current_season,
                seasons: item.seasons
            }
        )
    }
}

impl<'de> Deserialize<'de> for League {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = LeagueRaw::deserialize(deserializer)?;
        League::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl League {
    /// Constructor for the `League` struct in which the vec of league
    /// teams is empty
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// let my_league = League::new();
    /// ```
    pub fn new() -> League {
        League{
            teams: BTreeMap::new(),
            current_season: None,
            seasons: Vec::new()
        }
    }

    /// Adds a `LeagueTeam` to a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// let mut my_league = League::new();
    /// my_league.add_team();
    /// ```
    pub fn add_team(&mut self) -> () {
        // Get the last item in the BTreeMap, which is auto-sorted by ID
        if let Some((&max_id, _)) = self.teams.iter().next_back() {
            // The list is non-empty and has a max ID
            self.teams.insert(max_id + 1, LeagueTeam::new());
        } else {
            // The list is empty
            self.teams.insert(0, LeagueTeam::new());
        }
    }

    /// Borrows the BTreeMap of teams immutably
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the BTreeMap of LeagueTeams
    /// let my_teams = my_league.teams();
    /// ```
    pub fn teams(&self) -> &BTreeMap<usize, LeagueTeam> {
        &self.teams
    }

    /// Borrows an immutable `LeagueTeam` from a `League` given the team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a few LeagueTeams to the League
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Get the LeagueTeam with ID 2
    /// let my_team = my_league.team(2);
    /// ```
    pub fn team(&self, id: usize) -> Option<&LeagueTeam> {
        self.teams.get(&id)
    }

    /// Borrows a season from a `League` identified by its year
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Create a new season in the league
    /// let _ = my_league.add_season();
    ///
    /// // Borrow the past seasons from the League
    /// let my_season = my_league.season(2025);
    /// ```
    pub fn season(&self, year: usize) -> Option<&LeagueSeason> {
        // If the year corresponds to the current season, return it
        if let Some(season) = self.current_season() {
            if *season.year() == year {
                return Some(season);
            }
        }

        // Otherwise search for it in the past seasons
        for season in self.seasons().iter() {
            if *season.year() == year {
                return Some(season);
            }
        }
        return None
    }

    /// Borrows the past seasons from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let my_league = League::new();
    ///
    /// // Borrow the past seasons from the League
    /// let past_seasons = my_league.seasons();
    /// ```
    pub fn seasons(&self) -> &Vec<LeagueSeason> {
        &self.seasons
    }

    /// Borrows the current season from a `League`
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::LeagueSeason;
    ///
    /// // Instantiate a new League
    /// let my_league = League::new();
    ///
    /// // Borrow the current season from the League
    /// let my_season: &Option<LeagueSeason> = my_league.current_season();
    /// ```
    pub fn current_season(&self) -> &Option<LeagueSeason> {
        &self.current_season
    }

    /// Gets the most recent year among the completed seasons
    fn most_recent_year(&self) -> usize {
        let mut most_recent_year = 0_usize;
        let seasons = self.seasons();
        for season in seasons.iter() {
            let season_year = season.year();
            if *season_year > most_recent_year {
                most_recent_year = season_year.clone();
            }
        }
        most_recent_year
    }

    /// Creates a new `LeagueSeason` and archives the current `LeagueSeason`
    /// if the current `LeagueSeason` is complete
    ///
    /// ### Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use fbsim_core::league::League;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    /// ```
    pub fn add_season(&mut self) -> Result<(), String> {
        // Check if the current season exists
        if let Some(season) = &mut self.current_season {
            // If so, then check if the season is complete
            if season.complete() {
                // If the season is complete, archive and create new season
                let most_recent_year = season.year();
                let mut new_season = LeagueSeason::new();
                let new_year = new_season.year_mut();
                *new_year = most_recent_year + 1;
                let old_season = season.clone();
                *season = new_season;
                self.seasons.push(old_season);
                return Ok(());
            }

            // If the season is not complete, then error
            return Err(
                format!(
                    "Cannot create new season: {}",
                    "Current season still in progress"
                )
            );
        }

        // Create a new league season
        let mut new_season = LeagueSeason::new();

        // If the past seasons list is empty then stick with the default year
        if self.seasons.len() == 0 {
            self.current_season = Some(new_season);
            return Ok(());
        }

        // If the past seasons list is nonempty then update the year
        let most_recent_year = self.most_recent_year();
        let new_year = new_season.year_mut();
        *new_year = most_recent_year + 1;
        self.current_season = Some(new_season);
        return Ok(());
    }

    /// Adds a `LeagueSeasonTeam` to a `LeagueSeason`, and corresponds the
    /// `LeagueSeasonTeam` to the `LeagueTeam` with the given team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add a new team to the new league
    /// my_league.add_team();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    ///
    /// // Add a new season team to the new season corresponding to the new team
    /// my_league.add_season_team(0, LeagueSeasonTeam::new());
    /// ```
    pub fn add_season_team(&mut self, id: usize, team: LeagueSeasonTeam) -> Result<(), String> {
        // Ensure the given team ID exists in the league
        if !self.teams.contains_key(&id) {
            return Err(format!("No team with ID: {}", id));
        }
        
        // Add the team to the current season
        // Teams can only be added to the current season since all past seasons
        // must have already completed in order to be archived in that list
        match &mut self.current_season {
            Some(ref mut season) => return season.add_team(id, team),
            None => Err("No current season to which to add a new team".to_string()),
        }
    }

    /// Generate a schedule for the current season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add 4 new teams to the new league
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    ///
    /// // Add 4 new season teams to the new season
    /// my_league.add_season_team(0, LeagueSeasonTeam::new());
    /// my_league.add_season_team(1, LeagueSeasonTeam::new());
    /// my_league.add_season_team(2, LeagueSeasonTeam::new());
    /// my_league.add_season_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league.generate_schedule(None, &mut rng);
    /// ```
    pub fn generate_schedule(&mut self, weeks: Option<usize>, rng: &mut impl Rng) -> Result<(), String> {
        // Generate a schedule for the current season if it exists
        match &mut self.current_season {
            Some(ref mut season) => season.generate_schedule(weeks, rng), // Return the result
            None => Err("No current season to simulate".to_string()),
        }
    }

    /// Simulate the entire current season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add 4 new teams to the new league
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    ///
    /// // Add 4 new season teams to the new season
    /// my_league.add_season_team(0, LeagueSeasonTeam::new());
    /// my_league.add_season_team(1, LeagueSeasonTeam::new());
    /// my_league.add_season_team(2, LeagueSeasonTeam::new());
    /// my_league.add_season_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league.generate_schedule(None, &mut rng);
    ///
    /// // Simulate the season
    /// my_league.sim(&mut rng);
    /// ```
    pub fn sim(&mut self, rng: &mut impl Rng) -> Result<(), String> {
        // Simulate the current season if it exists, return the result
        match &mut self.current_season {
            Some(ref mut season) => season.sim(rng),
            None => Err("No current season to simulate".to_string()),
        }
    }

    /// Simulate a week of the current season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add 4 new teams to the new league
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    ///
    /// // Add 4 new season teams to the new season
    /// my_league.add_season_team(0, LeagueSeasonTeam::new());
    /// my_league.add_season_team(1, LeagueSeasonTeam::new());
    /// my_league.add_season_team(2, LeagueSeasonTeam::new());
    /// my_league.add_season_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league.generate_schedule(None, &mut rng);
    ///
    /// // Simulate the first week of the season
    /// my_league.sim_week(0, &mut rng);
    /// ```
    pub fn sim_week(&mut self, week: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Simulate a week of the current season if it exists, return the result
        match &mut self.current_season {
            Some(ref mut season) => season.sim_week(week, rng),
            None => Err("No current season to simulate".to_string()),
        }
    }

    /// Simulate a week of the current season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::League;
    /// use fbsim_core::league::season::team::LeagueSeasonTeam;
    ///
    /// // Instantiate a new League
    /// let mut my_league = League::new();
    ///
    /// // Add 4 new teams to the new league
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    /// my_league.add_team();
    ///
    /// // Create a new season for the new League
    /// let res = my_league.add_season();
    ///
    /// // Add 4 new season teams to the new season
    /// my_league.add_season_team(0, LeagueSeasonTeam::new());
    /// my_league.add_season_team(1, LeagueSeasonTeam::new());
    /// my_league.add_season_team(2, LeagueSeasonTeam::new());
    /// my_league.add_season_team(3, LeagueSeasonTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league.generate_schedule(None, &mut rng);
    ///
    /// // Simulate the first week of the season
    /// my_league.sim_week(0, &mut rng);
    /// ```
    pub fn sim_matchup(&mut self, week: usize, matchup: usize, rng: &mut impl Rng) -> Result<(), String> {
        // Simulate a matchup from the current season if it exists, return the result
        match &mut self.current_season {
            Some(ref mut season) => season.sim_matchup(week, matchup, rng),
            None => Err("No current season to simulate".to_string()),
        }
    }
}
