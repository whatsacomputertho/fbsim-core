# Play module

The `play` module includes various types for simulating plays, drives, and full games.

## Play simulation

The `Play` struct takes ownership over the initial `GameContext` that was used to generate it. It also contains two `PlayTypeResult` enum instances, one is the result of the play, the other is the post-play result and is guaranteed to contain a `BetweenPlayResult`.

The `PlaySimulator` struct can be used to generate a new `Play` given the home and away teams, an initial `GameContext`, and an RNG.

## Drive simulation

The `DriveResult` enum represents the result of a drive.

The `Drive` struct conains a vector of `Play` instances, a `DriveResult` instance, and a boolean property representing whether the drive has completed.

The `DriveSimulator` struct can be used to generate a new `Drive` given the home and away teams, an initial `GameContext`, and an RNG. It can also append new plays onto an existing mutably borrowed `Drive` which has not yet completed.

## Game simulation

The `Game` struct contains a vector of `Drive` instances.

The `GameSimulator` struct can be used to generate a new `Game` given the home and away teams, an initial `GameContext`, and an RNG. It can also append new drives onto an existing mutably borrowed `Game` which has not yet completed, and new plays onto the latest `Drive` in the mutably borrowed `Game` which is still in-progress.
