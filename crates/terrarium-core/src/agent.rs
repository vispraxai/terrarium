//! The boundary between Terrarium's latent world and an artificial agent.
//!
//! Observations are intentionally lossy.  An agent should receive what its
//! sensors/observation model exposes, not a direct reference to `WorldState`.

use crate::{EventId, PersonId, SimTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Observation {
    Text {
        timestamp: SimTime,
        observer: PersonId,
        text: String,
        source_events: Vec<EventId>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    Say(String),
    DoNothing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentAction {
    pub actor: PersonId,
    pub action: Action,
}

/// The minimal interface Terrarium requires from an external cognitive system.
///
/// Terrarium owns the world and the observation boundary; the agent owns the
/// decision. Keeping this trait here makes the dependency direction explicit:
/// an agent can inhabit Terrarium without Terrarium knowing how the agent works.
pub trait Agent {
    fn observe(&mut self, observation: Observation) -> Action;
}
