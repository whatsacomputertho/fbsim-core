# Offense module

The `offense` module implements the `FootballTeamOffense` struct which represents a coach. The `FootballTeamOffenseRaw` struct implements a `validate` method as well as a `TryFrom` trait implementation for `FootballTeamOffenseRaw -> FootballTeamOffense` in which the `FootballTeamOffenseRaw` properties are validated before the type conversion.

## Offense skill levels

Each of the following skill levels range from 0 to 100, and a higher value implies the offense is better in that skill
- `passing`: Controls how likely the quarterback is to complete a pass
- `blocking`: Controls how unlikely the offensive line is to allow a pressure on a pass play
- `rushing`: Controls the average rushing yards and likelihood of a big rushing play
- `receiving`: Controls the average yards after catch
- `scrambling`: Controls how likely the quarterback is to scramble under pressure, and how good the quarterback is at scrambling
- `turnovers`: Controls how unlikely the offense is at turning the ball over
- `field_goals`: Controls how likely the kicker is to make a field goal
- `punting`: Controls the average distance of a punt and the likelihood of landing a punt inside the 20
- `kickoffs`: Controls the average distance of a kickoff and the likelihood of a touchback or landing a kick inside the 20
- `kick_return_defense`: Controls the average kick / punt return yards against when defending against a kickoff or punt return
