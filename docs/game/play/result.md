# Result module

The `result` module includes various types which are central to the simulation of play-by-play games.

## Result traits

The `PlayResult` trait defines a set of methods used to generate the next `GameContext` given the result of a play. The `PlayResultSimulator` trait defines a single `sim` method that generates a `PlayResult` implementation. These traits are implemented by each of the result and result simulator structs belonging to the submodules of this module.

## Result enums

The `PlayTypeResult` enum is a generalization across each of the result structs belonging to the submodules of this module. The `ScoreResult` enum enumerates the various ways in which a team can score points. There are two methods in the `PlayResult` trait which return instances of the `ScoreResult` enum.
