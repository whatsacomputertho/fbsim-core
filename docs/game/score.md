# Score module

The `score` module includes structs for generating only the final score of a game without generating a full play-by-play game log.

## Final score sim

The `FinalScore` struct represents the final score of a game, and includes both of the team names of the teams involved in the game. There are `FinalScoreRaw` and `FinalScoreBuilder` implementations which are used for validating final score instances, and creating final score instances via the builder pattern.

The `FinalScoreSimulator` struct generates the final score of a game given the home and away team, and an RNG.
