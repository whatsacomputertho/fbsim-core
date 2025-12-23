#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};

/// # `FootballTeamOffenseRaw` struct
///
/// A `FootballTeamOffenseRaw` is a `FootballTeamOffense` before its properties
/// have been validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamOffenseRaw {
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

impl FootballTeamOffenseRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure each property is no greater than 100
        if self.passing > 100 {
            return Err(
                format!(
                    "Passing attribute is out of range [0, 100]: {}",
                    self.passing
                )
            )
        }
        if self.blocking > 100 {
            return Err(
                format!(
                    "Blocking attribute is out of range [0, 100]: {}",
                    self.blocking
                )
            )
        }
        if self.rushing > 100 {
            return Err(
                format!(
                    "Rushing attribute is out of range [0, 100]: {}",
                    self.rushing
                )
            )
        }
        if self.receiving > 100 {
            return Err(
                format!(
                    "Receiving attribute is out of range [0, 100]: {}",
                    self.receiving
                )
            )
        }
        if self.scrambling > 100 {
            return Err(
                format!(
                    "Scrambling attribute is out of range [0, 100]: {}",
                    self.scrambling
                )
            )
        }
        if self.turnovers > 100 {
            return Err(
                format!(
                    "Turnovers attribute is out of range [0, 100]: {}",
                    self.turnovers
                )
            )
        }
        if self.field_goals > 100 {
            return Err(
                format!(
                    "Field goals attribute is out of range [0, 100]: {}",
                    self.field_goals
                )
            )
        }
        if self.punting > 100 {
            return Err(
                format!(
                    "Punting attribute is out of range [0, 100]: {}",
                    self.punting
                )
            )
        }
        if self.kickoffs > 100 {
            return Err(
                format!(
                    "Kickoffs attribute is out of range [0, 100]: {}",
                    self.kickoffs
                )
            )
        }
        if self.kick_return_defense > 100 {
            return Err(
                format!(
                    "Kick return defense attribute is out of range [0, 100]: {}",
                    self.kick_return_defense
                )
            )
        }
        Ok(())
    }
}

/// # `FootballTeamOffense` struct
///
/// A `FootballTeamOffense` represents a football team offense
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
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

impl TryFrom<FootballTeamOffenseRaw> for FootballTeamOffense {
    type Error = String;

    fn try_from(item: FootballTeamOffenseRaw) -> Result<Self, Self::Error> {
        // Validate the raw coach
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            FootballTeamOffense{
                passing: item.passing,
                blocking: item.blocking,
                rushing: item.rushing,
                receiving: item.receiving,
                scrambling: item.scrambling,
                turnovers: item.turnovers,
                field_goals: item.field_goals,
                punting: item.punting,
                kickoffs: item.kickoffs,
                kick_return_defense: item.kick_return_defense
            }
        )
    }
}

impl<'de> Deserialize<'de> for FootballTeamOffense {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = FootballTeamOffenseRaw::deserialize(deserializer)?;
        FootballTeamOffense::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for FootballTeamOffense {
    /// Default constructor for the FootballTeamOffense class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::default();
    /// ```
    fn default() -> Self {
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
        FootballTeamOffense::default()
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
        let raw = FootballTeamOffenseRaw{
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
        };
        FootballTeamOffense::try_from(raw)
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

    /// Get the offense's field goal kicking skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffense;
    /// 
    /// let my_offense = FootballTeamOffense::new();
    /// let field_goals = my_offense.field_goals();
    /// assert!(field_goals == 50_u32);
    /// ```
    pub fn field_goals(&self) -> u32 {
        self.field_goals
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

/// # `FootballTeamOffenseBuilder` struct
///
/// A `FootballTeamOffenseBuilder` implements the builder pattern for the
/// `FootballTeamOffense` struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct FootballTeamOffenseBuilder {
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

impl Default for FootballTeamOffenseBuilder {
    /// Default constructor for the FootballTeamOffenseBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffenseBuilder;
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::default();
    /// ```
    fn default() -> Self {
        FootballTeamOffenseBuilder{
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
}

impl FootballTeamOffenseBuilder {
    /// Initialize a new offense builder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::FootballTeamOffenseBuilder;
    ///
    /// let mut my_offense_builder = FootballTeamOffenseBuilder::new();
    /// ```
    pub fn new() -> FootballTeamOffenseBuilder {
        FootballTeamOffenseBuilder::default()
    }

    /// Set the passing property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .passing(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.passing() == 60);
    /// ```
    pub fn passing(mut self, passing: u32) -> Self {
        self.passing = passing;
        self
    }

    /// Set the blocking property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .blocking(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.blocking() == 60);
    /// ```
    pub fn blocking(mut self, blocking: u32) -> Self {
        self.blocking = blocking;
        self
    }

    /// Set the rushing property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .rushing(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.rushing() == 60);
    /// ```
    pub fn rushing(mut self, rushing: u32) -> Self {
        self.rushing = rushing;
        self
    }

    /// Set the receiving property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .receiving(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.receiving() == 60);
    /// ```
    pub fn receiving(mut self, receiving: u32) -> Self {
        self.receiving = receiving;
        self
    }

    /// Set the scrambling property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .scrambling(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.scrambling() == 60);
    /// ```
    pub fn scrambling(mut self, scrambling: u32) -> Self {
        self.scrambling = scrambling;
        self
    }

    /// Set the turnovers property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .turnovers(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.turnovers() == 60);
    /// ```
    pub fn turnovers(mut self, turnovers: u32) -> Self {
        self.turnovers = turnovers;
        self
    }

    /// Set the field_goals property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .field_goals(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.field_goals() == 60);
    /// ```
    pub fn field_goals(mut self, field_goals: u32) -> Self {
        self.field_goals = field_goals;
        self
    }

    /// Set the punting property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .punting(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.punting() == 60);
    /// ```
    pub fn punting(mut self, punting: u32) -> Self {
        self.punting = punting;
        self
    }

    /// Set the kickoffs property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .kickoffs(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.kickoffs() == 60);
    /// ```
    pub fn kickoffs(mut self, kickoffs: u32) -> Self {
        self.kickoffs = kickoffs;
        self
    }

    /// Set the kick_return_defense property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .kick_return_defense(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_offense.kick_return_defense() == 60);
    /// ```
    pub fn kick_return_defense(mut self, kick_return_defense: u32) -> Self {
        self.kick_return_defense = kick_return_defense;
        self
    }

    /// Build the offense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::offense::{FootballTeamOffense, FootballTeamOffenseBuilder};
    /// 
    /// let my_offense = FootballTeamOffenseBuilder::new()
    ///     .passing(25)
    ///     .blocking(30)
    ///     .rushing(35)
    ///     .receiving(40)
    ///     .scrambling(45)
    ///     .turnovers(50)
    ///     .field_goals(55)
    ///     .punting(60)
    ///     .kickoffs(65)
    ///     .kick_return_defense(70)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<FootballTeamOffense, String> {
        let raw = FootballTeamOffenseRaw{
            passing: self.passing,
            blocking: self.blocking,
            rushing: self.rushing,
            receiving: self.receiving,
            scrambling: self.scrambling,
            turnovers: self.turnovers,
            field_goals: self.field_goals,
            punting: self.punting,
            kickoffs: self.kickoffs,
            kick_return_defense: self.kick_return_defense
        };
        FootballTeamOffense::try_from(raw)
    }
}
