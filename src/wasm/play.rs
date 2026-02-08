//! WASM bridge types for enriched play and drive results.
//!
//! These types provide JavaScript/TypeScript-compatible wrappers around the
//! core fbsim-core Rust types. They are intended exclusively for JS/TS
//! consumers via WebAssembly and are not part of the public Rust API.
//!
//! Feature-gated behind the `wasm` Cargo feature. Compiled to WebAssembly
//! via `wasm-pack`.

use serde::Serialize;
use tsify_next::Tsify;

use crate::game::context::GameContext;
use crate::game::play::result::{PlayResult, PlayTypeResult, ScoreResult};
use crate::game::play::DriveResult;
use crate::game::play::Drive as CoreDrive;
use crate::game::play::Play as CorePlay;

/// Computed values from the `PlayResult` trait.
///
/// This captures all the trait method outputs so JavaScript consumers
/// don't need to reimplement the Rust logic.
#[derive(Clone, Debug, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct PlayResultComputed {
    pub net_yards: i32,
    pub play_duration: u32,
    pub turnover: bool,
    pub offense_score: ScoreResult,
    pub defense_score: ScoreResult,
    pub offense_timeout: bool,
    pub defense_timeout: bool,
    pub incomplete: bool,
    pub out_of_bounds: bool,
    pub touchback: bool,
    pub kickoff: bool,
    pub punt: bool,
    pub next_play_kickoff: bool,
    pub next_play_extra_point: bool,
}

impl From<&PlayTypeResult> for PlayResultComputed {
    fn from(result: &PlayTypeResult) -> Self {
        PlayResultComputed {
            net_yards: result.net_yards(),
            play_duration: result.play_duration(),
            turnover: result.turnover(),
            offense_score: result.offense_score(),
            defense_score: result.defense_score(),
            offense_timeout: result.offense_timeout(),
            defense_timeout: result.defense_timeout(),
            incomplete: result.incomplete(),
            out_of_bounds: result.out_of_bounds(),
            touchback: result.touchback(),
            kickoff: result.kickoff(),
            punt: result.punt(),
            next_play_kickoff: result.next_play_kickoff(),
            next_play_extra_point: result.next_play_extra_point(),
        }
    }
}

/// An enriched play with computed values and display strings.
///
/// Includes the original play data plus:
/// - `result_computed` / `post_play_computed`: all `PlayResult` trait outputs
/// - `description`: the play's `Display` output (e.g. "Rush 5 yards. TOUCHDOWN!")
/// - `context_description`: the context's `Display` output (e.g. "Q1 14:22 1st & 10 ...")
#[derive(Clone, Debug, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Play {
    pub context: GameContext,
    pub result: PlayTypeResult,
    pub result_computed: PlayResultComputed,
    pub post_play: PlayTypeResult,
    pub post_play_computed: PlayResultComputed,
    pub description: String,
    pub context_description: String,
}

impl From<&CorePlay> for Play {
    fn from(play: &CorePlay) -> Self {
        Play {
            context: play.context().clone(),
            result: *play.result(),
            result_computed: PlayResultComputed::from(play.result()),
            post_play: *play.post_play(),
            post_play_computed: PlayResultComputed::from(play.post_play()),
            description: format!("{}", play),
            context_description: format!("{}", play.context()),
        }
    }
}

/// An enriched drive with computed values and display strings.
///
/// Includes enriched `Play` entries, total yards, and the drive's display output.
#[derive(Clone, Debug, Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Drive {
    pub plays: Vec<Play>,
    pub result: DriveResult,
    pub complete: bool,
    pub total_yards: i32,
    pub display: String,
}

impl From<&CoreDrive> for Drive {
    fn from(drive: &CoreDrive) -> Self {
        Drive {
            plays: drive.plays().iter().map(Play::from).collect(),
            result: *drive.result(),
            complete: drive.complete(),
            total_yards: drive.total_yards(),
            display: format!("{}", drive),
        }
    }
}

