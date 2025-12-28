# Game module

The `game` module ties together various submodules relevant to both play-by-play game simulation, and final score simulation.

## Play-by-play sim

The `context` submodule defines the `GameContext` type which stores various properties defining the game situation, like score, clock, down and distance, etcetera.

The `play` submodule defines the `Game` and `GameSimulator` types which are the highest-level types used for game simulation. It also defines lower-level game simulation types including `Drive` and `DriveSimulator`, `Play` and `PlaySimulator`.

The `stat` submodule defines various game statistics types including `PassingStats`, `RushingStats`, and `ReceivingStats`. Each of these stat types can be derived from a `Game` or `Drive`.

## Final score sim

The `score` submodule defines the `FinalScore` and `FinalScoreSimulator` types which are used to generate just the final score of a game rather than a full play-by-play game log.

The `matchup` submodule defines the `FootballMatchup` struct and `FootballMatchupResult` enum which are mainly just used in the API to define the input payload for the final score simulator.
