# Season module

The `season` module defines the `LeagueSeason` struct which represents a full season in a league. There is also a `LeagueSeasonRaw` struct used for validating league season properties before converting from `LeagueSeasonRaw -> LeagueSeason` via its `TryFrom` trait implementation. The module also defines `LeagueSeasonScheduleOptions` and `LeagueSeasonPlayoffOptions` for configuring schedule generation and playoff generation respectively.

## LeagueSeason struct

A `LeagueSeason` contains the following properties
- `year`: The year in which the season takes place
- `teams`: The teams which participated in the season (a `BTreeMap<usize, FootballTeam>`)
- `conferences`: The conference/division structure for the season (a `Vec<LeagueConference>`)
- `weeks`: The weeks of matchups in the season (a `Vec<LeagueSeasonWeek>`)
- `playoffs`: The postseason bracket structure (a `LeagueSeasonPlayoffs`)
