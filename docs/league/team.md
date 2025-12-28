# Team module

The `team` module defines the `LeagueTeam` struct which is just an empty struct corresponding to a unique ID in the higher-level `League` struct. By doing this, a `LeagueTeam` can correspond to many different `LeagueSeasonTeam` instances whose name and skill levels can differ season-by-season.
