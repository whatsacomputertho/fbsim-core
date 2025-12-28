#![doc = include_str!("../../docs/game/stat.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// # `RushingStats` struct
///
/// A `RushingStats` represents aggregated rushing statistics
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct RushingStats {
    rushes: u32,
    fumbles: u32,
    touchdowns: u32,
    yards: i32
}

impl RushingStats {
    /// Constructor for the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let my_stats = RushingStats::new();
    /// ```
    pub fn new() -> RushingStats {
        RushingStats::default()
    }

    /// Get the number of rushes from the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let my_stats = RushingStats::new();
    /// let rushes = my_stats.rushes();
    /// assert!(rushes == 0);
    /// ```
    pub fn rushes(&self) -> u32 {
        self.rushes
    }

    /// Increment the rushes in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let mut my_stats = RushingStats::new();
    /// my_stats.increment_rushes();
    /// assert!(my_stats.rushes() == 1);
    /// ```
    pub fn increment_rushes(&mut self) {
        self.rushes += 1;
    }

    /// Get the number of fumbles from the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let my_stats = RushingStats::new();
    /// let fumbles = my_stats.fumbles();
    /// assert!(fumbles == 0);
    /// ```
    pub fn fumbles(&self) -> u32 {
        self.fumbles
    }

    /// Increment the fumbles in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let mut my_stats = RushingStats::new();
    /// my_stats.increment_fumbles();
    /// assert!(my_stats.fumbles() == 1);
    /// ```
    pub fn increment_fumbles(&mut self) {
        self.fumbles += 1;
    }

    /// Get the number of touchdowns in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let my_stats = RushingStats::new();
    /// let touchdowns = my_stats.touchdowns();
    /// assert!(touchdowns == 0);
    /// ```
    pub fn touchdowns(&self) -> u32 {
        self.touchdowns
    }

    /// Increment the touchdowns in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let mut my_stats = RushingStats::new();
    /// my_stats.increment_touchdowns();
    /// assert!(my_stats.touchdowns() == 1);
    /// ```
    pub fn increment_touchdowns(&mut self) {
        self.touchdowns += 1;
    }

    /// Get the rushing yards in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let my_stats = RushingStats::new();
    /// let yards = my_stats.yards();
    /// assert!(yards == 0);
    /// ```
    pub fn yards(&self) -> i32 {
        self.yards
    }

    /// Increment the rushing yards in the RushingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    ///
    /// let mut my_stats = RushingStats::new();
    /// my_stats.increment_yards(12);
    /// assert!(my_stats.yards() == 12);
    /// ```
    pub fn increment_yards(&mut self, yards: i32) {
        self.yards += yards;
    }
}

impl std::fmt::Display for RushingStats {
    /// Display rushing stats as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::RushingStats;
    /// 
    /// let my_stats = RushingStats::new();
    /// println!("{}", my_stats);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rushing_str = format!(
            "{} rush, {} yards",
            self.rushes,
            self.yards
        );
        if self.touchdowns > 0 {
            rushing_str = format!(
                "{}, {} TD",
                rushing_str,
                self.touchdowns
            );
        }
        if self.fumbles > 0 {
            rushing_str = format!(
                "{}, {} FUM",
                rushing_str,
                self.fumbles
            );
        }
        f.write_str(&rushing_str)
    }
}

/// # `PassingStats` struct
///
/// A `PassingStats` represents aggregated passing statistics
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PassingStats {
    attempts: u32,
    completions: u32,
    touchdowns: u32,
    interceptions: u32,
    yards: i32
}

impl PassingStats {
    /// Constructor for the PassingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// ```
    pub fn new() -> PassingStats {
        PassingStats::default()
    }

    /// Get the pass attempts from the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// let attempts = my_stats.attempts();
    /// assert!(attempts == 0);
    /// ```
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Increment the pass attempts in the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let mut my_stats = PassingStats::new();
    /// my_stats.increment_attempts();
    /// assert!(my_stats.attempts() == 1);
    /// ```
    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }

    /// Get the completions from the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// let completions = my_stats.completions();
    /// assert!(completions == 0);
    /// ```
    pub fn completions(&self) -> u32 {
        self.completions
    }

    /// Increment the completions in the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let mut my_stats = PassingStats::new();
    /// my_stats.increment_completions();
    /// assert!(my_stats.completions() == 1);
    /// ```
    pub fn increment_completions(&mut self) {
        self.completions += 1;
    }

    /// Get the touchdowns from the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// let touchdowns = my_stats.touchdowns();
    /// assert!(touchdowns == 0);
    /// ```
    pub fn touchdowns(&self) -> u32 {
        self.touchdowns
    }

    /// Increment the touchdowns in the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let mut my_stats = PassingStats::new();
    /// my_stats.increment_touchdowns();
    /// assert!(my_stats.touchdowns() == 1);
    /// ```
    pub fn increment_touchdowns(&mut self) {
        self.touchdowns += 1;
    }

    /// Get the interceptions from the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// let interceptions = my_stats.interceptions();
    /// assert!(interceptions == 0);
    /// ```
    pub fn interceptions(&self) -> u32 {
        self.interceptions
    }

    /// Increment the interceptions in the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let mut my_stats = PassingStats::new();
    /// my_stats.increment_interceptions();
    /// assert!(my_stats.interceptions() == 1);
    /// ```
    pub fn increment_interceptions(&mut self) {
        self.interceptions += 1;
    }

    /// Get the yards from the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let my_stats = PassingStats::new();
    /// let yards = my_stats.yards();
    /// assert!(yards == 0);
    /// ```
    pub fn yards(&self) -> i32 {
        self.yards
    }

    /// Increment the yards in the PassingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    ///
    /// let mut my_stats = PassingStats::new();
    /// my_stats.increment_yards(25);
    /// assert!(my_stats.yards() == 25);
    /// ```
    pub fn increment_yards(&mut self, yards: i32) {
        self.yards += yards;
    }
}

