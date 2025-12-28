# Team module

The `team` module defines the `FootballTeam` struct and its sub-structs.

# FootballTeam struct

The `FootballTeam` struct represents a football team. The `FootballTeamRaw` struct implements a `validate` method as well as a `TryFrom` trait implementation for `FootballTeamRaw -> FootballTeam` in which the `FootballTeamRaw` properties are validated before the type conversion.

# Team sub-structs

A `FootballTeam` is made up of the following sub-structs as struct properties
- `FootballTeamCoach`: (`coach` module) Represents the coach's playcalling / decision making behavior
- `FootballTeamOffense`: (`offense` module) Represents the offense's skill levels
- `FootballTeamDefense`: (`defense` module) Represents the defense's skill levels
