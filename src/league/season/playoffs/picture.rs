#![doc = include_str!("../../../../docs/league/season/playoffs/picture.md")]
use std::collections::BTreeMap;

#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::league::matchup::LeagueTeamRecord;
use crate::league::season::LeagueSeason;

/// # `PlayoffStatus` enum
///
/// Represents a team's playoff qualification status during an ongoing season
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub enum PlayoffStatus {
    /// Team has clinched the #1 seed
    ClinchedTopSeed,
    /// Team has clinched a playoff berth (with current seed)
    ClinchedPlayoffs { current_seed: usize },
    /// Team is currently in playoff position but hasn't clinched
    InPlayoffPosition { current_seed: usize },
    /// Team is not in playoff position but still mathematically alive
    #[default] InTheHunt,
    /// Team cannot make playoffs regardless of remaining outcomes
    Eliminated,
}

/// # `PlayoffPictureEntry` struct
///
/// Represents a single team's entry in the playoff picture
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct PlayoffPictureEntry {
    team_id: usize,
    team_name: String,
    current_record: LeagueTeamRecord,
    status: PlayoffStatus,
    games_back: f64,
    remaining_games: usize,
    magic_number: Option<usize>,
}

impl PlayoffPictureEntry {
    /// Initialize a new PlayoffPictureEntry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    ///
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// ```
    pub fn new() -> PlayoffPictureEntry {
        PlayoffPictureEntry::default()
    }

    /// Get the entry's team ID
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    ///
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(my_playoff_picture_entry.team_id() == 0);
    /// ```
    pub fn team_id(&self) -> usize {
        self.team_id
    }

    /// Get the entry's team name
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    ///
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(my_playoff_picture_entry.team_name() == "".to_string());
    /// ```
    pub fn team_name(&self) -> &str {
        &self.team_name
    }

    /// Get the current record of the team in the entry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::matchup::LeagueTeamRecord;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    ///
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(*my_playoff_picture_entry.current_record() == LeagueTeamRecord::new());
    /// ```
    pub fn current_record(&self) -> &LeagueTeamRecord {
        &self.current_record
    }

    /// Get the playoff status of the team in the entry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::{PlayoffPictureEntry, PlayoffStatus};
    ///
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(*my_playoff_picture_entry.status() == PlayoffStatus::InTheHunt);
    /// ```
    pub fn status(&self) -> &PlayoffStatus {
        &self.status
    }

    /// Get games back from the playoff cutoff for the team in the entry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    /// 
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(my_playoff_picture_entry.games_back() == 0.0);
    /// ```
    pub fn games_back(&self) -> f64 {
        self.games_back
    }

    /// Get remaining games in the season for the team in the entry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    /// 
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(my_playoff_picture_entry.remaining_games() == 0);
    /// ```
    pub fn remaining_games(&self) -> usize {
        self.remaining_games
    }

    /// Get the magic number (wins needed to clinch), if applicable
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    /// 
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// let magic_number = my_playoff_picture_entry.magic_number();
    /// assert!(magic_number.is_none());
    /// ```
    pub fn magic_number(&self) -> Option<usize> {
        self.magic_number
    }

    /// Check if the team has clinched a playoff spot
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    /// 
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(!my_playoff_picture_entry.is_clinched());
    /// ```
    pub fn is_clinched(&self) -> bool {
        matches!(
            self.status,
            PlayoffStatus::ClinchedTopSeed | PlayoffStatus::ClinchedPlayoffs { .. }
        )
    }

    /// Check if the team has been eliminated
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPictureEntry;
    /// 
    /// let my_playoff_picture_entry = PlayoffPictureEntry::new();
    /// assert!(!my_playoff_picture_entry.is_eliminated());
    /// ```
    pub fn is_eliminated(&self) -> bool {
        matches!(self.status, PlayoffStatus::Eliminated)
    }
}

/// Internal helper struct for tracking potential record ranges
#[derive(Clone, Debug)]
struct RecordBounds {
    team_id: usize,
    current_wins: usize,
    #[allow(dead_code)]
    current_losses: usize,
    #[allow(dead_code)]
    current_ties: usize,
    remaining_games: usize,
    #[allow(dead_code)]
    total_games: usize,
    max_possible_wins: usize,
    max_possible_win_pct: f64,
    min_possible_wins: usize,
    min_possible_win_pct: f64,
}

