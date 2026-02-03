# Conference module

The `conference` module defines the `LeagueConference` and `LeagueDivision` structs which organize teams within a season into a hierarchical conference/division structure. There are also `LeagueConferenceRaw` and `LeagueDivisionRaw` structs used for validating properties before converting via their `TryFrom` trait implementations.

## LeagueDivision struct

A `LeagueDivision` contains the following properties
- `name`: The name of the division (max 64 characters)
- `teams`: The team IDs belonging to the division (a `Vec<usize>`)

Team IDs within a division must be unique. Divisions are validated on deserialization via `LeagueDivisionRaw`.

## LeagueConference struct

A `LeagueConference` contains the following properties
- `name`: The name of the conference (max 64 characters)
- `divisions`: The divisions within the conference (a `Vec<LeagueDivision>`)

Team IDs must be unique across all divisions within a conference. Conferences are validated on deserialization via `LeagueConferenceRaw`.
