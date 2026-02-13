# FBSim Core

[![Build](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/build.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/build.yaml) [![Test](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/test.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/test.yaml) [![Lint](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/lint.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/lint.yaml) [![Securty](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/sec.yaml/badge.svg)](https://github.com/whatsacomputertho/fbsim-core/actions/workflows/sec.yaml)

> A library for american football simulation

**Rust Docs**: [`fbsim-core` Rust crate documentation](https://docs.rs/fbsim-core/latest/fbsim_core/)

**TypeDocs**: [`fbsim-core` TypeScript & JavaScript WASM module documentation](https://whatsacomputertho.github.io/fbsim-core/)

**Contributing**: [CONTRIBUTING.md](https://github.com/whatsacomputertho/fbsim-core/blob/main/CONTRIBUTING.md)

## Overview

Provides utilities for simulating american football games and leagues. The core library is written in Rust; it also compiles into a WebAssembly module for use in JavaScript and TypeScript projects. It is based on various statistical models derived in repositories
- [whatsacomputertho/nfl-pbp-eda](https://github.com/whatsacomputertho/nfl-pbp-eda)
- [whatsacomputertho/fbdb-boxscore-eda](https://github.com/whatsacomputertho/fbdb-boxscore-eda)

## Example

Below is a quick example of running a play-by-play football simulation using `fbsim-core`. It includes equivalent examples, one in Rust using the `fbsim-core` Rust crate, the other in JavaScript using the `fbsim-core` NPM package.

### Rust

Here is a quick example of simulating a play-by-play game between two teams in Rust.

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

### JavaScript & TypeScript

The same play-by-play simulation is available in JavaScript and TypeScript via WebAssembly.

```typescript
import init, {
  FootballTeam,
  Game,
  GameSimulator,
  Rng,
  createGameContext,
} from "@whatsacomputertho/fbsim-core";

// Initialize the WASM module
await init();

// Create teams with overall offensive & defensive ratings
const home = new FootballTeam();
const away = new FootballTeam();

// Set up game state
const rng = new Rng();
const simulator = new GameSimulator();
const game = new Game();
let ctx = createGameContext();

// Simulate play-by-play and print the game log
while (!game.complete) {
  ctx = simulator.simPlay(home, away, ctx, game, rng);
  const play = game.getLatestPlay();
  console.log(play.description);
}
console.log("Game Over");
```

## Installing

### Rust

```sh
cargo add fbsim-core
```

### JavaScript & TypeScript

```sh
npm install @whatsacomputertho/fbsim-core
```
