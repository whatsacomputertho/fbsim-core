# Field goal result module

The `fieldgoal` module includes the `FieldGoalResult` and `FieldGoalResultSimulator` structs.

The `FieldGoalResult` struct represents the result of a field goal, like whether the field goal was made, missed, or blocked, and how long it took to execute the play. This module also includes a `FieldGoalResultBuilder` builder pattern implementation, and a `FieldGoalResultRaw` struct used for validating field goal result properties before converting into a `FieldGoalResult`.

The `FieldGoalResultSimulator` generates a `FieldGoalResult` using `FootballTeam` and `GameContext` properties.
