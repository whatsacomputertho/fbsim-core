# FBSim Core

[![Build](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/build.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/build.yaml) [![Test](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/test.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/test.yaml) [![Lint](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/lint.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/lint.yaml) [![Securty](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/sec.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/sec.yaml)

> A library for american football simulation

**Docs**: https://docs.rs/fbsim-core/latest/fbsim_core/

**Contributing**: [CONTRIBUTING.md](https://github.com/whatsacomputertho/fbsim-core/blob/main/CONTRIBUTING.md)

## Overview

Provides utilities for simulating american football games and leagues. It is based on various statistical models derived in repositories
- [whatsacomputertho/nfl-pbp-eda](https://github.com/whatsacomputertho/nfl-pbp-eda)
- [whatsacomputertho/fbdb-boxscore-eda](https://github.com/whatsacomputertho/fbdb-boxscore-eda)

## Play-by-play sim

Here is a quick example of simulating a play-by-play game between two teams.

```rust
use fbsim_core::game::context::GameContext;
use fbsim_core::game::play::GameSimulator;
use fbsim_core::team::FootballTeam;

// Instantiate the home and away teams, game context
let home_team = FootballTeam::new();
let away_team = FootballTeam::new();
let context = GameContext::new();

// Instantiate the rng, simulator, and simulate the game
let mut rng = rand::thread_rng();
let sim = GameSimulator::new();
let (game, next_context) = sim.sim(&home_team, &away_team, context, &mut rng).unwrap();

// Print the game log
println!("{}", game);
println!("{} Game over", next_context);
```

## Final score sim

There is also a less in-depth simulator that produces a final score given two teams. Here is a quick example of its usage.

```rust
use fbsim_core::game::score::FinalScoreSimulator;
use fbsim_core::team::FootballTeam;

// Instantiate the home and away teams
let home_team = FootballTeam::new();
let away_team = FootballTeam::new();

// Instantiate the rng, simulator, and simulate the game
let mut rng = rand::thread_rng();
let sim = FinalScoreSimulator::new();
let final_score = sim.sim(&home_team, &away_team, &mut rng).unwrap();

// Print the final score
println!("{}", final_score);
```

## Installing

To add the package to your project, run the following from your project directory.

```sh
cargo add fbsim-core
```
