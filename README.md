# FBSim Core

> A library for american football simulation

## Overview

In its initial iteration, this crate provides utilities for simulating american football leagues at the box score level.  It is based on the four regression models derived [in this repository](https://github.com/whatsacomputertho/fbdb-boxscore-eda).

## Usage

### Adding via Cargo

To add the package to your project, run the following from your project directory.
```sh
cargo add fbsim-core
```

### Box score simulator

To simulate a game using the box score simulator, one might replicate the following example.  Note that `FootballTeam` and `BoxScore` both derive the serde `Serialize` and `Deserialize` traits and thus may be instantiated from JSON.

```rust
use fbsim_core::sim::BoxScoreSimulator;
use fbsim_core::team::FootballTeam;

// Instantiate the simulator
let my_box_score_sim = BoxScoreSimulator::new();

// Instantiate the home and away team
let home_team = FootballTeam::from_properties(
    "Home Team",
    75,
    67
).unwrap();
let away_team = FootballTeam::from_properties(
    "Away Team",
    88,
    95
).unwrap();

// Instantiate an RNG and simulate
let mut rng = rand::thread_rng();
let my_box_score = my_box_score_sim.sim(
    &home_team,
    &away_team,
    &mut rng
);
println!("{}", my_box_score);
```
