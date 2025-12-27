# Game context module

The `context` module includes the `GameContext` struct (and related structs) which represents a game situation. It includes properties defining the time on the clock, the score, the yard line, the down and distance, and more.

## GameContext

The `GameContext` struct includes the following as properties
- The home & away team short names / acronyms
- The quarter & seconds remaining in the half
- The current down, distance, and yard line
- The home & away score
- The home & away timeouts remaining
- Whether the home team has possession, received the opening kick, and is moving in a positive direction
- Whether the last play was a turnover, out of bounds, timeout, kickoff, or punt
- Whether the next play will be an extra point or kickoff
- Whether this is the end of the half, or the end of the game

It also includes methods for deriving the next context / next context properties given a result of a play.

## Validation

The `GameContextRaw` struct includes the same properties as `GameContext`, but a single method `validate` which validates the game context properties. The `TryFrom` trait is then implemented for `GameContextRaw -> GameContext` which errors if `validate` fails, or returns a `GameContext` if it doesn't fail. This is used across all the game context constructors to ensure `GameContext` instances are always valid in memory.

## Builder

The `GameContextBuilder` struct implements the builder pattern for the `GameContext` struct. Here is an example of its use in whcih the opening kickoff is randomized.
```rust
use fbsim_core::game::context::{GameContext, GameContextBuilder};

// Initialize a new context with randomized opening kickoff
let mut rng = rand::thread_rng();
let home_opening_kickoff: bool = rng.gen::<bool>();
let context: GameContext = GameContextBuilder::new()
    .home_possession(!home_opening_kickoff)
    .home_positive_direction(!home_opening_kickoff)
    .home_opening_kickoff(home_opening_kickoff)
    .build()
    .unwrap();
```