impl RecordBounds {
    fn from_record(team_id: usize, record: &LeagueTeamRecord, remaining_games: usize, total_games: usize) -> Self {
        let current_wins = *record.wins();
        let current_losses = *record.losses();
        let current_ties = *record.ties();

        let max_possible_wins = current_wins + remaining_games;
        let min_possible_wins = current_wins;

        // Win percentage = (wins + 0.5*ties) / total_games
        let max_possible_win_pct = if total_games > 0 {
            (max_possible_wins as f64 + 0.5 * current_ties as f64) / total_games as f64
        } else {
            0.0
        };
        let min_possible_win_pct = if total_games > 0 {
            (min_possible_wins as f64 + 0.5 * current_ties as f64) / total_games as f64
        } else {
            0.0
        };

        RecordBounds {
            team_id,
            current_wins,
            current_losses,
            current_ties,
            remaining_games,
            total_games,
            max_possible_wins,
            max_possible_win_pct,
            min_possible_wins,
            min_possible_win_pct,
        }
    }
}

/// # `PlayoffPictureOptions` struct
///
/// Options for configuring how the playoff picture is generated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct PlayoffPictureOptions {
    /// If `Some(true)`, force conference-based playoff picture.
    /// If `Some(false)`, force flat (non-conference) playoff picture.
    /// If `None`, auto-detect: use conferences if the season has multiple conferences.
    pub by_conference: Option<bool>,
    /// If true, division winners are guaranteed a playoff berth (conference mode only)
    pub division_winners_guaranteed: bool,
}

/// # `PlayoffPicture` struct
///
/// Represents the complete playoff picture for a season, showing the
/// qualification status of all teams
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayoffPicture {
    num_playoff_teams: usize,
    entries: Vec<PlayoffPictureEntry>,
    games_remaining_in_season: usize,
}

