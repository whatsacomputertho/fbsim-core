#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// # `FootballTeamDefense` struct
///
/// A `FootballTeamDefense` represents a football team defense
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamDefense {
    blitzing: u32,
    rush_defense: u32,
    pass_defense: u32,
    coverage: u32,
    turnovers: u32,
    kick_returning: u32
}

impl FootballTeamDefense {
    /// Constructor for the `FootballTeamDefense` struct in which each
    /// skill level is defaulted to 50
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    ///
    /// let my_defense = FootballTeamDefense::new();
    /// ```
    pub fn new() -> FootballTeamDefense {
        FootballTeamDefense{
            blitzing: 50_u32,
            rush_defense: 50_u32,
            pass_defense: 50_u32,
            coverage: 50_u32,
            turnovers: 50_u32,
            kick_returning: 50_u32
        }
    }

    /// Constructor for the `FootballTeamDefense` struct in which each
    /// skill level is set to the provided overall
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::from_overall(20).unwrap();
    /// assert!(my_defense.overall() == 20_u32);
    /// ```
    pub fn from_overall(overall: u32) -> Result<FootballTeamDefense, String> {
        if overall > 100_u32 {
            return Err(format!("Overall not in range [0, 100]: {}", overall))
        }
        Ok(
            FootballTeamDefense{
                blitzing: overall,
                rush_defense: overall,
                pass_defense: overall,
                coverage: overall,
                turnovers: overall,
                kick_returning: overall
            }
        )
    }

    /// Constructor for the `FootballTeamDefense` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    ///
    /// let my_defense = FootballTeamDefense::from_properties(0, 20, 40, 60, 80, 100);
    /// ```
    pub fn from_properties(blitzing: u32, rush_defense: u32, pass_defense: u32, coverage: u32, turnovers: u32, kick_returning: u32) -> Result<FootballTeamDefense, String> {
        // Ensure each skill level is in range
        if blitzing > 100_u32 {
            return Err(format!("Blitzing not in range [0, 100]: {}", blitzing))
        }
        if rush_defense > 100_u32 {
            return Err(format!("Rush defense not in range [0, 100]: {}", rush_defense))
        }
        if pass_defense > 100_u32 {
            return Err(format!("Pass defense not in range [0, 100]: {}", pass_defense))
        }
        if coverage > 100_u32 {
            return Err(format!("Coverage not in range [0, 100]: {}", coverage))
        }
        if turnovers > 100_u32 {
            return Err(format!("Turnovers not in range [0, 100]: {}",turnovers))
        }
        if kick_returning > 100_u32 {
            return Err(format!("Kick returning not in range [0, 100]: {}", kick_returning))
        }
        Ok(
            FootballTeamDefense{
                blitzing: blitzing,
                rush_defense: rush_defense,
                pass_defense: pass_defense,
                coverage: coverage,
                turnovers: turnovers,
                kick_returning: kick_returning
            }
        )
    }

    /// Calculate the defense's overall rating
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    ///
    /// let my_defense = FootballTeamDefense::new();
    /// let overall = my_defense.overall();
    /// assert!(overall == 50_u32);
    /// ```
    pub fn overall(&self) -> u32 {
        return (
            (
                self.blitzing + self.rush_defense + self.pass_defense +
                self.coverage + self.turnovers + self.kick_returning
            ) as f32 / 6_f32
        ) as u32;
    }

    /// Borrow the defense's rush defense skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let rush_defense = my_defense.rush_defense();
    /// assert!(rush_defense == 50_u32);
    /// ```
    pub fn rush_defense(&self) -> u32 {
        self.rush_defense
    }

    /// Borrow the offense's turnovers skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let turnovers = my_defense.turnovers();
    /// assert!(turnovers == 50_u32);
    /// ```
    pub fn turnovers(&self) -> u32 {
        self.turnovers
    }
}
