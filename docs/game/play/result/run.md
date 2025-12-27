# Run result module

The `run` module includes the `RunResult` and `RunResultSimulator` structs.

The `RunResult` struct represents the result of a run play, like whether there was a fumble on the run play, and the rushing yards on the play. This module also includes a `RunResultBuilder` builder pattern implementation, and a `RunResultRaw` struct used for validating run result properties before converting into a `RunResult`.

The `RunResultSimulator` generates a `RunResult` using `FootballTeam` and `GameContext` properties.
