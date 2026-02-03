# League Simulation

> Feature specification for the FootballSim league simulation capability

- [League Simulation](#league-simulation)
  - [Requirements](#requirements)
    - [League](#league)
    - [LeagueTeam](#leagueteam)
    - [LeagueSeason](#leagueseason)
    - [FootballTeam](#footballteam)
    - [LeagueConference](#leagueconference)
    - [LeagueDivision](#leaguedivision)
    - [LeagueSeasonWeek](#leagueseasonweek)
    - [LeagueSeasonMatchup](#leagueseasonmatchup)
    - [LeagueSeasonPlayoffs](#leagueseasonplayoffs)
    - [PlayoffTeam](#playoffteam)
    - [PlayoffPicture](#playoffpicture)
    - [LeagueTeamRecord](#leagueteamrecord)
  - [Architecture](#architecture)
    - [Type Hierarchy](#type-hierarchy)
    - [Schema](#schema)
      - [Example](#example)
  - [API Specification](#api-specification)
  - [CLI Specification](#cli-specification)
    - [League](#league-1)
    - [League Team](#league-team)
    - [League Team Stats](#league-team-stats)
    - [League Season](#league-season)
    - [League Season Conference](#league-season-conference)
    - [League Season Conference Division](#league-season-conference-division)
    - [League Season Team](#league-season-team)
    - [League Season Team Stats](#league-season-team-stats)
    - [League Season Standings](#league-season-standings)
    - [League Season Schedule](#league-season-schedule)
    - [League Season Week](#league-season-week)
    - [League Season Week Matchup](#league-season-week-matchup)
    - [League Season Week Matchup Play](#league-season-week-matchup-play)
    - [League Season Playoffs](#league-season-playoffs)
    - [League Season Playoffs Round](#league-season-playoffs-round)
    - [League Season Playoffs Round Matchup](#league-season-playoffs-round-matchup)
  - [Roadmap](#roadmap)
  - [Next Steps](#next-steps)

## Requirements

### League

> A `League` is the top-level structure representing a football league over the course of many seasons

1. A `League` MUST be serializable as JSON and YAML
2. A `League` aggregates `LeagueSeason` structures
3. A `League` MAY contain 0 `LeagueSeason` structures
4. A `League` MUST contain at most 1 active `LeagueSeason`
5. A `League` MUST ONLY create a new `LeagueSeason` if it has no active `LeagueSeason`
6. A `League` MUST distinguish between its active `LeagueSeason` and its history of past `LeagueSeason` structures
7. A `League` MUST aggregate `LeagueTeam` structures
8. A `League` MUST compute a unique ID for each of its `LeagueTeam` structures
9. A `League` MAY add new teams to its collection of `LeagueTeam` structures
10. A `League` MUST NOT remove teams from its collection of `LeagueTeam` structures
11. A `League` MUST validate that all teams that appear in a season exist in its collection of `LeagueTeam` structures
12. A `League` MUST be capable of simulating an entire season at once
13. A `League` MUST be capable of computing the historical performance of a given `LeagueTeam`

### LeagueTeam

> A `LeagueTeam` represents a football team over the course of many seasons

1. A `LeagueTeam` MUST have a unique ID assigned to it by its parent `League`
2. A `LeagueTeam` MUST ONLY contain properties which do not vary by season

### LeagueSeason

> A `LeagueSeason` represents a single season of a football league

1. A `LeagueSeason` aggregates `FootballTeam`, `LeagueConference`, and `LeagueSeasonWeek` structures
2. A `LeagueSeason` contains `LeagueSeasonPlayoffs` structures
3. A `LeagueSeason` MAY contain 0 `FootballTeam` structures
4. A `LeagueSeason` MUST contain an even number (greater than or equal to 4) of `FootballTeam` structures before generating its schedule
5. A `LeagueSeason` MAY contain 0 `LeagueConference` structures
6. A `LeagueSeason` MUST contain at least one `LeagueConference` structure before generating its schedule
7. A `LeagueSeason` MAY contain 0 `LeagueSeasonWeek` structures
8. A `LeagueSeason` MUST NOT populate its `LeagueSeasonPlayoffs` structure until all of its `LeagueSeasonWeek` structures contain completed matchups
9. A `LeagueSeason` MUST validate that the number of scheduled weeks, if nonzero, is between `num_teams` and `(num_teams - 1) * 3`
10. A `LeagueSeason` MUST validate that all teams that appear in a conference exist in its collection of teams and that none are duplicated across conferences
11. A `LeagueSeason` MUST validate that weeks progress sequentially (week N cannot start before week N-1)
12. A `LeagueSeason` MUST validate that each team plays at most once per week
13. A `LeagueSeason` MUST be capable of generating a schedule customizable by number of division, in-conference, and out-of-conference games
14. A `LeagueSeason` MUST be capable of computing league, conference, and division standings
15. A `LeagueSeason` MUST be capable of computing a playoff picture identifying teams that have clinched the playoffs, or that have been eliminated customizable by number of teams (in total or per conference) that make the playoffs
16. A `LeagueSeason` MUST be capable of generating and simulating playoffs customizable by number of teams (in total or per conference) that make the playoffs
17. A `LeagueSeason` MUST validate that its conference playoff bracket IDs each correspond to valid conference IDs

### FootballTeam

> A `FootballTeam` represents the season-specific attributes of a league team

1. A `FootballTeam` has a name, acronym, `FootballTeamCoach`, `FootballTeamOffense`, and `FootballTeamDefense`
2. A `FootballTeam` MUST validate that its name is at most 64 characters
3. A `FootballTeam` MUST validate that its acronym at most 4 characters

### FootballTeamCoach

> A `FootballTeamCoach` represents the coach of a football team

1. A `FootballTeamCoach` has various numeric coaching behavior attributes
2. A `FootballTeamCoach` MUST validate that its attributes are in range `[0, 100]`

### FootballTeamOffense

> A `FootballTeamOffense` represents the offense of a football team

1. A `FootballTeamOffense` has various numeric skill attributes
2. A `FootballTeamOffense` MUST validate that its attributes are in range `[0, 100]`

### FootballTeamDefense

> A `FootballTeamDefense` represents the defense of a football team

1. A `FootballTeamDefense` has various numeric skill attributes
2. A `FootballTeamDefense` MUST validate that its attributes are in range `[0, 100]`

### LeagueConference

> A `LeagueConference` groups teams within a season into a named conference containing one or more divisions

1. A `LeagueConference` has a name and a collection of `LeagueDivision` structures
2. A `LeagueConference` MUST validate that its name is at most 64 characters
3. A `LeagueConference` MUST validate that no team is duplicated across its collection of divisions

### LeagueDivision

> A `LeagueDivision` groups teams within a conference into a named division

1. A `LeagueDivision` has a name and a collection of team IDs
2. A `LeagueDivision` MUST validate that its name is at most 64 characters
3. A `LeagueDivision` MUST validate that no team ID is duplicated in its collection of team IDs

### LeagueSeasonWeek

> A `LeagueSeasonWeek` represents a week of football matchups during a particular season

1. A `LeagueSeasonWeek` MUST contain a vector of `LeagueSeasonMatchup` structures
2. A `LeagueSeasonWeek` MUST be capable of computing whether it has started and whether it has completed
3. A `LeagueSeasonWeek` MUST be capable of returning a specific team's matchup for the week

### LeagueSeasonMatchup

> A `LeagueSeasonMatchup` represents a football game between two teams during a season

1. A `LeagueSeasonMatchup` MUST contain the full game log when a game is in-progress
2. A `LeagueSeasonMatchup` MUST contain the final game stats after a game is complete
3. A `LeagueSeasonMatchup` MUST be capable of producing a matchup result (Win, Loss, or Tie) for a given team

### LeagueSeasonPlayoffs

> `LeagueSeasonPlayoffs` represents the postseason bracket structure for a season

1. A `LeagueSeasonPlayoffs` has a collection of `PlayoffTeam` structures organized by conference
2. A `LeagueSeasonPlayoffs` MUST auto-increment its `PlayoffTeam` structures' seeds as they are added
3. A `LeagueSeasonPlayoffs` has a winners bracket and a collection of conference brackets identified by conference ID, which themselves each have a collection of `LeagueSeasonWeek` structures representing the rounds of the playoff bracket
4. A `LeagueSeasonPlayoffs` MUST validate that each team ID that appears across each of its brackets exist in its collection of `PlayoffTeam` structures
5. A `LeagueSeasonPlayoffs` MUST validate that no duplicate team IDs appear across conferences or conference brackets
5. A `LeagueSeasonPlayoffs` MUST support both single-conference and multi-conference league playoffs
6. A `LeagueSeasonPlayoffs` MUST advance conference champions to a winners bracket for multi-conference playoffs
7. A `LeagueSeasonPlayoffs` MAY also support single-bracket playoffs for multi-conference leagues

### PlayoffTeam

> A `PlayoffTeam` represents a team's entry in the playoffs with seeding information

1. A `PlayoffTeam` has a seed and a team acronym
2. A `PlayoffTeam` MUST validate that its acronym is at most 4 characters

## Architecture

### Type Hierarchy

The following is a UML-style diagram depicting the type/struct hierarchy for a `League`.

```
┌──────────────────────────┐
│          League          │
├──────────────────────────┤
│ teams: BTreeMap<usize,   │
│         LeagueTeam>      │
│ current_season:          │
│         Option<Season>   │
│ seasons: Vec<Season>     │
└────┬────────────┬────────┘
     │            │
     ▼            ▼
┌──────────┐  ┌──────────────────────────────────────────────┐
│LeagueTeam│  │              LeagueSeason                    │
├──────────┤  ├──────────────────────────────────────────────┤
│ (empty)  │  │ year: usize                                  │
└──────────┘  │ teams: BTreeMap<usize, FootballTeam>         │
              │ conferences: Vec<LeagueConference>           │
              │ weeks: Vec<LeagueSeasonWeek>                 │
              │ playoffs: LeagueSeasonPlayoffs               │
              └───┬──────────┬──────────┬───────────┬────────┘
                  │          │          │           │
     ┌────────────┘          │          │           └──────────────┐
     ▼                       ▼          ▼                          ▼
┌──────────────────┐ ┌─────────────┐ ┌─────────────────┐ ┌────────────────────────┐
│   FootballTeam   │ │  League     │ │ LeagueSeason    │ │  LeagueSeasonPlayoffs  │
├──────────────────┤ │  Conference │ │ Week            │ ├────────────────────────┤
│ name: String     │ ├─────────────┤ ├─────────────────┤ │ teams: PlayoffTeams    │
│ short_name:      │ │ name: String│ │ matchups: Vec<  │ │ conference_brackets:   │
│         String   │ │ divisions:  │ │  LeagueSeason   │ │   BTreeMap<usize,      │
│ coach:           │ │  Vec<League │ │  Matchup>       │ │   Vec<LeagueSeason     │
│  FBTeamCoach     │ │  Division>  │ └────────┬────────┘ │   Week>>               │
│ offense:         │ └──────┬──────┘          │          │ winners_bracket:       │
│  FBTeamOffense   │        │                 ▼          │   Vec<LeagueSeasonWeek>│
│ defense:         │        ▼          ┌──────────────┐  └───────────┬────────────┘
│  FBTeamDefense   │ ┌─────────────┐   │ LeagueSeason │              │
└──────────────────┘ │  League     │   │ Matchup      │              ▼
                     │  Division   │   ├──────────────┤       ┌─────────────┐
                     ├─────────────┤   │ home_team:   │       │PlayoffTeams │
                     │ name: String│   │       usize  │       ├─────────────┤
                     │ teams:      │   │ away_team:   │       │ teams:      │
                     │  Vec<usize> │   │       usize  │       │  BTreeMap<  │
                     └─────────────┘   │ context:     │       │  usize,     │
                                       │  GameContext │       │  BTreeMap<  │
                                       │ game:        │       │  usize,     │
                                       │  Option<Game>│       │  PlayoffTeam│
                                       │ home_stats:  │       │  >>         │
                                       │  Option<     │       └──────┬──────┘
                                       │  Offensive   │              │
                                       │  Stats>      │              ▼
                                       │ away_stats:  │       ┌─────────────┐
                                       │  Option<     │       │ PlayoffTeam │
                                       │  Offensive   │       ├─────────────┤
                                       │  Stats>      │       │ seed: usize │
                                       └──────────────┘       │ short_name: │
                                                              │      String │
                                                              └─────────────┘

┌──────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│  FBTeamCoach     │  │   FBTeamOffense      │  │   FBTeamDefense      │
├──────────────────┤  ├──────────────────────┤  ├──────────────────────┤
│ risk_taking: u8  │  │ passing: u8          │  │ blitzing: u8         │
│ run_pass: u8     │  │ blocking: u8         │  │ rush_defense: u8     │
│ up_tempo: u8     │  │ rushing: u8          │  │ pass_defense: u8     │
│ (each 0-100)     │  │ receiving: u8        │  │ coverage: u8         │
└──────────────────┘  │ scrambling: u8       │  │ turnovers: u8        │
                      │ turnovers: u8        │  │ kick_returning: u8   │
                      │ field_goals: u8      │  │ (each 0-100)         │
                      │ punting: u8          │  └──────────────────────┘
                      │ kickoffs: u8         │
                      │ kick_return_defense: │
                      │               u8     │
                      │ (each 0-100)         │
                      └──────────────────────┘

┌──────────────────────┐  ┌──────────────────────┐
│  LeagueTeamRecord    │  │  OffensiveStats      │
├──────────────────────┤  ├──────────────────────┤
│ wins: usize          │  │ passing: PassingStats│
│ losses: usize        │  │ rushing: RushingStats│
│ ties: usize          │  │ receiving:           │
│ Display: "W-L-T"     │  │   ReceivingStats     │
└──────────────────────┘  └──────────────────────┘
```

### Schema

```
league (League)
  teams (BTreeMap<usize, LeagueTeam>)
    i (LeagueTeam)
      (empty - identity only)
  current_season (Option<LeagueSeason>)
    year (usize)
    teams (BTreeMap<usize, FootballTeam>)
      i (FootballTeam)
        name (String, max 64 chars)
        short_name (String, max 4 chars)
        coach (FootballTeamCoach)
          risk_taking (u8, 0-100)
          run_pass (u8, 0-100)
          up_tempo (u8, 0-100)
        offense (FootballTeamOffense)
          passing (u8, 0-100)
          blocking (u8, 0-100)
          rushing (u8, 0-100)
          receiving (u8, 0-100)
          scrambling (u8, 0-100)
          turnovers (u8, 0-100)
          field_goals (u8, 0-100)
          punting (u8, 0-100)
          kickoffs (u8, 0-100)
          kick_return_defense (u8, 0-100)
        defense (FootballTeamDefense)
          blitzing (u8, 0-100)
          rush_defense (u8, 0-100)
          pass_defense (u8, 0-100)
          coverage (u8, 0-100)
          turnovers (u8, 0-100)
          kick_returning (u8, 0-100)
    conferences (Vec<LeagueConference>)
      i (LeagueConference)
        name (String, max 64 chars)
        divisions (Vec<LeagueDivision>)
          j (LeagueDivision)
            name (String, max 64 chars)
            teams (Vec<usize>)
    weeks (Vec<LeagueSeasonWeek>)
      i (LeagueSeasonWeek)
        matchups (Vec<LeagueSeasonMatchup>)
          j (LeagueSeasonMatchup)
            home_team (usize)
            away_team (usize)
            context (GameContext)
            game (Option<Game>)
            home_stats (Option<OffensiveStats>)
            away_stats (Option<OffensiveStats>)
    playoffs (LeagueSeasonPlayoffs)
      teams (PlayoffTeams)
        teams (BTreeMap<usize, BTreeMap<usize, PlayoffTeam>>)
          conference_id -> team_id -> PlayoffTeam
            seed (usize)
            short_name (String, max 4 chars)
      conference_brackets (BTreeMap<usize, Vec<LeagueSeasonWeek>>)
        conference_id -> rounds (Vec<LeagueSeasonWeek>)
      winners_bracket (Vec<LeagueSeasonWeek>)
  seasons (Vec<LeagueSeason>)
    (same structure as current_season)
```

#### Example

<details>

<summary>Example League serialized as JSON</summary>

```json
{
  "teams": {
    "0": {},
    "1": {},
    "2": {},
    "3": {}
  },
  "current_season": {
    "year": 2025,
    "teams": {
      "0": {
        "name": "New York Monsters",
        "short_name": "NYM",
        "coach": { "risk_taking": 50, "run_pass": 50, "up_tempo": 50 },
        "offense": {
          "passing": 50, "blocking": 50, "rushing": 50, "receiving": 50,
          "scrambling": 50, "turnovers": 50, "field_goals": 50, "punting": 50,
          "kickoffs": 50, "kick_return_defense": 50
        },
        "defense": {
          "blitzing": 50, "rush_defense": 50, "pass_defense": 50,
          "coverage": 50, "turnovers": 50, "kick_returning": 50
        }
      },
      "1": {
        "name": "Carolina Wombats",
        "short_name": "CAR",
        "coach": { "risk_taking": 50, "run_pass": 50, "up_tempo": 50 },
        "offense": {
          "passing": 50, "blocking": 50, "rushing": 50, "receiving": 50,
          "scrambling": 50, "turnovers": 50, "field_goals": 50, "punting": 50,
          "kickoffs": 50, "kick_return_defense": 50
        },
        "defense": {
          "blitzing": 50, "rush_defense": 50, "pass_defense": 50,
          "coverage": 50, "turnovers": 50, "kick_returning": 50
        }
      },
      "2": {
        "name": "New England Tulips",
        "short_name": "NET",
        "coach": { "risk_taking": 50, "run_pass": 50, "up_tempo": 50 },
        "offense": {
          "passing": 50, "blocking": 50, "rushing": 50, "receiving": 50,
          "scrambling": 50, "turnovers": 50, "field_goals": 50, "punting": 50,
          "kickoffs": 50, "kick_return_defense": 50
        },
        "defense": {
          "blitzing": 50, "rush_defense": 50, "pass_defense": 50,
          "coverage": 50, "turnovers": 50, "kick_returning": 50
        }
      },
      "3": {
        "name": "New Orleans Grouse",
        "short_name": "NOG",
        "coach": { "risk_taking": 50, "run_pass": 50, "up_tempo": 50 },
        "offense": {
          "passing": 50, "blocking": 50, "rushing": 50, "receiving": 50,
          "scrambling": 50, "turnovers": 50, "field_goals": 50, "punting": 50,
          "kickoffs": 50, "kick_return_defense": 50
        },
        "defense": {
          "blitzing": 50, "rush_defense": 50, "pass_defense": 50,
          "coverage": 50, "turnovers": 50, "kick_returning": 50
        }
      }
    },
    "conferences": [
      {
        "name": "American",
        "divisions": [
          { "name": "East", "teams": [0, 1] },
          { "name": "West", "teams": [2, 3] }
        ]
      }
    ],
    "weeks": [
      {
        "matchups": [
          {
            "home_team": 0,
            "away_team": 1,
            "context": { "home_score": 28, "away_score": 14, "game_over": true },
            "home_stats": { "passing": {}, "rushing": {}, "receiving": {} },
            "away_stats": { "passing": {}, "rushing": {}, "receiving": {} }
          },
          {
            "home_team": 2,
            "away_team": 3,
            "context": { "home_score": 21, "away_score": 17, "game_over": true },
            "home_stats": { "passing": {}, "rushing": {}, "receiving": {} },
            "away_stats": { "passing": {}, "rushing": {}, "receiving": {} }
          }
        ]
      }
    ],
    "playoffs": {
      "teams": { "teams": {} },
      "conference_brackets": {},
      "winners_bracket": []
    }
  },
  "seasons": []
}
```

</details>

## API Specification

The API will store leagues in a database tied to authenticated users.  Rather than accepting a full `League` JSON in every request, the API will persist league state server-side and expose RESTful endpoints that transform the stored league.  All endpoints are prefixed with `/v1/`.

### Leagues

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/leagues` | Create a new league for the authenticated user |
| `GET` | `/v1/leagues` | List the authenticated user's leagues |
| `GET` | `/v1/leagues/:id` | Get a league by ID |
| `DELETE` | `/v1/leagues/:id` | Delete a league by ID |

### League Teams

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/teams` | List all teams in the league |
| `POST` | `/v1/leagues/:id/teams` | Add a new team to the league |
| `GET` | `/v1/leagues/:id/teams/:team_id` | Get a team (including all-time record, stats, playoff record, championship history) |

### League Team Stats

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/teams/stats/passing` | All-time passing stats for all teams |
| `GET` | `/v1/leagues/:id/teams/stats/rushing` | All-time rushing stats for all teams |
| `GET` | `/v1/leagues/:id/teams/stats/receiving` | All-time receiving stats for all teams |

### League Seasons

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons` | List all seasons (past and current) |
| `POST` | `/v1/leagues/:id/seasons` | Create a new season |
| `GET` | `/v1/leagues/:id/seasons/:year` | Get a specific season |
| `POST` | `/v1/leagues/:id/seasons/sim` | Simulate the current season in its entirety |

### Season Conferences

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/conferences` | List conferences |
| `POST` | `/v1/leagues/:id/seasons/conferences` | Add a conference to the current season |
| `GET` | `/v1/leagues/:id/seasons/:year/conferences/:conf_id` | Get a conference |

### Season Conference Divisions

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/conferences/:conf_id/divisions` | List divisions |
| `POST` | `/v1/leagues/:id/seasons/conferences/:conf_id/divisions` | Add a division to a conference |
| `GET` | `/v1/leagues/:id/seasons/:year/conferences/:conf_id/divisions/:div_id` | Get a division |

### Season Teams

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/teams` | List season teams |
| `POST` | `/v1/leagues/:id/seasons/teams` | Add a team to the current season |
| `POST` | `/v1/leagues/:id/seasons/teams/:team_id/assign` | Assign a team to a conference/division |
| `GET` | `/v1/leagues/:id/seasons/:year/teams/:team_id` | Get a season team |

### Season Team Stats

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/teams/stats/passing` | Season passing stats |
| `GET` | `/v1/leagues/:id/seasons/:year/teams/stats/rushing` | Season rushing stats |
| `GET` | `/v1/leagues/:id/seasons/:year/teams/stats/receiving` | Season receiving stats |

### Season Standings

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/standings` | Overall standings (supports query params: `conference`, `division`, `by_conference`, `by_division`) |

### Season Schedule

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/leagues/:id/seasons/schedule` | Generate a schedule (accepts schedule options in body) |

### Season Weeks

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/weeks` | List weeks |
| `GET` | `/v1/leagues/:id/seasons/:year/weeks/:week_id` | Get a specific week |
| `POST` | `/v1/leagues/:id/seasons/weeks/:week_id/sim` | Simulate a week |

### Season Week Matchups

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/weeks/:week_id/matchups` | List matchups for a week |
| `GET` | `/v1/leagues/:id/seasons/:year/weeks/:week_id/matchups/:matchup_id` | Get a matchup |
| `POST` | `/v1/leagues/:id/seasons/weeks/:week_id/matchups/:matchup_id/sim` | Simulate a matchup |
| `POST` | `/v1/leagues/:id/seasons/weeks/:week_id/matchups/:matchup_id/play` | Simulate the next play of a matchup |

### Season Playoffs

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/leagues/:id/seasons/playoffs` | Generate playoffs (accepts playoff options in body) |
| `GET` | `/v1/leagues/:id/seasons/:year/playoffs` | Get the playoff bracket |
| `GET` | `/v1/leagues/:id/seasons/:year/playoffs/picture` | Get the playoff picture (supports query params: `num_teams`, `per_conference`, `division_winners`, `conference`) |
| `POST` | `/v1/leagues/:id/seasons/playoffs/sim` | Simulate all playoff rounds |

### Season Playoff Rounds

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/leagues/:id/seasons/:year/playoffs/rounds/:round_id` | Get a playoff round (supports query params: `conference`, `winners_bracket`) |
| `POST` | `/v1/leagues/:id/seasons/playoffs/rounds/sim` | Simulate the next playoff round |
| `GET` | `/v1/leagues/:id/seasons/:year/playoffs/rounds/:round_id/matchups/:matchup_id` | Get a playoff matchup (supports query params: `conference`, `winners_bracket`) |
| `POST` | `/v1/leagues/:id/seasons/playoffs/rounds/:round_id/matchups/:matchup_id/sim` | Simulate a playoff matchup (supports query params: `conference`, `winners_bracket`) |

## CLI Specification

The following subcommand hierarchy is implemented in the `fbsim` CLI for league management and simulation.

```
fbsim
└── league
    ├── create
    ├── team
    │   ├── add
    │   ├── get
    │   ├── list
    │   └── stats
    │       ├── passing
    │       ├── rushing
    │       └── receiving
    └── season
        ├── add
        ├── get
        ├── list
        ├── sim
        ├── standings
        ├── conference
        │   ├── add
        │   ├── get
        │   ├── list
        │   └── division
        │       ├── add
        │       ├── get
        │       └── list
        ├── team
        │   ├── add
        │   ├── assign
        │   ├── get
        │   ├── list
        │   └── stats
        │       ├── passing
        │       ├── rushing
        │       └── receiving
        ├── schedule
        │   └── gen
        ├── week
        │   ├── get
        │   ├── list
        │   ├── sim
        │   └── matchup
        │       ├── get
        │       ├── sim
        │       └── play
        │           └── sim
        └── playoffs
            ├── gen
            ├── get
            ├── picture
            ├── sim
            └── round
                ├── get
                ├── sim
                └── matchup
                    ├── get
                    └── sim
```

### League

```
Manage FootballSim leagues

Usage: fbsim league <COMMAND>

Commands:
  create   Create a new FootballSim league
  season   Manage seasons for an existing FootballSim league
  team     Manage teams for an existing FootballSim league

Options:
  -h, --help     Print help
```

#### `fbsim league create`

```
Create a new FootballSim league

Options:
  -f, --file <OUTPUT_FILE>   File path to write the new league to (required)
  -h, --help                 Print help
```

### League Team

```
Manage teams for an existing FootballSim league

Usage: fbsim league team <COMMAND>

Commands:
  add     Add a new team to the FootballSim league
  get     Display historical information about a team in the league
  list    List all teams in the league
  stats   View team statistics

Options:
  -h, --help     Print help
```

#### `fbsim league team add`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

#### `fbsim league team get`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -t, --team <TEAM>       The ID of the team to display (required)
  -h, --help              Print help
```

#### `fbsim league team list`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

### League Team Stats

```
Usage: fbsim league team stats <COMMAND>

Commands:
  passing     Get all-time passing stats for each team
  rushing     Get all-time rushing stats for each team
  receiving   Get all-time receiving stats for each team

Options:
  -h, --help   Print help
```

Each stats subcommand accepts:

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

### League Season

```
Manage seasons for an existing FootballSim league

Usage: fbsim league season <COMMAND>

Commands:
  add          Add a new season to the league
  get          Get a past or current season
  list         List all past and current seasons
  sim          Simulate the current season in its entirety
  standings    Display season standings
  conference   Manage conferences for a season
  team         Manage teams for a season
  schedule     Generate a schedule for the season
  week         Manage weeks for a season
  playoffs     Manage playoffs for a season

Options:
  -h, --help     Print help
```

#### `fbsim league season add`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

#### `fbsim league season get`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -h, --help              Print help
```

#### `fbsim league season list`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

#### `fbsim league season sim`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

### League Season Conference

```
Usage: fbsim league season conference <COMMAND>

Commands:
  add        Add a conference to the current season
  get        Get a specific conference
  list       List all conferences
  division   Manage divisions within a conference

Options:
  -h, --help   Print help
```

#### `fbsim league season conference add`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -n, --name <NAME>       Name of the conference (required)
  -h, --help              Print help
```

#### `fbsim league season conference get`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -c, --conference <CONFERENCE>   The conference index (required)
  -h, --help                      Print help
```

#### `fbsim league season conference list`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -h, --help              Print help
```

### League Season Conference Division

```
Usage: fbsim league season conference division <COMMAND>

Commands:
  add    Add a division to a conference
  get    Get a specific division
  list   List all divisions in a conference

Options:
  -h, --help   Print help
```

#### `fbsim league season conference division add`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -c, --conference <CONFERENCE>   The conference index (required)
  -n, --name <NAME>               Name of the division (required)
  -h, --help                      Print help
```

#### `fbsim league season conference division get`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -c, --conference <CONFERENCE>   The conference index (required)
  -d, --division <DIVISION>       The division index (required)
  -h, --help                      Print help
```

#### `fbsim league season conference division list`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -c, --conference <CONFERENCE>   The conference index (required)
  -h, --help                      Print help
```

### League Season Team

```
Usage: fbsim league season team <COMMAND>

Commands:
  add      Add a team to the current season
  assign   Assign a team to a conference/division
  get      Display a team from a season
  list     List all teams from a season
  stats    View season team statistics

Options:
  -h, --help   Print help
```

#### `fbsim league season team add`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -t, --team <TEAM>       Path to the team file (JSON) (required)
  -i, --id <ID>           The league-level team ID (required)
  -h, --help              Print help
```

#### `fbsim league season team assign`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -t, --team <TEAM>               The team ID to assign (required)
  -c, --conference <CONFERENCE>   The conference index (required)
  -d, --division <DIVISION>       The division index (required)
  -h, --help                      Print help
```

#### `fbsim league season team get`

```
Options:
  -l, --league <LEAGUE>                           Path to the league file (required)
  -y, --year <YEAR>                               The year of the season (required)
  -t, --team <TEAM>                               The team ID (required)
  -n, --num-playoff-teams <NUM_PLAYOFF_TEAMS>     Number of playoff teams for picture calculation (default: 4)
  -h, --help                                      Print help
```

#### `fbsim league season team list`

```
Options:
  -l, --league <LEAGUE>                           Path to the league file (required)
  -y, --year <YEAR>                               The year of the season (required)
  -n, --num-playoff-teams <NUM_PLAYOFF_TEAMS>     Number of playoff teams for picture calculation (default: 4)
  -h, --help                                      Print help
```

### League Season Team Stats

```
Usage: fbsim league season team stats <COMMAND>

Commands:
  passing     Get passing stats for each team in a season
  rushing     Get rushing stats for each team in a season
  receiving   Get receiving stats for each team in a season

Options:
  -h, --help   Print help
```

Each stats subcommand accepts:

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -h, --help              Print help
```

### League Season Standings

```
fbsim league season standings

Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -c, --conference <CONFERENCE>   Filter by conference index (optional)
  -d, --division <DIVISION>       Filter by division index (optional, requires --conference)
      --by-conference             Group standings by conference
      --by-division               Group standings by division
  -h, --help                      Print help
```

### League Season Schedule

```
Usage: fbsim league season schedule <COMMAND>

Commands:
  gen   Generate a schedule for the current season

Options:
  -h, --help   Print help
```

#### `fbsim league season schedule gen`

```
Options:
  -l, --league <LEAGUE>                                   Path to the league file (required)
  -w, --weeks <WEEKS>                                     Number of weeks (optional)
  -s, --seed <SEED>                                       Schedule generation seed (optional)
      --shift <SHIFT>                                     Shift weeks after generating (optional)
  -p, --permute <PERMUTE>                                 Permute the schedule (optional) [true/false]
      --division-games <DIVISION_GAMES>                   Games per division opponent (optional)
      --conference-games <CONFERENCE_GAMES>               Games per non-div conference opponent (optional)
      --cross-conference-games <CROSS_CONFERENCE_GAMES>   Total cross-conference games (optional)
  -h, --help                                              Print help
```

### League Season Week

```
Usage: fbsim league season week <COMMAND>

Commands:
  get       Display a week from a season
  list      List all weeks of a season
  sim       Simulate a week in its entirety
  matchup   Manage matchups for a week

Options:
  -h, --help   Print help
```

#### `fbsim league season week get`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -w, --week <WEEK>       The week index (required)
  -h, --help              Print help
```

#### `fbsim league season week list`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -h, --help              Print help
```

#### `fbsim league season week sim`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -w, --week <WEEK>       The week index (required)
  -h, --help              Print help
```

### League Season Week Matchup

```
Usage: fbsim league season week matchup <COMMAND>

Commands:
  get    Display a matchup
  sim    Simulate a matchup
  play   Manage plays for a matchup

Options:
  -h, --help   Print help
```

#### `fbsim league season week matchup get`

```
Options:
  -l, --league <LEAGUE>     Path to the league file (required)
  -y, --year <YEAR>         The year of the season (required)
  -w, --week <WEEK>         The week index (required)
  -m, --matchup <MATCHUP>   The matchup index (required)
  -h, --help                Print help
```

#### `fbsim league season week matchup sim`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -w, --week <WEEK>               The week index (required)
  -m, --matchup <MATCHUP>         The matchup index (required)
  -s, --speed <PLAYBACK_SPEED>    Playback speed (optional)
  -h, --help                      Print help
```

### League Season Week Matchup Play

```
Usage: fbsim league season week matchup play <COMMAND>

Commands:
  sim   Simulate the next play in a matchup

Options:
  -h, --help   Print help
```

#### `fbsim league season week matchup play sim`

```
Options:
  -l, --league <LEAGUE>     Path to the league file (required)
  -w, --week <WEEK>         The week index (required)
  -m, --matchup <MATCHUP>   The matchup index (required)
  -h, --help                Print help
```

### League Season Playoffs

```
Usage: fbsim league season playoffs <COMMAND>

Commands:
  gen       Generate playoffs for the current season
  get       Display the playoff bracket
  picture   Display the playoff picture
  sim       Simulate all playoff rounds
  round     Manage individual playoff rounds

Options:
  -h, --help   Print help
```

#### `fbsim league season playoffs gen`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -n, --num-teams <NUM_TEAMS>     Number of playoff teams (required)
  -p, --per-conference            Use per-conference bracket mode
  -d, --division-winners          Guarantee division winners a playoff berth
  -h, --help                      Print help
```

#### `fbsim league season playoffs get`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -y, --year <YEAR>       The year of the season (required)
  -h, --help              Print help
```

#### `fbsim league season playoffs picture`

```
Options:
  -l, --league <LEAGUE>                           Path to the league file (required)
  -y, --year <YEAR>                               The year of the season (required)
  -n, --num-playoff-teams <NUM_PLAYOFF_TEAMS>     Number of playoff teams (default: 4)
  -p, --per-conference                            Calculate per-conference
  -d, --division-winners                          Account for guaranteed division winners
  -c, --conference <CONFERENCE>                   Show only this conference (optional)
  -h, --help                                      Print help
```

#### `fbsim league season playoffs sim`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

### League Season Playoffs Round

```
Usage: fbsim league season playoffs round <COMMAND>

Commands:
  get       Display a playoff round
  sim       Simulate the next playoff round
  matchup   Manage playoff matchups

Options:
  -h, --help   Print help
```

#### `fbsim league season playoffs round get`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -r, --round <ROUND>             The round index (required)
  -c, --conference <CONFERENCE>   Conference bracket index (optional)
  -w, --winners-bracket           Get from winners bracket
  -h, --help                      Print help
```

#### `fbsim league season playoffs round sim`

```
Options:
  -l, --league <LEAGUE>   Path to the league file (required)
  -h, --help              Print help
```

### League Season Playoffs Round Matchup

```
Usage: fbsim league season playoffs round matchup <COMMAND>

Commands:
  get   Display a playoff matchup
  sim   Simulate a playoff matchup

Options:
  -h, --help   Print help
```

#### `fbsim league season playoffs round matchup get`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -y, --year <YEAR>               The year of the season (required)
  -r, --round <ROUND>             The round index (required)
  -m, --matchup <MATCHUP>         The matchup index (required)
  -c, --conference <CONFERENCE>   Conference bracket index (optional)
  -w, --winners-bracket           Get from winners bracket
  -h, --help                      Print help
```

#### `fbsim league season playoffs round matchup sim`

```
Options:
  -l, --league <LEAGUE>           Path to the league file (required)
  -r, --round <ROUND>             The round index (required)
  -m, --matchup <MATCHUP>         The matchup index (required)
  -s, --speed <PLAYBACK_SPEED>    Playback speed (optional)
  -c, --conference <CONFERENCE>   Conference bracket index (default: 0)
  -w, --winners-bracket           Simulate from winners bracket
  -h, --help                      Print help
```

## Roadmap

At a high-level, the roadmap is:

1. **CLI development** (largely complete): Iteratively develop league management and simulation subcommands in the `fbsim` CLI, building reusable functionality in `fbsim-core`
2. **UI development**: Build corresponding pages and web components in the `fbsim` UI that use a WASM module based on `fbsim-core` to interact with `League` types locally in the browser

## Next Steps

- **Authentication**: The API and UI are authenticated and user-based; leagues are stored per-user in a database
- **Database layer**: The API has an ORM layer atop a relational database for persisting league state
- **PWA Experience**: The WASM module is used as a PWA layer when the network is unavailable, while the UI supports both local and remote (authenticated) leagues
