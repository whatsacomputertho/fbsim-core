# Playoffs module

The `playoffs` module defines the `LeagueSeasonPlayoffs`, `PlayoffTeams`, and `PlayoffTeam` structs which represent the postseason bracket structure for a league season. There is also a `PlayoffTeamRaw` struct used for validating playoff team properties before converting via its `TryFrom` trait implementation. The module also contains the `picture` submodule for computing real-time playoff standings.

## PlayoffTeam struct

A `PlayoffTeam` contains the following properties
- `seed`: The team's seed within its conference bracket (a `usize`, auto-assigned)
- `short_name`: The team's short name / acronym (max 4 characters)

## PlayoffTeams struct

A `PlayoffTeams` maps conference IDs to their playoff rosters. It contains a `BTreeMap<usize, BTreeMap<usize, PlayoffTeam>>` keyed by conference ID, then by team ID. Team IDs must be unique across all conferences.

## LeagueSeasonPlayoffs struct

A `LeagueSeasonPlayoffs` contains the following properties
- `teams`: The playoff rosters (a `PlayoffTeams`)
- `conference_brackets`: The bracket rounds per conference (a `BTreeMap<usize, Vec<LeagueSeasonWeek>>`)
- `winners_bracket`: The championship bracket rounds (a `Vec<LeagueSeasonWeek>`)

In single-conference mode, all rounds use conference bracket 0 and the winners bracket is empty. In multi-conference mode, conference champions advance to the winners bracket for the championship.
