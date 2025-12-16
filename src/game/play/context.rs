#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::game::context::GameContext;

/// # `PlayContext` struct
///
/// A `PlayContext` represents a play scenario
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct PlayContext {
    quarter: u32,
    half_seconds: u32,
    down: u32,
    distance: u32,
    yard_line: u32,
    score_diff: i32,
    off_timeouts: u32,
    def_timeouts: u32,
    clock_running: bool
}

impl From<&GameContext> for PlayContext {
    /// Initialize a PlayContext from a borrowed GameContext
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// ```
    fn from(item: &GameContext) -> PlayContext {
        // Determine score diff and timeouts based on possession
        let score_diff: i32 = if *item.home_possession() {
            *item.home_score() as i32 - *item.away_score() as i32
        } else {
            *item.away_score() as i32 - *item.home_score() as i32
        };
        let off_timeouts: u32 = if *item.home_possession() {
            *item.home_timeouts()
        } else {
            *item.away_timeouts()
        };
        let def_timeouts: u32 = if *item.home_possession() {
            *item.away_timeouts()
        } else {
            *item.home_timeouts()
        };

        // Determine yard line based on possession and direction
        let yard_line: u32 = if *item.home_possession() ^ *item.home_positive_direction() {
            match u32::try_from(100_i32 - *item.yard_line() as i32) {
                Ok(n) => n,
                0
            }
        } else {
            *item.yard_line()
        };

        // Construct the play context
        PlayContext{
            quarter: *item.quarter(),
            half_seconds: *item.half_seconds(),
            down: *item.down(),
            distance: *item.distance(),
            yard_line: yard_line,
            score_diff: score_diff,
            off_timeouts: off_timeouts,
            def_timeouts: def_timeouts,
            clock_running: item.clock_running()
        }
    }
}

impl PlayContext {
    /// Whether the clock is running
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let clock_running = play_context.clock_running();
    /// assert!(!clock_running);
    /// ```
    pub fn clock_running(&self) -> bool {
        self.clock_running
    }

    /// Gets the current down
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let down = play_context.down();
    /// assert!(down == 0);
    /// ```
    pub fn down(&self) -> u32 {
        self.down
    }

    /// Gets the current distance
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let distance = play_context.distance();
    /// assert!(distance == 10);
    /// ```
    pub fn distance(&self) -> u32 {
        self.distance
    }

    /// Gets the current yard line
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let yard_line = play_context.yard_line();
    /// assert!(yard_line == 35);
    /// ```
    pub fn yard_line(&self) -> u32 {
        self.yard_line
    }

    /// Gets the number of timeouts the offense has
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let offense_timeouts = play_context.offense_timeouts();
    /// assert!(offense_timeouts == 3);
    /// ```
    pub fn offense_timeouts(&self) -> u32 {
        self.off_timeouts
    }

    /// Gets the number of timeouts the defense has
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let defense_timeouts = play_context.defense_timeouts();
    /// assert!(defense_timeouts == 3);
    /// ```
    pub fn defense_timeouts(&self) -> u32 {
        self.def_timeouts
    }

    /// Gets the current quarter
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let quarter = play_context.quarter();
    /// assert!(quarter == 1);
    /// ```
    pub fn quarter(&self) -> u32 {
        self.quarter
    }

    /// Whether this is a drain-clock scenario for the offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let drain_clock = play_context.drain_clock();
    /// assert!(!drain_clock);
    /// ```
    pub fn drain_clock(&self) -> bool {
        if self.score_diff <= 0 {
            return false
        }
        let scores_up_by: f32 = self.score_diff as f32 / 8_f32;
        let drain_threshold_sig: i32 = (scores_up_by * 4_f32 * 60_f32) as i32;
        let drain_threshold: u32 = match u32::try_from(drain_threshold_sig) {
            Ok(n) => n,
            Err(_) => 0
        };
        if self.quarter >= 4 && self.half_seconds < drain_threshold {
            return true
        }
        return false
    }

    /// Whether this is an up-tempo scenario for the offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let up_tempo = play_context.up_tempo();
    /// assert!(!up_tempo);
    /// ```
    pub fn up_tempo(&self) -> bool {
        self.quarter >= 4 && self.half_seconds <= 180 &&
        self.score_diff < 0 && self.score_diff >= -17
    }

    /// Whether this is a critical down scenario
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let critical_down = play_context.critical_down();
    /// assert!(!critical_down);
    /// ```
    pub fn critical_down(&self) -> bool {
        self.down == 3 && self.half_seconds <= 180 &&
        self.score_diff < 9 && self.score_diff > -9
    }

    /// Whether this is a conserve-clock scenario for the offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let conserve_clock = play_context.offense_conserve_clock();
    /// assert!(!conserve_clock);
    /// ```
    pub fn offense_conserve_clock(&self) -> bool {
        self.quarter >= 4 && self.half_seconds <= 180 &&
        self.score_diff < 0 && self.score_diff > -18
    }

