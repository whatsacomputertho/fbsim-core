# Punt result module

The `punt` module includes the `PuntResult` and `PuntResultSimulator` structs.

The `PuntResult` struct represents the result of a punt, like whether the punt was muffed or went out of bounds, the distance of the punt, and the punt return yards. This module also includes a `PuntResultBuilder` builder pattern implementation, and a `PuntResultRaw` struct used for validating punt result properties before converting into a `PuntResult`.

The `PuntResultSimulator` generates a `PuntResult` using `FootballTeam` and `GameContext` properties.
