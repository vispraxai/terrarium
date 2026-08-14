use crate::{PersonId, SimTime};
use serde::{Deserialize, Serialize};

/// What the agent is allowed to see. It deliberately contains no direct
/// references to latent WorldState fields such as beliefs or affect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Observation {
    Text {
        timestamp: SimTime,
        text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Say(String),
    DoNothing,
}

/// Adapter boundary between Terrarium and an arbitrary cognitive architecture.
pub trait Agent {
    fn observe(&mut self, observation: Observation) -> Action;
}

/// Useful when an experiment wants to associate an action with its actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub actor: PersonId,
    pub action: Action,
}
