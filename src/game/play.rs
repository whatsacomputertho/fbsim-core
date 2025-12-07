pub mod context;
pub mod result;

use crate::team::coach::FootballTeamCoach;
use crate::team::defense::FootballTeamDefense;
use crate::team::offense::FootballTeamOffense;

pub trait PlaySimulatable {
    fn coach(&self) -> &FootballTeamCoach;
    fn defense(&self) -> &FootballTeamDefense;
    fn offense(&self) -> &FootballTeamOffense;
}
