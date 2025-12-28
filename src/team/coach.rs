#![doc = include_str!("../../docs/team/coach.md")]
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize, Deserializer};

/// # `FootballTeamCoachRaw` struct
///
/// A `FootballTeamCoachRaw` is a `FootballTeamCoach` before its properties
/// have been validated
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamCoachRaw {
    risk_taking: u32,
    run_pass: u32,
    up_tempo: u32
}

impl FootballTeamCoachRaw {
    pub fn validate(&self) -> Result<(), String> {
        // Ensure each property is no greater than 100
        if self.risk_taking > 100 {
            return Err(
                format!(
                    "Risk taking attribute is out of range [0, 100]: {}",
                    self.risk_taking
                )
            )
        }
        if self.run_pass > 100 {
            return Err(
                format!(
                    "Run-pass attribute is out of range [0, 100]: {}",
                    self.run_pass
                )
            )
        }
        if self.up_tempo > 100 {
            return Err(
                format!(
                    "Up-tempo attribute is out of range [0, 100]: {}",
                    self.up_tempo
                )
            )
        }
        Ok(())
    }
}

/// # `FootballTeamCoach` struct
///
/// A `FootballTeamCoach` represents a football team coach
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct FootballTeamCoach {
    risk_taking: u32,
    run_pass: u32,
    up_tempo: u32
}

impl TryFrom<FootballTeamCoachRaw> for FootballTeamCoach {
    type Error = String;

    fn try_from(item: FootballTeamCoachRaw) -> Result<Self, Self::Error> {
        // Validate the raw coach
        match item.validate() {
            Ok(()) => (),
            Err(error) => return Err(error),
        };

        // If valid, then convert
        Ok(
            FootballTeamCoach{
                risk_taking: item.risk_taking,
                run_pass: item.run_pass,
                up_tempo: item.up_tempo
            }
        )
    }
}

impl<'de> Deserialize<'de> for FootballTeamCoach {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Only deserialize if the conversion from raw succeeds
        let raw = FootballTeamCoachRaw::deserialize(deserializer)?;
        FootballTeamCoach::try_from(raw).map_err(serde::de::Error::custom)
    }
}

impl Default for FootballTeamCoach {
    /// Default constructor for the FootballTeamCoach class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    /// 
    /// let my_coach_builder = FootballTeamCoach::default();
    /// ```
    fn default() -> Self {
        FootballTeamCoach{
            risk_taking: 50_u32,
            run_pass: 50_u32,
            up_tempo: 50_u32
        }
    }
}

impl FootballTeamCoach {
    /// Constructor for the `FootballTeamCoach` struct in which each
    /// skill level is defaulted to 50
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    ///
    /// let my_coach = FootballTeamCoach::new();
    /// ```
    pub fn new() -> FootballTeamCoach {
        FootballTeamCoach::default()
    }

    /// Get the coach's risk taking tendency
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    /// 
    /// let my_coach = FootballTeamCoach::new();
    /// let risk_taking = my_coach.risk_taking();
    /// assert!(risk_taking == 50_u32);
    /// ```
    pub fn risk_taking(&self) -> u32 {
        self.risk_taking
    }

    /// Get the coach's up-tempo tendency
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    /// 
    /// let my_coach = FootballTeamCoach::new();
    /// let up_tempo = my_coach.up_tempo();
    /// assert!(up_tempo == 50_u32);
    /// ```
    pub fn up_tempo(&self) -> u32 {
        self.up_tempo
    }

    /// Get the coach's run-pass playcalling tendency
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    /// 
    /// let my_coach = FootballTeamCoach::new();
    /// let run_pass = my_coach.run_pass();
    /// assert!(run_pass == 50_u32);
    /// ```
    pub fn run_pass(&self) -> u32 {
        self.run_pass
    }
}

/// # `FootballTeamCoachBuilder` struct
///
/// A `FootballTeamCoachBuilder` implements the builder pattern for the
/// `FootballTeamCoach` struct
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize)]
pub struct FootballTeamCoachBuilder {
    risk_taking: u32,
    run_pass: u32,
    up_tempo: u32
}

impl Default for FootballTeamCoachBuilder {
    /// Default constructor for the FootballTeamCoachBuilder class
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoachBuilder;
    /// 
    /// let my_coach_builder = FootballTeamCoachBuilder::default();
    /// ```
    fn default() -> Self {
        FootballTeamCoachBuilder{
            risk_taking: 50_u32,
            run_pass: 50_u32,
            up_tempo: 50_u32
        }
    }
}

impl FootballTeamCoachBuilder {
    /// Initialize a new coach builder
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoachBuilder;
    ///
    /// let mut my_coach_builder = FootballTeamCoachBuilder::new();
    /// ```
    pub fn new() -> FootballTeamCoachBuilder {
        FootballTeamCoachBuilder::default()
    }

    /// Set the risk taking property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::{FootballTeamCoach, FootballTeamCoachBuilder};
    /// 
    /// let my_coach = FootballTeamCoachBuilder::new()
    ///     .risk_taking(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_coach.risk_taking() == 60);
    /// ```
    pub fn risk_taking(mut self, risk_taking: u32) -> Self {
        self.risk_taking = risk_taking;
        self
    }

    /// Set the run-pass property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::{FootballTeamCoach, FootballTeamCoachBuilder};
    /// 
    /// let my_coach = FootballTeamCoachBuilder::new()
    ///     .run_pass(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_coach.run_pass() == 60);
    /// ```
    pub fn run_pass(mut self, run_pass: u32) -> Self {
        self.run_pass = run_pass;
        self
    }

    /// Set the up-tempo property
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::{FootballTeamCoach, FootballTeamCoachBuilder};
    /// 
    /// let my_coach = FootballTeamCoachBuilder::new()
    ///     .up_tempo(60)
    ///     .build()
    ///     .unwrap();
    /// assert!(my_coach.up_tempo() == 60);
    /// ```
    pub fn up_tempo(mut self, up_tempo: u32) -> Self {
        self.up_tempo = up_tempo;
        self
    }

    /// Build the coach
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::{FootballTeamCoach, FootballTeamCoachBuilder};
    /// 
    /// let my_coach = FootballTeamCoachBuilder::new()
    ///     .risk_taking(40)
    ///     .run_pass(50)
    ///     .up_tempo(60)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<FootballTeamCoach, String> {
        let raw = FootballTeamCoachRaw{
            risk_taking: self.risk_taking,
            run_pass: self.run_pass,
            up_tempo: self.up_tempo
        };
        FootballTeamCoach::try_from(raw)
    }
}