impl std::fmt::Display for PassingStats {
    /// Display passing stats as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::PassingStats;
    /// 
    /// let my_stats = PassingStats::new();
    /// println!("{}", my_stats);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut passing_str = format!(
            "{}/{}, {} yards",
            self.completions,
            self.attempts,
            self.yards
        );
        if self.touchdowns > 0 {
            passing_str = format!(
                "{}, {} TD",
                passing_str,
                self.touchdowns
            );
        }
        if self.interceptions > 0 {
            passing_str = format!(
                "{}, {} INT",
                passing_str,
                self.interceptions
            );
        }
        f.write_str(&passing_str)
    }
}

/// # `ReceivingStats` struct
///
/// A `ReceivingStats` represents aggregated receiving statistics
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct ReceivingStats {
    targets: u32,
    receptions: u32,
    touchdowns: u32,
    fumbles: u32,
    yards: i32
}

impl ReceivingStats {
    /// Constructor for the ReceivingStats struct
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// ```
    pub fn new() -> ReceivingStats {
        ReceivingStats::default()
    }

    /// Get the receiving targets from the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// let targets = my_stats.targets();
    /// assert!(targets == 0);
    /// ```
    pub fn targets(&self) -> u32 {
        self.targets
    }

    /// Increment the receiving targets in the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let mut my_stats = ReceivingStats::new();
    /// my_stats.increment_targets(1);
    /// assert!(my_stats.targets() == 1);
    /// ```
    pub fn increment_targets(&mut self, targets: u32) {
        self.targets += targets;
    }

    /// Get the receptions from the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// let receptions = my_stats.receptions();
    /// assert!(receptions == 0);
    /// ```
    pub fn receptions(&self) -> u32 {
        self.receptions
    }

    /// Increment the receptions in the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let mut my_stats = ReceivingStats::new();
    /// my_stats.increment_receptions(1);
    /// assert!(my_stats.receptions() == 1);
    /// ```
    pub fn increment_receptions(&mut self, receptions: u32) {
        self.receptions += receptions;
    }

    /// Get the touchdowns from the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// let touchdowns = my_stats.touchdowns();
    /// assert!(touchdowns == 0);
    /// ```
    pub fn touchdowns(&self) -> u32 {
        self.touchdowns
    }

    /// Increment the touchdowns in the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let mut my_stats = ReceivingStats::new();
    /// my_stats.increment_touchdowns(1);
    /// assert!(my_stats.touchdowns() == 1);
    /// ```
    pub fn increment_touchdowns(&mut self, touchdowns: u32) {
        self.touchdowns += touchdowns;
    }

    /// Get the fumbles from the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// let fumbles = my_stats.fumbles();
    /// assert!(fumbles == 0);
    /// ```
    pub fn fumbles(&self) -> u32 {
        self.fumbles
    }

    /// Increment the fumbles in the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let mut my_stats = ReceivingStats::new();
    /// my_stats.increment_fumbles(1);
    /// assert!(my_stats.fumbles() == 1);
    /// ```
    pub fn increment_fumbles(&mut self, fumbles: u32) {
        self.fumbles += fumbles;
    }

    /// Get the yards from the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let my_stats = ReceivingStats::new();
    /// let yards = my_stats.yards();
    /// assert!(yards == 0);
    /// ```
    pub fn yards(&self) -> i32 {
        self.yards
    }

    /// Increment the yards in the ReceivingStats
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    ///
    /// let mut my_stats = ReceivingStats::new();
    /// my_stats.increment_yards(11);
    /// assert!(my_stats.yards() == 11);
    /// ```
    pub fn increment_yards(&mut self, yards: i32) {
        self.yards += yards;
    }
}

impl std::fmt::Display for ReceivingStats {
    /// Display receiving stats as a human readable string
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::stat::ReceivingStats;
    /// 
    /// let my_stats = ReceivingStats::new();
    /// println!("{}", my_stats);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut receiving_str = format!(
            "{} rec ({} tar), {} yards",
            self.receptions,
            self.targets,
            self.yards
        );
        if self.touchdowns > 0 {
            receiving_str = format!(
                "{}, {} TD",
                receiving_str,
                self.touchdowns
            );
        }
        if self.fumbles > 0 {
            receiving_str = format!(
                "{}, {} FUM",
                receiving_str,
                self.fumbles
            );
        }
        f.write_str(&receiving_str)
    }
}