impl PlayoffPicture {
    /// Create a playoff picture from a season
    ///
    /// When the season has multiple conferences, the playoff picture is organized
    /// by conference by default (with `num_playoff_teams` interpreted as the number
    /// of playoff teams *per conference*). This behavior can be overridden via
    /// `PlayoffPictureOptions`.
    ///
    /// ### Arguments
    /// * `season` - The league season to analyze
    /// * `num_playoff_teams` - Number of teams that make the playoffs. In conference
    ///   mode this is the number of teams *per conference*.
    /// * `options` - Optional configuration; pass `None` for defaults
    ///
    /// ### Returns
    /// * `Ok(PlayoffPicture)` - The current playoff picture
    /// * `Err(String)` - If the season hasn't started or parameters are invalid
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None);
    /// assert!(picture.is_ok());
    /// ```
    pub fn from_season(
        season: &LeagueSeason,
        num_playoff_teams: usize,
        options: Option<PlayoffPictureOptions>,
    ) -> Result<Self, String> {
        let opts = options.unwrap_or_default();

        let use_conferences = match opts.by_conference {
            Some(v) => v,
            None => season.conferences().len() > 1,
        };

        if use_conferences {
            Self::conference_playoff_picture(
                season,
                num_playoff_teams,
                opts.division_winners_guaranteed,
            )
        } else {
            Self::non_conference_playoff_picture(season, num_playoff_teams)
        }
    }

    /// Build a playoff picture using overall league standings (no conference separation)
    fn non_conference_playoff_picture(season: &LeagueSeason, num_playoff_teams: usize) -> Result<Self, String> {
        let total_teams = season.teams().len();

        // Validate parameters
        if num_playoff_teams == 0 {
            return Err("Number of playoff teams must be at least 1".to_string());
        }
        if num_playoff_teams > total_teams {
            return Err(format!(
                "Number of playoff teams ({}) cannot exceed total teams ({})",
                num_playoff_teams, total_teams
            ));
        }
        if season.weeks().is_empty() {
            return Err("Season has no schedule".to_string());
        }

        // Get current standings
        let standings = season.standings();

        // Calculate total games per team (assumes all teams play same number of games)
        let total_games = season.weeks().len();

        // Compute remaining games for each team
        let mut team_remaining_games: BTreeMap<usize, usize> = BTreeMap::new();
        let mut games_remaining_in_season = 0;

        for week in season.weeks().iter() {
            for matchup in week.matchups().iter() {
                if !matchup.context().game_over() {
                    games_remaining_in_season += 1;
                    *team_remaining_games.entry(*matchup.home_team()).or_insert(0) += 1;
                    *team_remaining_games.entry(*matchup.away_team()).or_insert(0) += 1;
                }
            }
        }

        // Compute record bounds for all teams
        let bounds: Vec<RecordBounds> = standings
            .iter()
            .map(|(team_id, record)| {
                let remaining = *team_remaining_games.get(team_id).unwrap_or(&0);
                RecordBounds::from_record(*team_id, record, remaining, total_games)
            })
            .collect();

        // Build entries for each team
        let mut entries = Vec::with_capacity(total_teams);

        for (position, (team_id, record)) in standings.iter().enumerate() {
            let team_name = season
                .teams()
                .get(team_id)
                .map(|t| t.name().to_string())
                .unwrap_or_else(|| format!("Team {}", team_id));

            let remaining = *team_remaining_games.get(team_id).unwrap_or(&0);
            let in_playoff_position = position < num_playoff_teams;

            // Calculate games back
            let games_back = if in_playoff_position {
                0.0
            } else {
                Self::compute_games_back(&standings, position, num_playoff_teams)
            };

            // Determine status
            let status = Self::compute_status(
                *team_id,
                position,
                &bounds,
                num_playoff_teams,
            );

            // Calculate magic number for teams in contention
            let magic_number = if matches!(status, PlayoffStatus::Eliminated) {
                None
            } else {
                Self::compute_magic_number(*team_id, &bounds, num_playoff_teams)
            };

            entries.push(PlayoffPictureEntry {
                team_id: *team_id,
                team_name,
                current_record: record.clone(),
                status,
                games_back,
                remaining_games: remaining,
                magic_number,
            });
        }

        Ok(PlayoffPicture {
            num_playoff_teams,
            entries,
            games_remaining_in_season,
        })
    }

    /// Build a playoff picture organized by conference
    fn conference_playoff_picture(
        season: &LeagueSeason,
        playoff_teams_per_conference: usize,
        division_winners_guaranteed: bool,
    ) -> Result<Self, String> {
        // Validate that we have conferences
        if season.conferences().is_empty() {
            return Err("Season has no conferences defined".to_string());
        }

        if season.weeks().is_empty() {
            return Err("Season has no schedule".to_string());
        }

        if playoff_teams_per_conference == 0 {
            return Err("Number of playoff teams per conference must be at least 1".to_string());
        }

        // Validate that playoff_teams_per_conference does not exceed the
        // smallest conference size
        let min_conference_size = season
            .conferences()
            .iter()
            .map(|c| c.num_teams())
            .min()
            .unwrap_or(0);

        if playoff_teams_per_conference > min_conference_size {
            return Err(format!(
                "Playoff teams per conference ({}) exceeds the smallest conference size ({})",
                playoff_teams_per_conference, min_conference_size
            ));
        }

        let num_conferences = season.conferences().len();
        let total_playoff_teams = playoff_teams_per_conference * num_conferences;

        // Compute remaining games for each team
        let mut team_remaining_games: BTreeMap<usize, usize> = BTreeMap::new();
        let mut games_remaining_in_season = 0;

        for week in season.weeks().iter() {
            for matchup in week.matchups().iter() {
                if !matchup.context().game_over() {
                    games_remaining_in_season += 1;
                    *team_remaining_games.entry(*matchup.home_team()).or_insert(0) += 1;
                    *team_remaining_games.entry(*matchup.away_team()).or_insert(0) += 1;
                }
            }
        }

        let total_games = season.weeks().len();
        let mut all_entries = Vec::new();

        // Process each conference
        for conf_index in 0..num_conferences {
            let conference = season.conferences().get(conf_index)
                .ok_or_else(|| format!("Conference {} not found", conf_index))?;

            // Get conference standings
            let conf_standings = season.conference_standings(conf_index)?;

            // Determine division winners
            let mut division_winners: Vec<usize> = Vec::new();
            if division_winners_guaranteed {
                for (div_id, _) in conference.divisions().iter().enumerate() {
                    let div_standings = season.division_standings(conf_index, div_id)?;
                    if let Some((winner_id, _)) = div_standings.first() {
                        division_winners.push(*winner_id);
                    }
                }
            }

            // Determine playoff teams for this conference
            let mut conf_playoff_teams: Vec<usize> = Vec::new();

            // First, add all division winners (if guaranteed)
            for &winner in &division_winners {
                if conf_playoff_teams.len() < playoff_teams_per_conference {
                    conf_playoff_teams.push(winner);
                }
            }

            // Fill remaining spots with wild cards (best records not already in)
            for (team_id, _record) in conf_standings.iter() {
                if conf_playoff_teams.len() >= playoff_teams_per_conference {
                    break;
                }
                if !conf_playoff_teams.contains(team_id) {
                    conf_playoff_teams.push(*team_id);
                }
            }

            // Compute record bounds for conference teams
            let bounds: Vec<RecordBounds> = conf_standings
                .iter()
                .map(|(team_id, record)| {
                    let remaining = *team_remaining_games.get(team_id).unwrap_or(&0);
                    RecordBounds::from_record(*team_id, record, remaining, total_games)
                })
                .collect();

            // Build entries for each team in the conference
            for (position, (team_id, record)) in conf_standings.iter().enumerate() {
                let team_name = season
                    .teams()
                    .get(team_id)
                    .map(|t| t.name().to_string())
                    .unwrap_or_else(|| format!("Team {}", team_id));

                let remaining = *team_remaining_games.get(team_id).unwrap_or(&0);
                let in_playoff_position = position < playoff_teams_per_conference;
                let is_division_winner = division_winners.contains(team_id);

                // Calculate games back (within conference)
                let games_back = if in_playoff_position {
                    0.0
                } else {
                    Self::compute_games_back(&conf_standings, position, playoff_teams_per_conference)
                };

                // Determine status (within conference context)
                let status = Self::compute_status(
                    *team_id,
                    position,
                    &bounds,
                    playoff_teams_per_conference,
                );

                // Calculate magic number
                let magic_number = if matches!(status, PlayoffStatus::Eliminated) {
                    None
                } else {
                    Self::compute_magic_number(*team_id, &bounds, playoff_teams_per_conference)
                };

                // Adjust status based on division winner guarantee
                let final_status = if is_division_winner && division_winners_guaranteed {
                    // Division winners get special consideration
                    match &status {
                        PlayoffStatus::Eliminated => status, // Can't be eliminated if div winner guaranteed
                        _ => status,
                    }
                } else {
                    status
                };

                all_entries.push(PlayoffPictureEntry {
                    team_id: *team_id,
                    team_name,
                    current_record: record.clone(),
                    status: final_status,
                    games_back,
                    remaining_games: remaining,
                    magic_number,
                });
            }
        }

        Ok(PlayoffPicture {
            num_playoff_teams: total_playoff_teams,
            entries: all_entries,
            games_remaining_in_season,
        })
    }

    /// Compute how many games a team is behind the playoff cutoff
    fn compute_games_back(
        standings: &[(usize, LeagueTeamRecord)],
        position: usize,
        num_playoff_teams: usize,
    ) -> f64 {
        if position < num_playoff_teams || num_playoff_teams == 0 {
            return 0.0;
        }

        // Get the last playoff team's record
        let cutoff_idx = num_playoff_teams - 1;
        let (_, cutoff_record) = &standings[cutoff_idx];
        let (_, team_record) = &standings[position];

        // Games back = (cutoff_wins - team_wins + 0.5*(cutoff_ties - team_ties)) / 2
        // This follows standard "games back" calculation
        let cutoff_win_value = *cutoff_record.wins() as f64 + 0.5 * *cutoff_record.ties() as f64;
        let team_win_value = *team_record.wins() as f64 + 0.5 * *team_record.ties() as f64;
        let cutoff_loss_value = *cutoff_record.losses() as f64 + 0.5 * *cutoff_record.ties() as f64;
        let team_loss_value = *team_record.losses() as f64 + 0.5 * *team_record.ties() as f64;

        ((cutoff_win_value - team_win_value) + (team_loss_value - cutoff_loss_value)) / 2.0
    }

    /// Compute a team's playoff status
    fn compute_status(
        team_id: usize,
        position: usize,
        bounds: &[RecordBounds],
        num_playoff_teams: usize,
    ) -> PlayoffStatus {
        let in_playoff_position = position < num_playoff_teams;
        let current_seed = position + 1;

        // Check if eliminated (best case can't make playoffs)
        if Self::is_eliminated(team_id, bounds, num_playoff_teams) {
            return PlayoffStatus::Eliminated;
        }

        // Check if clinched top seed
        if in_playoff_position && position == 0 && Self::has_clinched_top_seed(team_id, bounds) {
            return PlayoffStatus::ClinchedTopSeed;
        }

        // Check if clinched playoffs
        if Self::has_clinched_playoffs(team_id, bounds, num_playoff_teams) {
            return PlayoffStatus::ClinchedPlayoffs { current_seed };
        }

        // If in playoff position but not clinched
        if in_playoff_position {
            return PlayoffStatus::InPlayoffPosition { current_seed };
        }

        // Not in playoff position but not eliminated
        PlayoffStatus::InTheHunt
    }

    /// Check if a team has clinched a playoff spot
    ///
    /// A team clinches if: even when they lose ALL remaining games AND
    /// all other teams win ALL remaining games, they still finish in top N
    fn has_clinched_playoffs(
        team_id: usize,
        bounds: &[RecordBounds],
        num_playoff_teams: usize,
    ) -> bool {
        let team_bounds = match bounds.iter().find(|b| b.team_id == team_id) {
            Some(b) => b,
            None => return false,
        };

        // Count how many teams could potentially finish ahead of us
        let mut teams_that_could_pass = 0;

        for other_bounds in bounds.iter() {
            if other_bounds.team_id == team_id {
                continue;
            }

            // Can this team pass us in our worst case vs their best case?
            if Self::would_finish_ahead(
                other_bounds.max_possible_wins,
                other_bounds.max_possible_win_pct,
                other_bounds.team_id,
                team_bounds.min_possible_wins,
                team_bounds.min_possible_win_pct,
                team_id,
            ) {
                teams_that_could_pass += 1;
            }
        }

        // If fewer than num_playoff_teams could pass us, we've clinched
        teams_that_could_pass < num_playoff_teams
    }

    /// Check if a team has clinched the #1 seed
    fn has_clinched_top_seed(team_id: usize, bounds: &[RecordBounds]) -> bool {
        let team_bounds = match bounds.iter().find(|b| b.team_id == team_id) {
            Some(b) => b,
            None => return false,
        };

        // In worst case, can any other team pass us?
        for other_bounds in bounds.iter() {
            if other_bounds.team_id == team_id {
                continue;
            }

            if Self::would_finish_ahead(
                other_bounds.max_possible_wins,
                other_bounds.max_possible_win_pct,
                other_bounds.team_id,
                team_bounds.min_possible_wins,
                team_bounds.min_possible_win_pct,
                team_id,
            ) {
                return false; // Someone could pass us
            }
        }
        true
    }

    /// Check if a team has been eliminated from playoff contention
    ///
    /// A team is eliminated if: even when they win ALL remaining games AND
    /// all teams above them lose ALL remaining games, they still can't finish in top N
    fn is_eliminated(
        team_id: usize,
        bounds: &[RecordBounds],
        num_playoff_teams: usize,
    ) -> bool {
        let team_bounds = match bounds.iter().find(|b| b.team_id == team_id) {
            Some(b) => b,
            None => return true,
        };

        // Count how many teams will definitely finish ahead of us
        let mut teams_definitely_ahead = 0;

        for other_bounds in bounds.iter() {
            if other_bounds.team_id == team_id {
                continue;
            }

            // Will this team definitely be ahead even in their worst case vs our best case?
            if Self::would_finish_ahead(
                other_bounds.min_possible_wins,
                other_bounds.min_possible_win_pct,
                other_bounds.team_id,
                team_bounds.max_possible_wins,
                team_bounds.max_possible_win_pct,
                team_id,
            ) {
                teams_definitely_ahead += 1;
            }
        }

        // If num_playoff_teams or more will definitely be ahead, we're eliminated
        teams_definitely_ahead >= num_playoff_teams
    }

    /// Determine if a team would finish ahead of another team given their records
    fn would_finish_ahead(
        team1_wins: usize,
        team1_pct: f64,
        team1_id: usize,
        team2_wins: usize,
        team2_pct: f64,
        team2_id: usize,
    ) -> bool {
        // Primary: win percentage (higher is better)
        if (team1_pct - team2_pct).abs() > 1e-9 {
            return team1_pct > team2_pct;
        }

        // Secondary: total wins (higher is better)
        if team1_wins != team2_wins {
            return team1_wins > team2_wins;
        }

        // Tertiary: team ID (lower is better)
        team1_id < team2_id
    }

    /// Calculate magic number for clinching playoffs, if applicable
    fn compute_magic_number(
        team_id: usize,
        bounds: &[RecordBounds],
        num_playoff_teams: usize,
    ) -> Option<usize> {
        let team_bounds = bounds.iter().find(|b| b.team_id == team_id)?;

        // If no remaining games, magic number is 0 if clinched, None otherwise
        if team_bounds.remaining_games == 0 {
            if Self::has_clinched_playoffs(team_id, bounds, num_playoff_teams) {
                return Some(0);
            } else {
                return None;
            }
        }

        // Find the team that would be at the cutoff (first team out)
        // Sort bounds by best-case scenario to find who could potentially be in the last spot
        let mut sorted_bounds: Vec<&RecordBounds> = bounds.iter().collect();
        sorted_bounds.sort_by(|a, b| {
            // Sort by max possible win pct descending
            b.max_possible_win_pct.partial_cmp(&a.max_possible_win_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.max_possible_wins.cmp(&a.max_possible_wins))
                .then_with(|| a.team_id.cmp(&b.team_id))
        });

        // Magic number = team's current wins + team's remaining games + 1 - cutoff team's max possible wins
        // Simplified: we need our wins to be greater than what the (num_playoff_teams)th best team could achieve

        // Find teams that could potentially block us (excluding ourselves)
        let mut potential_blockers: Vec<&RecordBounds> = bounds
            .iter()
            .filter(|b| b.team_id != team_id)
            .collect();

        // Sort by max potential (best case for them)
        potential_blockers.sort_by(|a, b| {
            b.max_possible_win_pct.partial_cmp(&a.max_possible_win_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.max_possible_wins.cmp(&a.max_possible_wins))
                .then_with(|| a.team_id.cmp(&b.team_id))
        });

        // The magic number is based on eliminating the possibility of being pushed out
        // We need to beat or tie the (num_playoff_teams)th best team's maximum potential
        if potential_blockers.len() < num_playoff_teams {
            // Not enough other teams to fill playoffs, so we're in
            return Some(0);
        }

        // The team at index (num_playoff_teams - 1) is the last team that could take a playoff spot
        // (since we excluded ourselves, this is the team that would push us out if they finish strong)
        let blocker = potential_blockers[num_playoff_teams - 1];

        // To clinch, we need: our_final_wins > blocker's max wins (accounting for tiebreakers)
        // Magic number = blocker's max wins + 1 - our current wins
        // But we also need to cap it at our remaining games
        let target_wins = blocker.max_possible_wins + 1;
        if team_bounds.current_wins >= target_wins {
            Some(0)
        } else {
            let magic = target_wins - team_bounds.current_wins;
            if magic > team_bounds.remaining_games {
                // Can't reach the magic number with remaining games - might still make it via tiebreakers
                // Return remaining games as an approximation
                Some(team_bounds.remaining_games)
            } else {
                Some(magic)
            }
        }
    }

    /// Get the number of playoff spots
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// assert!(picture.num_playoff_teams() == 2);
    /// ```
    pub fn num_playoff_teams(&self) -> usize {
        self.num_playoff_teams
    }

    /// Get all entries sorted by standings
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let entries = picture.entries();
    /// ```
    pub fn entries(&self) -> &Vec<PlayoffPictureEntry> {
        &self.entries
    }

    /// Get total games remaining in the season
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let entries = picture.entries();
    /// ```
    pub fn games_remaining_in_season(&self) -> usize {
        self.games_remaining_in_season
    }

    /// Get all teams currently in playoff position
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let playoff_teams = picture.playoff_teams();
    /// ```
    pub fn playoff_teams(&self) -> Vec<&PlayoffPictureEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(
                e.status,
                PlayoffStatus::ClinchedTopSeed
                    | PlayoffStatus::ClinchedPlayoffs { .. }
                    | PlayoffStatus::InPlayoffPosition { .. }
            ))
            .collect()
    }

    /// Get all teams that have clinched a playoff spot
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let clinched_teams = picture.clinched_teams();
    /// ```
    pub fn clinched_teams(&self) -> Vec<&PlayoffPictureEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(
                e.status,
                PlayoffStatus::ClinchedTopSeed | PlayoffStatus::ClinchedPlayoffs { .. }
            ))
            .collect()
    }

    /// Get all teams still in the hunt (not in playoffs but not eliminated)
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let in_the_hunt = picture.in_the_hunt();
    /// ```
    pub fn in_the_hunt(&self) -> Vec<&PlayoffPictureEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.status, PlayoffStatus::InTheHunt))
            .collect()
    }

    /// Get all eliminated teams
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let eliminated_teams = picture.eliminated_teams();
    /// ```
    pub fn eliminated_teams(&self) -> Vec<&PlayoffPictureEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.status, PlayoffStatus::Eliminated))
            .collect()
    }

    /// Get a specific team's entry
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::FootballTeam;
    /// use fbsim_core::league::season::LeagueSeason;
    /// use fbsim_core::league::season::LeagueSeasonScheduleOptions;
    /// use fbsim_core::league::season::playoffs::picture::PlayoffPicture;
    ///
    /// // Create a new season with 4 teams
    /// let mut my_league_season = LeagueSeason::new();
    /// my_league_season.add_team(0, FootballTeam::new());
    /// my_league_season.add_team(1, FootballTeam::new());
    /// my_league_season.add_team(2, FootballTeam::new());
    /// my_league_season.add_team(3, FootballTeam::new());
    ///
    /// // Generate the season schedule
    /// let mut rng = rand::thread_rng();
    /// my_league_season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng);
    ///
    /// // Get the playoff picture for a 2-team playoff
    /// let picture = PlayoffPicture::from_season(&my_league_season, 2, None).unwrap();
    /// let team_status = picture.team_status(3);
    /// assert!(team_status.is_some());
    /// ```
    pub fn team_status(&self, team_id: usize) -> Option<&PlayoffPictureEntry> {
        self.entries.iter().find(|e| e.team_id == team_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playoff_status_ordering() {
        // Ensure enum variants can be compared
        assert!(PlayoffStatus::ClinchedTopSeed < PlayoffStatus::ClinchedPlayoffs { current_seed: 1 });
    }

    #[test]
    fn test_would_finish_ahead_by_pct() {
        // Team with higher win percentage should finish ahead
        assert!(PlayoffPicture::would_finish_ahead(8, 0.8, 1, 6, 0.6, 2));
        assert!(!PlayoffPicture::would_finish_ahead(6, 0.6, 2, 8, 0.8, 1));
    }

    #[test]
    fn test_would_finish_ahead_by_wins_tiebreaker() {
        // Same percentage, more wins should finish ahead
        assert!(PlayoffPicture::would_finish_ahead(8, 0.5, 1, 4, 0.5, 2));
        assert!(!PlayoffPicture::would_finish_ahead(4, 0.5, 2, 8, 0.5, 1));
    }

    #[test]
    fn test_would_finish_ahead_by_team_id_tiebreaker() {
        // Same percentage and wins, lower team ID wins
        assert!(PlayoffPicture::would_finish_ahead(8, 0.5, 1, 8, 0.5, 2));
        assert!(!PlayoffPicture::would_finish_ahead(8, 0.5, 2, 8, 0.5, 1));
    }

    #[test]
    fn test_entry_is_clinched() {
        let entry = PlayoffPictureEntry {
            team_id: 1,
            team_name: "Test".to_string(),
            current_record: LeagueTeamRecord::new(),
            status: PlayoffStatus::ClinchedPlayoffs { current_seed: 1 },
            games_back: 0.0,
            remaining_games: 0,
            magic_number: Some(0),
        };
        assert!(entry.is_clinched());
        assert!(!entry.is_eliminated());
    }

    #[test]
    fn test_entry_is_eliminated() {
        let entry = PlayoffPictureEntry {
            team_id: 1,
            team_name: "Test".to_string(),
            current_record: LeagueTeamRecord::new(),
            status: PlayoffStatus::Eliminated,
            games_back: 5.0,
            remaining_games: 0,
            magic_number: None,
        };
        assert!(!entry.is_clinched());
        assert!(entry.is_eliminated());
    }

    #[test]
    fn test_playoff_picture_new_season() {
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        // Create a season with 4 teams
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        // Generate the schedule
        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();

        // Get playoff picture for 2-team playoffs
        let picture = PlayoffPicture::from_season(&season, 2, None).unwrap();

        // All teams should have games remaining
        assert!(picture.games_remaining_in_season() > 0);
        assert_eq!(picture.entries().len(), 4);
        assert_eq!(picture.num_playoff_teams(), 2);

        // At the start, no team should be clinched or eliminated
        assert!(picture.clinched_teams().is_empty());
        assert!(picture.eliminated_teams().is_empty());

        // All teams should be either in playoff position or in the hunt
        for entry in picture.entries() {
            assert!(!entry.is_clinched());
            assert!(!entry.is_eliminated());
            assert!(entry.remaining_games() > 0);
        }
    }

    #[test]
    fn test_playoff_picture_partial_season() {
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        // Create a season with 4 teams
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        // Generate the schedule
        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();

        // Simulate half the season
        let total_weeks = season.weeks().len();
        for week_idx in 0..(total_weeks / 2) {
            season.sim_week(week_idx, &mut rng).unwrap();
        }

        // Get playoff picture
        let picture = PlayoffPicture::from_season(&season, 2, None).unwrap();

        // Should have some games remaining
        assert!(picture.games_remaining_in_season() > 0);
        assert_eq!(picture.entries().len(), 4);

        // Verify playoff_teams returns top 2 teams
        let playoff_teams = picture.playoff_teams();
        assert_eq!(playoff_teams.len(), 2);

        // Verify games back is 0 for playoff teams
        for team in playoff_teams {
            assert_eq!(team.games_back(), 0.0);
        }
    }

    #[test]
    fn test_playoff_picture_complete_season() {
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        // Create a season with 4 teams
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        // Generate and complete the schedule
        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();
        season.sim_regular_season(&mut rng).unwrap();

        // Get playoff picture
        let picture = PlayoffPicture::from_season(&season, 2, None).unwrap();

        // No games remaining
        assert_eq!(picture.games_remaining_in_season(), 0);

        // All remaining games for each team should be 0
        for entry in picture.entries() {
            assert_eq!(entry.remaining_games(), 0);
        }

        // Top 2 should be clinched, bottom 2 should be eliminated
        let clinched = picture.clinched_teams();
        let eliminated = picture.eliminated_teams();
        assert_eq!(clinched.len(), 2);
        assert_eq!(eliminated.len(), 2);
    }

    #[test]
    fn test_playoff_picture_validation_errors() {
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        // Create a season with 4 teams
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        // Generate the schedule
        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();

        // Test: num_playoff_teams = 0 should fail
        let result = PlayoffPicture::from_season(&season, 0, None);
        assert!(result.is_err());

        // Test: num_playoff_teams > total teams should fail
        let result = PlayoffPicture::from_season(&season, 5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_playoff_picture_empty_schedule() {
        use crate::team::FootballTeam;
        use crate::league::season::LeagueSeason;

        // Create a season with teams but no schedule
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();

        // Should fail because no schedule
        let result = PlayoffPicture::from_season(&season, 1, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_playoff_picture_team_status() {
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        // Create a season with 4 teams
        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        // Generate the schedule
        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();

        // Get playoff picture
        let picture = PlayoffPicture::from_season(&season, 2, None).unwrap();

        // Should be able to look up each team
        for id in 0..4 {
            let status = picture.team_status(id);
            assert!(status.is_some());
            assert_eq!(status.unwrap().team_id(), id);
        }

        // Non-existent team should return None
        assert!(picture.team_status(99).is_none());
    }

    #[test]
    fn test_games_back_calculation() {
        // Test the games back calculation directly
        use crate::team::FootballTeam;
        use crate::league::season::{LeagueSeason, LeagueSeasonScheduleOptions};

        let mut season = LeagueSeason::new();
        season.add_team(0, FootballTeam::new()).unwrap();
        season.add_team(1, FootballTeam::new()).unwrap();
        season.add_team(2, FootballTeam::new()).unwrap();
        season.add_team(3, FootballTeam::new()).unwrap();

        let mut rng = rand::thread_rng();
        season.generate_schedule(LeagueSeasonScheduleOptions::new(), &mut rng).unwrap();

        // Simulate most of the season
        let total_weeks = season.weeks().len();
        for week_idx in 0..(total_weeks - 1) {
            season.sim_week(week_idx, &mut rng).unwrap();
        }

        let picture = PlayoffPicture::from_season(&season, 2, None).unwrap();

        // Teams in playoff position should have games_back = 0
        for entry in picture.playoff_teams() {
            assert_eq!(entry.games_back(), 0.0);
        }

        // Teams in the hunt should have games_back >= 0
        for entry in picture.in_the_hunt() {
            assert!(entry.games_back() >= 0.0);
        }
    }
}
