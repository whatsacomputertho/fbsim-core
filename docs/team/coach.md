# Coach module

The `coach` module implements the `FootballTeamCoach` struct which represents a coach. The `FootballTeamCoachRaw` struct implements a `validate` method as well as a `TryFrom` trait implementation for `FootballTeamCoachRaw -> FootballTeamCoach` in which the `FootballTeamCoachRaw` properties are validated before the type conversion.

## Decision making properties

- `risk_taking`: How likely the coach is to go for it on 4th down
- `run_pass`: The run:pass playcalling ratio; a greater value implies more run plays
- `up_tempo`: How likely the offense is to go up-tempo between plays in non-clock-management scenarios
