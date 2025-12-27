# Between play result module

The `betweenplay` module includes the `BetweenPlayResult` and `BetweenPlayResultSimulator` structs.

The `BetweenPlayResult` struct represents the events which occur between the play, like the clock running while the offense gets ready to run a play, or timeouts called by either team after the play. This module also includes a `BetweenPlayResultBuilder` builder pattern implementation, and a `BetweenPlayResultRaw` struct used for validating between play result properties before converting into a `BetweenPlayResult`.

The `BetweenPlayResultSimulator` generates a `BetweenPlayResult` using `FootballTeamCoach` and `PlayContext` and `GameContext` properties.
