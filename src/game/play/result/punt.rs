
// Punt block probability regression
const P_BLOCK_INTR: f64 = -0.0010160286505995551_f64;
const P_BLOCK_COEF: f64 = 0.00703673_f64;

// Punt inside 20 skill-based probability regression
const P_PUNT_INSIDE_20_SKILL_INTR: f64 = 0.21398823243670145_f64;
const P_PUNT_INSIDE_20_SKILL_COEF: f64 = 0.32878206_f64;

// Punt inside 20 yard-line-based probability regression
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_1: f64 = 0.783829627_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_2: f64 = -0.200560110_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_3: f64 = 0.651500015_f64;
const P_PUNT_INSIDE_20_YARD_LINE_PARAM_4: f64 = -0.00178251834_f64;

// Punt inside 20 mean relative distance regression
const PUNT_INSIDE_20_MEAN_REL_DIST_INTR: f64 = 0.20907739629135946_f64;
const PUNT_INSIDE_20_MEAN_REL_DIST_COEF: f64 = -0.0001755_f64;

// Punt inside 20 std relative distance regression
const PUNT_INSIDE_20_STD_REL_DIST_INTR: f64 = 0.17519244654293623_f64;
const PUNT_INSIDE_20_STD_REL_DIST_COEF: f64 = -0.0016178_f64;

// Punt inside 20 skew relative distance regression
const PUNT_INSIDE_20_SKEW_REL_DIST_INTR: f64 = 3.691739354624472_f64;
const PUNT_INSIDE_20_SKEW_REL_DIST_COEF_1: f64 = -0.11961015_f64;
const PUNT_INSIDE_20_SKEW_REL_DIST_COEF_2: f64 = 0.00081621_f64;

// Punt outside 20 mean relative distance regression
const PUNT_OUTSIDE_20_MEAN_REL_DIST_INTR: f64 = -0.24995460069957565_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_1: f64 = 0.0400507456_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_2: f64 = -0.000758718087_f64;
const PUNT_OUTSIDE_20_MEAN_REL_DIST_COEF_3: f64 = 0.00000442573043_f64;

// Punt outside 20 std relative distance regression
const PUNT_OUTSIDE_20_STD_REL_DIST_INTR: f64 = 0.2748076520973469_f64;
const PUNT_OUTSIDE_20_STD_REL_DIST_COEF: f64 = -0.00196699_f64;

// Punt outside 20 skew relative distance regression
const PUNT_OUTSIDE_20_SKEW_REL_DIST_INTR: f64 = -5.631745519232158_f64;
const PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_1: f64 = 0.19789058_f64;
const PUNT_OUTSIDE_20_SKEW_REL_DIST_COEF_2: f64 = -0.00134607_f64;

// Punt out of bounds probability regression
const P_PUNT_OOB_INTR: f64 = -0.0846243447082426_f64;
const P_PUNT_OOB_COEF_1: f64 = 0.00575805979_f64;
const P_PUNT_OOB_COEF_2: f64 = -0.0000428367831_f64;

// Punt fair catch probability regression
const P_FAIR_CATCH_INTR: f64 = 0.47613371173695526_f64;
const P_FAIR_CATCH_COEF: f64 = -0.00141214_f64;

// Punt muffed probability regression
const P_MUFFED_PUNT_INTR: f64 = 0.036855240326056096_f64;
const P_MUFFED_PUNT_COEF: f64 = -0.02771741_f64;

// Mean relative punt return yards regression
const MEAN_REL_RETURN_YARDS_INTR: f64 = -0.0570321871_f64;
const MEAN_REL_RETURN_YARDS_COEF_1: f64 = -0.02282631_f64;
const MEAN_REL_RETURN_YARDS_COEF_2: f64 = 0.28982747_f64;

// Std relative punt return yards regression
const STD_REL_RETURN_YARDS_INTR: f64 = 0.06751127059206394_f64;
const STD_REL_RETURN_YARDS_COEF_1: f64 = 0.01035858_f64;
const STD_REL_RETURN_YARDS_COEF_2: f64 = 0.26338509_f64;

// Skew relative punt return yards regression
const SKEW_REL_RETURN_YARDS_INTR: f64 = -0.0167472281_f64;
const SKEW_REL_RETURN_YARDS_COEF_1: f64 = 7.06931813_f64;
const SKEW_REL_RETURN_YARDS_COEF_2: f64 = -6.94528823_f64;

// Fumble probability regression
const P_FUMBLE_INTR: f64 = 0.0460047101408259_f64;
const P_FUMBLE_COEF: f64 = -0.04389777_f64;

// Punt play duration regression
const PUNT_PLAY_DURATION_INTR: f64 = 5.2792296_f64;
const PUNT_PLAY_DURATION_COEF: f64 = 0.09291598_f64;

/// # `PuntResult` struct
///
/// A `PuntResult` represents a result of a punt play
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct PuntResult {
    fumble_return_yards: i32,
    punt_yards: i32,
    punt_return_yards: i32,
    play_duration: u32,
    blocked: bool,
    touchback: bool,
    out_of_bounds: bool,
    fair_catch: bool,
    muffed: bool,
    fumble: bool,
    touchdown: bool
}

impl Default for PuntResult {
    /// Default constructor for the PuntResult class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::game::play::result::punt::PuntResult;
    /// 
    /// let my_result = PuntResult::default();
    /// ```
    fn default() -> Self {
        PuntResult{
            fumble_return_yards: 0,
            punt_yards: 0,
            punt_return_yards: 0,
            play_duration: 0,
            blocked: false,
            touchback: false,
            out_of_bounds: false,
            fair_catch: false,
            muffed: false,
            fumble: false,
            touchdown: false
        }
    }
}

impl PlayResult for PuntResult {
    fn next_context(&self, context: &GameContext) -> GameContext {
        context.next_context(self)
    }

    fn play_duration(&self) -> u32 {
        self.play_duration
    }

    fn net_yards(&self) -> i32 {
        self.punt_yards - self.punt_return_yards - self.fumble_return_yards
    }

    fn turnover(&self) -> bool {
        // In this case, turnover means change of possession
        // Usually fumble means turnover but in this case fumble means no change of possession
        !self.fumble
    }

    fn offense_score(&self) -> ScoreResult {
        if self.touchdown && self.fumble {
            return ScoreResult::Touchdown;
        }
        ScoreResult::None
    }

    fn defense_score(&self) -> ScoreResult {
        if self.touchdown && !self.fumble {
            ScoreResult::Touchdown
        }
        ScoreResult::None
    }

    fn offense_timeout(&self) -> bool { false }

    fn defense_timeout(&self) -> bool { false }

    fn incomplete(&self) -> bool { false }

    fn out_of_bounds(&self) -> bool {
        self.out_of_bounds
    }

    fn kickoff(&self) -> bool { false }

    fn next_play_kickoff(&self) -> bool { false }

    fn next_play_extra_point(&self) -> bool {
        self.touchdown
    }
}
