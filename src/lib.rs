#![doc = include_str!("../README.md")]
pub mod game;
pub mod league;
pub mod team;

#[cfg(feature = "wasm")]
pub mod wasm;
