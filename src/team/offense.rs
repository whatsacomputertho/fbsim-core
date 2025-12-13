#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// # `FootballTeamOffense` struct
///
/// A `FootballTeamOffense` represents a football team offense
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamOffense {
    passing: u32,
    blocking: u32,
    rushing: u32,
    receiving: u32,
    scrambling: u32,
    turnovers: u32,
    field_goals: u32,
    punting: u32,
    kickoffs: u32,
    kick_return_defense: u32
}

impl FootballTeamOffense {
    /// Constructor for the `FootballTeamOffense` struct in which each
    /// skill level is defaulted to 50
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    ///
    /// let my_offense = FootballTeamOffense::new();
    /// ```
    pub fn new() -> FootballTeamOffense {
        FootballTeamOffense{
            passing: 50_u32,
            blocking: 50_u32,
            rushing: 50_u32,
            receiving: 50_u32,
            scrambling: 50_u32,
            turnovers: 50_u32,
            field_goals: 50_u32,
            punting: 50_u32,
            kickoffs: 50_u32,
            kick_return_defense: 50_u32
        }
    }

    /// Constructor for the `FootballTeamOffense` struct in which each
    /// skill level is set to the provided overall
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::from_overall(20).unwrap();
    /// assert!(my_offense.overall() == 20_u32);
    /// ```
    pub fn from_overall(overall: u32) -> Result<FootballTeamOffense, String> {
        if overall > 100_u32 {
            return Err(format!("Overall not in range [0, 100]: {}", overall))
        }
        Ok(
            FootballTeamOffense{
                passing: overall,
                blocking: overall,
                rushing: overall,
                receiving: overall,
                scrambling: overall,
                turnovers: overall,
                field_goals: overall,
                punting: overall,
                kickoffs: overall,
                kick_return_defense: overall
            }
        )
    }

    /// Constructor for the `FootballTeamOffense` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    ///
    /// let my_offense = FootballTeamOffense::from_properties(10, 20, 30, 40, 50, 60, 70, 80, 90, 100);
    /// ```
    pub fn from_properties(passing: u32, blocking: u32, rushing: u32, receiving: u32, scrambling: u32, turnovers: u32, field_goals: u32, punting: u32, kickoffs: u32, kick_return_defense: u32) -> Result<FootballTeamOffense, String> {
        // Ensure each skill level is in range
        if passing > 100_u32 {
            return Err(format!("Passing not in range [0, 100]: {}", passing))
        }
        if blocking > 100_u32 {
            return Err(format!("Blocking not in range [0, 100]: {}", blocking))
        }
        if rushing > 100_u32 {
            return Err(format!("Rushing not in range [0, 100]: {}", rushing))
        }
        if receiving > 100_u32 {
            return Err(format!("Receiving not in range [0, 100]: {}", receiving))
        }
        if scrambling > 100_u32 {
            return Err(format!("Scrambling not in range [0, 100]: {}", scrambling))
        }
        if turnovers > 100_u32 {
            return Err(format!("Turnovers not in range [0, 100]: {}",turnovers))
        }
        if field_goals > 100_u32 {
            return Err(format!("Field goals not in range [0, 100]: {}", field_goals))
        }
        if punting > 100_u32 {
            return Err(format!("Punting not in range [0, 100]: {}", punting))
        }
        if kickoffs > 100_u32 {
            return Err(format!("Kickoffs not in range [0, 100]: {}", kickoffs))
        }
        if kick_return_defense > 100_u32 {
            return Err(format!("Kick return defense not in range [0, 100]: {}", kick_return_defense))
        }
        Ok(
            FootballTeamOffense{
                passing: passing,
                blocking: blocking,
                rushing: rushing,
                receiving: receiving,
                scrambling: scrambling,
                turnovers: turnovers,
                field_goals: field_goals,
                punting: punting,
                kickoffs: kickoffs,
                kick_return_defense: kick_return_defense
            }
        )
    }

    /// Calculate the offense's overall rating
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    ///
    /// let my_offense = FootballTeamOffense::new();
    /// let overall = my_offense.overall();
    /// assert!(overall == 50_u32);
    /// ```
    pub fn overall(&self) -> u32 {
        return (
            (
                self.passing + self.blocking + self.rushing +
                self.receiving + self.scrambling + self.turnovers +
                self.field_goals + self.punting + self.kickoffs +
                self.kick_return_defense
            ) as f32 / 10_f32
        ) as u32;
    }

    /// Get the offense's rushing skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let rushing = my_offense.rushing();
    /// assert!(rushing == 50_u32);
    /// ```
    pub fn rushing(&self) -> u32 {
        self.rushing
    }

    /// Get the offense's passing skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let passing = my_offense.passing();
    /// assert!(passing == 50_u32);
    /// ```
    pub fn passing(&self) -> u32 {
        self.passing
    }

    /// Get the offense's receiving skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let receiving = my_offense.receiving();
    /// assert!(receiving == 50_u32);
    /// ```
    pub fn receiving(&self) -> u32 {
        self.receiving
    }

    /// Get the offense's scrambling skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let scrambling = my_offense.scrambling();
    /// assert!(scrambling == 50_u32);
    /// ```
    pub fn scrambling(&self) -> u32 {
        self.scrambling
    }

    /// Get the offense's blocking skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let blocking = my_offense.blocking();
    /// assert!(blocking == 50_u32);
    /// ```
    pub fn blocking(&self) -> u32 {
        self.blocking
    }

    /// Get the offense's turnovers skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let turnovers = my_offense.turnovers();
    /// assert!(turnovers == 50_u32);
    /// ```
    pub fn turnovers(&self) -> u32 {
        self.turnovers
    }

    /// Get the offense's punting skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let punting = my_offense.punting();
    /// assert!(punting == 50_u32);
    /// ```
    pub fn punting(&self) -> u32 {
        self.punting
    }

    /// Get the offense's kickoffs skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let kickoffs = my_offense.kickoffs();
    /// assert!(kickoffs == 50_u32);
    /// ```
    pub fn kickoffs(&self) -> u32 {
        self.kickoffs
    }

    /// Get the offense's kick return defense skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let kick_return_defense = my_offense.kick_return_defense();
    /// assert!(kick_return_defense == 50_u32);
    /// ```
    pub fn kick_return_defense(&self) -> u32 {
        self.kick_return_defense
    }
}
