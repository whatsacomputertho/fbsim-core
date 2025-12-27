# Kickoff result module

The `kickoff` module includes the `KickoffResult` and `KickoffResultSimulator` structs.

The `KickoffResult` struct represents the result of a kickoff, like whether the kickoff resulted in a touchback, the distance of the kick, and the return yards. This module also includes a `KickoffResultBuilder` builder pattern implementation, and a `KickoffResultRaw` struct used for validating kickoff result properties before converting into a `KickoffResult`.

The `KickoffResultSimulator` generates a `KickoffResult` using `FootballTeam` and `GameContext` properties.
