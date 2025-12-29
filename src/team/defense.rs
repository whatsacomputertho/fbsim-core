#![doc = include_str!("../../docs/team/defense.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};

/// # `FootballTeamDefenseRaw` struct
///
/// A `FootballTeamDefenseRaw` is a `FootballTeamDefense` before its properties
/// have been validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamDefenseRaw {
    blitzing: u32,
    rush_defense: u32,
    pass_defense: u32,
    coverage: u32,
    turnovers: u32,
    kick_returning: u32
}

impl FootballTeamDefenseRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure each property is no greater than 100
        if self.blitzing > 100 {
            return Err(
                format!(
                    "Blitzing attribute is out of range [0, 100]: {}",
                    self.blitzing
                )
            )
        }
        if self.rush_defense > 100 {
            return Err(
                format!(
                    "Rush defense attribute is out of range [0, 100]: {}",
                    self.rush_defense
                )
            )
        }
        if self.pass_defense > 100 {
            return Err(
                format!(
                    "Pass defense attribute is out of range [0, 100]: {}",
                    self.pass_defense
                )
            )
        }
        if self.coverage > 100 {
            return Err(
                format!(
                    "Coverage attribute is out of range [0, 100]: {}",
                    self.coverage
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
        if self.kick_returning > 100 {
            return Err(
                format!(
                    "Kick returning attribute is out of range [0, 100]: {}",
                    self.kick_returning
                )
            )
        }
        Ok(())
    }
}

/// # `FootballTeamDefense` struct
///
/// A `FootballTeamDefense` represents a football team defense
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct FootballTeamDefense {
    blitzing: u32,
    rush_defense: u32,
    pass_defense: u32,
    coverage: u32,
    turnovers: u32,
    kick_returning: u32
}

impl TryFrom<FootballTeamDefenseRaw> for FootballTeamDefense {
    type Error = String;

    fn try_from(item: FootballTeamDefenseRaw) -> Result<Self, Self::Error> {
        // Validate the raw coach
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            FootballTeamDefense{
                blitzing: item.blitzing,
                rush_defense: item.rush_defense,
                pass_defense: item.pass_defense,
                coverage: item.coverage,
                turnovers: item.turnovers,
                kick_returning: item.kick_returning
            }
        )
    }
}

impl<'de> Deserialize<'de> for FootballTeamDefense {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = FootballTeamDefenseRaw::deserialize(deserializer)?;
        FootballTeamDefense::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for FootballTeamDefense {
    /// Default constructor for the FootballTeamDefense class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::default();
    /// ```
    fn default() -> Self {
        FootballTeamDefense{
            blitzing: 50_u32,
            rush_defense: 50_u32,
            pass_defense: 50_u32,
            coverage: 50_u32,
            turnovers: 50_u32,
            kick_returning: 50_u32
        }
    }
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
        FootballTeamDefense::default()
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
        let raw = FootballTeamDefenseRaw{
            blitzing: overall,
            rush_defense: overall,
            pass_defense: overall,
            coverage: overall,
            turnovers: overall,
            kick_returning: overall
        };
        FootballTeamDefense::try_from(raw)
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
        (
            (
                self.blitzing + self.rush_defense + self.pass_defense +
                self.coverage + self.turnovers + self.kick_returning
            ) as f32 / 6_f32
        ) as u32
    }

    /// Get the defense's rush defense skill level
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

    /// Get the defense's pass defense skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let pass_defense = my_defense.pass_defense();
    /// assert!(pass_defense == 50_u32);
    /// ```
    pub fn pass_defense(&self) -> u32 {
        self.pass_defense
    }

    /// Get the defense's coverage skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let coverage = my_defense.coverage();
    /// assert!(coverage == 50_u32);
    /// ```
    pub fn coverage(&self) -> u32 {
        self.coverage
    }

    /// Get the defense's blitzing skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let blitzing = my_defense.blitzing();
    /// assert!(blitzing == 50_u32);
    /// ```
    pub fn blitzing(&self) -> u32 {
        self.blitzing
    }

    /// Get the defense's turnovers skill level
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

    /// Get the defense's kick returning skill level
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefense;
    /// 
    /// let my_defense = FootballTeamDefense::new();
    /// let kick_returning = my_defense.kick_returning();
    /// assert!(kick_returning == 50_u32);
    /// ```
    pub fn kick_returning(&self) -> u32 {
        self.kick_returning
    }
}

/// # `FootballTeamDefenseBuilder` struct
///
/// A `FootballTeamDefenseBuilder` implements the builder pattern for the
/// `FootballTeamDefense` struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct FootballTeamDefenseBuilder {
    blitzing: u32,
    rush_defense: u32,
    pass_defense: u32,
    coverage: u32,
    turnovers: u32,
    kick_returning: u32
}

impl Default for FootballTeamDefenseBuilder {
    /// Default constructor for the FootballTeamDefenseBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefenseBuilder;
    /// 
    /// let my_defense_builder = FootballTeamDefenseBuilder::default();
    /// ```
    fn default() -> Self {
        FootballTeamDefenseBuilder{
            blitzing: 50_u32,
            rush_defense: 50_u32,
            pass_defense: 50_u32,
            coverage: 50_u32,
            turnovers: 50_u32,
            kick_returning: 50_u32
        }
    }
}

impl FootballTeamDefenseBuilder {
    /// Initialize a new defense builder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::FootballTeamDefenseBuilder;
    ///
    /// let mut my_defense_builder = FootballTeamDefenseBuilder::new();
    /// ```
    pub fn new() -> FootballTeamDefenseBuilder {
        FootballTeamDefenseBuilder::default()
    }

    /// Set the blitzing property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .blitzing(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.blitzing() == 60);
    /// ```
    pub fn blitzing(mut self, blitzing: u32) -> Self {
        self.blitzing = blitzing;
        self
    }

    /// Set the rush defense property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .rush_defense(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.rush_defense() == 60);
    /// ```
    pub fn rush_defense(mut self, rush_defense: u32) -> Self {
        self.rush_defense = rush_defense;
        self
    }

    /// Set the pass defense property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .pass_defense(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.pass_defense() == 60);
    /// ```
    pub fn pass_defense(mut self, pass_defense: u32) -> Self {
        self.pass_defense = pass_defense;
        self
    }

    /// Set the coverage property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .coverage(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.coverage() == 60);
    /// ```
    pub fn coverage(mut self, coverage: u32) -> Self {
        self.coverage = coverage;
        self
    }

    /// Set the turnovers property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .turnovers(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.turnovers() == 60);
    /// ```
    pub fn turnovers(mut self, turnovers: u32) -> Self {
        self.turnovers = turnovers;
        self
    }

    /// Set the kick returning property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .kick_returning(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_defense.kick_returning() == 60);
    /// ```
    pub fn kick_returning(mut self, kick_returning: u32) -> Self {
        self.kick_returning = kick_returning;
        self
    }

    /// Build the defense
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::defense::{FootballTeamDefense, FootballTeamDefenseBuilder};
    /// 
    /// let my_defense = FootballTeamDefenseBuilder::new()
    ///     .blitzing(40)
    ///     .rush_defense(45)
    ///     .pass_defense(50)
    ///     .coverage(55)
    ///     .turnovers(60)
    ///     .kick_returning(65)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<FootballTeamDefense, String> {
        let raw = FootballTeamDefenseRaw{
            blitzing: self.blitzing,
            rush_defense: self.rush_defense,
            pass_defense: self.pass_defense,
            coverage: self.coverage,
            turnovers: self.turnovers,
            kick_returning: self.kick_returning
        };
        FootballTeamDefense::try_from(raw)
    }
}
