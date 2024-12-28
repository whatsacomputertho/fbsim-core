# FBSim Box Score Gen

> An american football box score generator model

## Overview

This crate provides the `BoxScoreGenerator` struct which will generate an american football box score given the normalized skill differential between the home offense and the away defense, and vice versa, the away offense and the home defense.  It is based on the four regression models trained [in this repository](https://github.com/whatsacomputertho/fbdb-boxscore-eda).

## Usage

Below usage 

### Adding via Cargo

To add the package to your project, run the following from your project directory.
```sh
cargo add fbsim_box_score_gen
```

### BoxScoreGenerator instantiation

One can instantiate the generator with normalized differentials defaulted to `0.5_f64` respectively using the `BoxScoreGenerator::new()` method.  It is recommended in this case to instantiate it as mutable so that the setters can be used to set the normalized differentials later.

```rust
// Instantiate the BoxScoreGenerator with default normalized differentials
let mut my_box_score_gen = BoxScoreGenerator::new();

// Then set the normalized differentials later
my_box_score_gen.set_home_off_away_def_norm_diff(0.75_f64).unwrap();
my_box_score_gen.set_away_off_home_def_norm_diff(0.25_f64).unwrap();
```

Or one can instantiate the generator with values explicitly given at instantiation time.
```rust
let my_box_score_gen = BoxScoreGenerator::from_properties(
    0.75_f64,
    0.25_f64
);
```

### Box score generation

Once the generator is instantiated, one can simply call the `gen()` method on it, passing a mutable [`rand::Rng`](https://docs.rs/rand/latest/rand/trait.Rng.html) instance as an argument.

```rust
let mut rng = rand::thread_rng();
let my_box_score_gen = BoxScoreGenerator::new();
let (home_score, away_score): (i32, i32) = my_box_score_gen.gen(&mut rng);
```
