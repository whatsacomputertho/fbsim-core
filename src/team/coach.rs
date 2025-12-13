#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket_okapi")]
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// # `FootballTeamCoach` struct
///
/// A `FootballTeamCoach` represents a football team coach
#[cfg_attr(feature = "rocket_okapi", derive(JsonSchema))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct FootballTeamCoach {
    risk_taking: u32,
    run_pass: u32,
    up_tempo: u32
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
        FootballTeamCoach{
            risk_taking: 50_u32,
            run_pass: 50_u32,
            up_tempo: 50_u32
        }
    }

    /// Constructor for the `FootballTeamCoach` struct in which each
    /// property is given as an argument.
    ///
    /// ### Example
    /// ```
    /// use fbsim_core::team::coach::FootballTeamCoach;
    ///
    /// let my_coach = FootballTeamCoach::from_properties(0, 50, 100);
    /// ```
    pub fn from_properties(risk_taking: u32, run_pass: u32, up_tempo: u32) -> Result<FootballTeamCoach, String> {
        // Ensure each skill level is in range
        if risk_taking > 100_u32 {
            return Err(format!("Risk taking not in range [0, 100]: {}", risk_taking))
        }
        if run_pass > 100_u32 {
            return Err(format!("Run-pass not in range [0, 100]: {}", run_pass))
        }
        if up_tempo > 100_u32 {
            return Err(format!("Up tempo not in range [0, 100]: {}", up_tempo))
        }
        Ok(
            FootballTeamCoach{
                risk_taking: risk_taking,
                run_pass: run_pass,
                up_tempo: up_tempo
            }
        )
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
}
