# League Simulation

> Feature specification for the FootballSim league simulation capability

- [League Simulation](#league-simulation)
  - [Requirements](#requirements)
    - [League](#league)
    - [LeagueTeam](#leagueteam)
    - [LeagueSeason](#leagueseason)
    - [LeagueSeasonTeam](#leagueseasonteam)
    - [LeagueSeasonWeek](#leagueseasonweek)
    - [LeagueSeasonMatchup](#leagueseasonmatchup)
  - [Architecture](#architecture)
    - [Schema](#schema)
      - [Example](#example)
  - [API Specification](#api-specification)
  - [CLI Specification](#cli-specification)
    - [League](#league-1)
    - [League Team](#league-team)
    - [League Season](#league-season)
    - [League Season Team](#league-season-team)
    - [League Season Week](#league-season-week)
    - [League Season Week Matchup](#league-season-week-matchup)
  - [Roadmap](#roadmap)
  - [Next Steps](#next-steps)

## Requirements

### League

> A `League` is the top-level structure representing a football league over the course of many seasons

1. A `League` MUST be serializable as JSON and YAML
2. A `League` MUST aggregate `LeagueSeason` structures
3. A `League` MAY contain 0 `LeagueSeason` structures
4. A `League` MUST contain at most 1 active `LeagueSeason`
5. A `League` MUST ONLY create a new `LeagueSeason` if it has no active `LeagueSeason`
6. A `League` MUST distinguish between its active `LeagueSeason` and its history of past `LeagueSeason` structures
7. A `League` MUST aggregate `LeagueTeam` structures
8. A `League` MUST compute a unique ID for each of its `LeagueTeam` structures
9.  A `League` MAY add new teams to its collection of `LeagueTeam` structures
10. A `League` MUST NOT remove teams from its collection of `LeagueTeam` structures
11. A `League` MUST ONLY create a new `LeagueSeason` from an even subset (strictly greater than 2) of its `LeagueTeam` structures
12. A `League` MUST be capable of simulating an entire season at once
13. A `League` MUST be capable of computing the historical performance of a given `LeagueTeam`

### LeagueTeam

> A `LeagueTeam` represents a football team over the course of many seasons.  As of writing the spec, it should only consist of a unique ID.

1. A `LeagueTeam` MUST have a unique ID assigned to it
2. A `LeagueTeam` MUST ONLY contain properties which do not vary by season
3. A `LeagueTeam` MAY generate a new `LeagueSeasonTeam` from its properties
4. A `LeagueTeam` SHOULD default its generated `LeagueSeasonTeam` properties to its parent `League` structue's most recent `LeagueSeasonTeam` corresponding to the same unique ID
5. A `LeagueTeam` MAY adjust its generated `LeagueSeasonTeam` properties with respect to its parent `League` structue's most recent `LeagueSeasonTeam` corresponding to the same unique ID

### LeagueSeason

> A `LeagueSeason` represents a single season of a football league

1. A `LeagueSeason` MUST contain an even subset (strictly greater than 2) of `LeagueSeasonTeam` structures generated from its parent `League` structure's `LeagueTeam` structures
2. A `LeagueSeason` MUST be capable of generating a round-robin schedule, represented as a vector of `LeagueSeasonWeek` structures

### LeagueSeasonTeam

> A `LeagueSeasonTeam` represents a football team during a particular season of a football league

1. A `LeagueSeasonTeam` MUST contain a name and logo as properties
2. A `LeagueSeasonTeam` MUST be capable of generating or supplying an offensive and defensive skill level

### LeagueSeasonWeek

> A `LeagueSeasonWeek` represents a week of football matchups during a particular season of a football league

1. A `LeagueSeasonWeek` MAY contain matchups involving a subset of the `LeagueSeasonTeam` structures
2. A `LeagueSeasonWeek` MUST be capable of computing whether all of its matchups are complete

### LeagueSeasonMatchup

> A `LeagueSeasonMatchup` represents a football matchup during a particular week of a particular season of a football league

1. A `LeagueSeasonMatchup` MUST contain exactly 2 `LeagueSeasonTeam` references, one home and one away team
2. A `LeagueSeasonMatchup` MUST contain home score and away score properties
3. A `LeagueSeasonMatchup` MUST contain a property identifying whether the atchup is complete
4. A `LeagueSeasonMatchup` MUST be convertible into a `BoxScore`

## Architecture

### Schema

```
league (League)
  teams (Vec<LeagueTeam>)
    i (LeagueTeam)
      id (int)
      name (String)
      logo (String)
      offense_overall (int)
      defense_overall (int)
  current_season (LeagueSeason)
    year (int)
    teams (Vec<LeagueSeasonTeam>)
      i (LeagueSeasonTeam)
    weeks (Vec<LeagueSeasonWeek>)
      i (LeagueSeasonWeek)
        week (int)
        matchups (Vec<LeagueSeasonMatchup>)
          i (LeagueSeasonMatchup)
            home_team (int)
            away_team (int)
            home_score (int)
            away_score (int)
            complete (bool)
  seasons (Vec<LeagueSeason>)
    i (LeagueSeason)
      year (int)
      teams (Vec<LeagueSeasonTeam>)
        i (LeagueSeasonTeam)
      weeks (Vec<LeagueSeasonWeek>)
        i (LeagueSeasonWeek)
          week (int)
          matchups (Vec<LeagueSeasonMatchup>)
            i (LeagueSeasonMatchup)
              home_team (int)
              away_team (int)
              home_score (int)
              away_score (int)
              complete (bool)
```

#### Example

<details>

<summary>Example League serialized as JSON</summary>

```json
{
  "teams": [
    { "id": 1 },
    { "id": 2 },
    { "id": 3 },
    { "id": 4 },
    { "id": 5 }
  ],
  "current_season": {
    "year": 2025,
    "teams": [
      {
        "id": 1,
        "name": "New York Monsters",
        "logo": "<blob>",
        "offense_overall": 50,
        "defense_overall": 50
      },
      {
        "id": 2,
        "name": "Carolina Wombats",
        "logo": "<blob>",
        "offense_overall": 50,
        "defense_overall": 50
      },
      {
        "id": 3,
        "name": "New England Tulips",
        "logo": "<blob>",
        "offense_overall": 50,
        "defense_overall": 50
      },
      {
        "id": 4,
        "name": "New Orleans Grouse",
        "logo": "<blob>",
        "offense_overall": 50,
        "defense_overall": 50
      }
    ],
    "weeks": [
      {
        "week": 1,
        "matchups": [
          {
            "home_team": 1,
            "away_team": 2,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          },
          {
            "home_team": 3,
            "away_team": 4,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          }
        ]
      },
      {
        "week": 2,
        "matchups": [
          {
            "home_team": 1,
            "away_team": 3,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          },
          {
            "home_team": 2,
            "away_team": 4,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          }
        ]
      },
      {
        "week": 3,
        "matchups": [
          {
            "home_team": 1,
            "away_team": 4,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          },
          {
            "home_team": 2,
            "away_team": 3,
            "home_score": 28,
            "away_score": 14,
            "complete": true
          }
        ]
      }
    ]
  },
  "seasons": []
}
```

</details>

## API Specification

The following API endpoints will be available for the stateless FootballSim league API.  Each `POST` command will accept a JSON-serialized `League` and return the same `League` after making its mutations.  The one `GET` command will simply generate a new default, zeroed-out `League`.

- `GET /v1/stateless/leagues/new`: Get a newly created league
- `POST /v1/stateless/leagues/teams`: List the teams belonging to a league
- `POST /v1/stateless/leagues/teams/new`: Add a new team to an existing league
- `POST /v1/stateless/leagues/teams/<id>`: Display historical information about a team in the league 
- `POST /v1/stateless/leagues/seasons`: List the seasons belonging to a league
- `POST /v1/stateless/leagues/seasons/create`: Create a new season in a league
- `POST /v1/stateless/leagues/seasons/<year>/sim`: Simulate a league season in its entirety
- `POST /v1/stateless/leagues/seasons/<year>/teams`: List the teams belonging to a league season
- `POST /v1/stateless/leagues/seasons/<year>/teams/<id>`: Display information about a team belonging to a league season
- `POST /v1/stateless/leagues/seasons/<year>/weeks`: List the weeks belonging to a league
- `POST /v1/stateless/leagues/seasons/<year>/weeks/<id>`: Display information about a week of a league season
- `POST /v1/stateless/leagues/seasons/<year>/weeks/<id>/sim`: Simulate a league season week in its entirety
- `POST /v1/stateless/leagues/seasons/<year>/weeks/<id>/matchups`: List the matchups for a league season week
- `POST /v1/stateless/leagues/seasons/<year>/weeks/<id>/matchups/<id>`: Display information about a matchup for a league season week
- `POST /v1/stateless/leagues/seasons/<year>/weeks/<id>/matchups/<id>/sim`: Simulate a matchup belonging to a league season week

## CLI Specification

The following subcommand hierarchy will be implemented in support of league management & simulation
- fbsim (root command)
  - league
    - create
    - team
      - add
      - get
      - list
    - season
      - create
      - list
      - sim
      - team
        - get
        - list
      - week
        - get
        - list
        - sim
        - matchup
          - get
          - list
          - sim

### League

The expected usage for the `fbsim league` subcommand is as follows

```
Manage FootballSim leagues

Usage: fbsim league <COMMAND>

Commands:
  create   Create a new FootballSim league
  season   Manage seasons for an existing FootballSim league
  team     Manage teams for an existing FootballSim league

Options:
  -h, --help     Print help
  -o, --output   The output format of the newly created league
  -f, --file     The destination filepath of the newly created league
```

### League Team

The expected usage for the `fbsim league team` subcommand is as follows

```
Manage teams for an existing FootballSim league

Usage: fbsim league team <COMMAND>

Commands:
  add    Add a new team to the FootballSim league
  get    Display historical information about a team in the league
  list   List all teams in the league

Options:
  -h, --help     Print help
  -o, --output   The output format for the team history command
  -l, --league   The input filepath for the league
  -t, --team     The ID of the team to manage
```

### League Season

The expected usage for the `fbsim league season` subcommand is as follows

```
Manage seasons for an existing FootballSim league

Usage: fbsim league season <COMMAND>

Commands:
  create   Create a new season for the FootballSim league
  list     List all past and current seasons for the FootballSim league
  sim      Simulate the current season in its entirety
  team     Manage teams for a season of a FootballSim league
  week     Manage weeks for a season of a FootballSim league

Options:
  -h, --help     Print help
  -o, --output   The output format for the season list command
  -l, --league   The input filepath for the league
  -s, --season   The year of the season to manage
```

### League Season Team

The expected usage for the `fbsim league season team` subcommand is as follows

```
Manage teams for a season of a FootballSim league

Usage: fbsim league season team <COMMAND>

Commands:
  get    Display information about a team for a season
  list   List all teams for a season

Options:
  -h, --help     Print help
  -o, --output   The output format for the team get command
  -l, --league   The input filepath for the league
  -s, --season   The year of the season to manage
  -t, --team     The ID of the team to manage
```

### League Season Week

The expected usage for the `fbsim league season week` subcommand is as follows

```
Manage weeks for a season of a FootballSim league

Usage: fbsim league season week <COMMAND>

Commands:
  get       Display information about a week of a season
  list      List all weeks of a season
  matchup   Manage matchups for a week of a FootballSim season
  sim       Simulate the week in its entirety

Options:
  -h, --help     Print help
  -o, --output   The output format for each week subcommand
  -l, --league   The input filepath for the league
  -s, --season   The year of the season to manage
  -w, --week     The ID of the week to manage
```

### League Season Week Matchup

The expected usage for the `fbsim league season week matchup` subcommand is as follows

```
Manage matchups for a week of a FootballSim season

Usage: fbsim league season week matchup <COMMAND>

Command:
  get    Display information about a matchup
  list   List all matchups for a week
  sim    Simulate the matchup

Options:
  -h, --help      Print help
  -o, --output    The output format for each matchup subcommand
  -l, --league    The input filepath for the league
  -s, --season    The year of the season to manage
  -w, --week      The ID of the week to manage
  -m, --matchup   The ID of the matchup to manage
```

## Roadmap

At a high-level, my view of the roadmap is:
1. We iteratively develop the aforementioned CLI subcommands in the `fbsim` CLI, in the process developing re-usable functionality in `fbsim-core`
2. We revise the above CLI and/or API specifications based on new findings in the process of developing the CLI
3. We fully implement the above CLI specification in the `fbsim` CLI and it functions as required
4. We iteratively develop equivalent endpoints in the stateless `fbsim` API alongside new corresponding pages and web components in the `fbsim` UI
5. We revise the above CLI and/or API specifications based on new findings in the process of developing the API and UI
6. We fully implement the stateless `fbsim` API and UI.  We have a full-fledged user experience in the UI in which users upload their league, modify it in the UI, then download their league to store it on the client side

## Next Steps

Out of scope for this initial iteration, but which may come in future iterations
- **Authentication**: The API and UI are authenticated and user-based
- **Stateful API**: The API has an ORM layer atop a relational database, the UI can toggle between stateful and stateless mode
- **PWA Experience**: The stateless API can compile into WASM and be executed on the client side without making networked API calls, this is handled seamlessly in the UI (not sure if we want this to be dynamic or user-driven)
