# Picture module

The `picture` module defines the `PlayoffPicture`, `PlayoffPictureEntry`, `PlayoffPictureOptions`, and `PlayoffStatus` types which provide a real-time view of playoff qualification status during an ongoing season.

## PlayoffStatus enum

`PlayoffStatus` represents a team's playoff qualification status with the following variants
- `ClinchedTopSeed`: Team has clinched the #1 seed
- `ClinchedPlayoffs { current_seed }`: Team has clinched a playoff berth
- `InPlayoffPosition { current_seed }`: Team is in playoff position but hasn't clinched
- `InTheHunt`: Team is not in playoff position but still mathematically alive
- `Eliminated`: Team cannot make playoffs regardless of remaining outcomes

## PlayoffPictureEntry struct

A `PlayoffPictureEntry` contains the following properties
- `team_id`: The team's league-level ID
- `team_name`: The team's display name
- `current_record`: The team's current win-loss-tie record (a `LeagueTeamRecord`)
- `status`: The team's playoff qualification status (a `PlayoffStatus`)
- `games_back`: How many games behind the playoff cutoff the team is
- `remaining_games`: How many games the team has left to play
- `magic_number`: The number of wins needed to clinch a playoff spot, if applicable

## PlayoffPicture struct

A `PlayoffPicture` is constructed from a `LeagueSeason` via `PlayoffPicture::from_season()`. It supports both flat (non-conference) and conference-based playoff pictures, auto-detecting the mode based on the season's conference structure. It contains the following properties
- `num_playoff_teams`: The total number of playoff spots
- `entries`: The playoff picture entries for all teams (a `Vec<PlayoffPictureEntry>`)
- `games_remaining_in_season`: The total number of unplayed games in the season
