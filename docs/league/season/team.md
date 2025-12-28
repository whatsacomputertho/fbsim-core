# Team module

The `team` module defines the `LeagueSeasonTeam` struct which represents a team during a football season. There is also a `LeagueSeasonTeamRaw` struct used for validating league season properties before converting from `LeagueSeasonTeamRaw -> LeagueSeasonTeam` via its `TryFrom` trait implementation.
