# League module

The `league` module defines the `League` struct. A `League` represents a football league spanning over the course of many seasons. There is also a `LeagueRaw` struct used for validating league properties before converting from `LeagueRaw -> League` via its `TryFrom` trait implementation.
