# Pass result module

The `pass` module includes the `PassResult` and `PassResultSimulator` structs.

The `PassResult` struct represents the result of a pass play, like whether the pass was complete or intercepted, and the pass distance and yards after catch. This module also includes a `PassResultBuilder` builder pattern implementation, and a `PassResultRaw` struct used for validating pass result properties before converting into a `PassResult`.

The `PassResultSimulator` generates a `PassResult` using `FootballTeam` and `GameContext` properties.
