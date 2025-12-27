# Defense module

The `defense` module implements the `FootballTeamDefense` struct which represents a coach. The `FootballTeamDefenseRaw` struct implements a `validate` method as well as a `TryFrom` trait implementation for `FootballTeamDefenseRaw -> FootballTeamDefense` in which the `FootballTeamDefenseRaw` properties are validated before the type conversion.

## Defense skill levels

Each of the following skill levels range from 0 to 100, and a higher value implies the defense is better in that skill
- `blitzing`: Controls how likely the defense is to pressure or sack the quarterback on a pass play
- `rush_defense`: Controls the average rushing yards against
- `pass_defense`: Controls how unlikely the quarterback is to complete a pass
- `coverage`: Controls the average yards after catch against
- `turnovers`: Controls how likely the defense is to force a turnover
- `kick_returning`: Controls the average kick / punt return yards when returning a kick or punt