    /// Whether this is a conserve-clock scenario for the defense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let conserve_clock = play_context.defense_conserve_clock();
    /// assert!(!conserve_clock);
    /// ```
    pub fn defense_conserve_clock(&self) -> bool {
        self.quarter >= 4 && self.half_seconds <= 180 &&
        self.score_diff > 0 && self.score_diff < 18
    }

    /// Whether this is the last play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let last_play = play_context.last_play();
    /// assert!(!last_play);
    /// ```
    pub fn last_play(&self) -> bool {
        self.half_seconds < 6
    }

    /// Whether the offense needs a touchdown on the last play
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let last_play_need_td = play_context.last_play_need_td();
    /// assert!(!last_play_need_td);
    /// ```
    pub fn last_play_need_td(&self) -> bool {
        self.score_diff < -3
    }

    /// Whether the offense can kneel to end the game
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let can_kneel = play_context.can_kneel();
    /// assert!(!can_kneel);
    /// ```
    pub fn can_kneel(&self) -> bool {
        let downs_remaining = 4 - self.down;
        let runoff_seconds = 42 * 0.max(downs_remaining - self.def_timeouts);
        runoff_seconds >= self.half_seconds
    }

    /// Whether this is a must-score scenario for 4th-down playcalling
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let must_score = play_context.must_score();
    /// assert!(!must_score);
    /// ```
    pub fn must_score(&self) -> bool {
        if self.score_diff >= 0 {
            return false
        }
        let timeout_drive_time = (42 * (3 - self.off_timeouts)) + 8;
        if self.half_seconds <= timeout_drive_time {
            return true
        }
        let non_timeout_drive_time = (42 * 3) + 8;
        let timeout_drives_remaining: u32 = 1;
        let non_timeout_drive_time_remaining = match u32::try_from(self.half_seconds - timeout_drive_time) {
            Ok(n) => n,
            Err(_) => 0
        };
        let non_timeout_drives_remaining = (
            non_timeout_drive_time_remaining as f32 / non_timeout_drive_time as f32
        ).ceil() as u32;
        let scores_needed = (self.score_diff as f32 / 8_f32).round().abs() as u32;
        let drives_remaining = timeout_drives_remaining + non_timeout_drives_remaining;
        drives_remaining <= scores_needed
    }

    /// Whether this is a go for it on 4th scenario
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let can_go_for_it = play_context.can_go_for_it();
    /// assert!(!can_go_for_it);
    /// ```
    pub fn can_go_for_it(&self) -> bool {
        self.distance <= 4 && (
            self.yard_line >= 80 ||
            (self.yard_line >= 40 && self.yard_line <= 60)
        )
    }

    /// Whether the offense is in field goal range
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    /// 
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// let in_field_goal_range = play_context.in_field_goal_range();
    /// assert!(!in_field_goal_range);
    /// ```
    pub fn in_field_goal_range(&self) -> bool {
        self.yard_line >= 45
    }
}

impl std::fmt::Display for PlayContext {
    /// Format a `PlayContext` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use fbsim_core::game::context::GameContext;
    /// use fbsim_core::game::play::context::PlayContext;
    ///
    /// // Initialize a play context and display it
    /// let game_context = GameContext::new();
    /// let play_context = PlayContext::from(&game_context);
    /// println!("{}", play_context);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Format the clock
        let clock_total = if self.half_seconds < 900 {
            self.half_seconds
        } else {
            self.half_seconds - 900
        };
        let clock_mins = clock_total / 60;
        let clock_secs = clock_total - (clock_mins * 60);
        let clock_secs_str = if clock_secs < 10 {
            format!("0{}", clock_secs)
        } else {
            format!("{}", clock_secs)
        };
        let clock_str = format!("{}:{}", clock_mins, &clock_secs_str);

        // Format the quarter
        let quarter_str = if self.quarter <= 4 {
            format!("{}Q", self.quarter)
        } else {
            let num_ot = self.quarter - 4;
            format!("{}OT", num_ot)
        };

        // Format the down & distance
        let down_suf = match self.down {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };
        let down_dist_str = if self.yard_line + self.distance >= 100 {
            format!("{}{} & goal", self.down, down_suf)
        } else {
            format!("{}{} & {}", self.down, down_suf, self.distance)
        };

        // Format the yard line
        let (yard, side_of_field) = if self.yard_line < 50 {
            (self.yard_line, "OWN")
        } else {
            (100 - self.yard_line, "OPP")
        };
        let yard_str = format!("{} {}", side_of_field, yard);

        // Format the play context
        let context_str = format!(
            "[{} {}] {} at {}",
            clock_str,
            quarter_str,
            down_dist_str,
            yard_str
        );
        f.write_str(&context_str)
    }
}
