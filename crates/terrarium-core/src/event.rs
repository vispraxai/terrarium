use crate::{Action, EventId, PersonId, SimTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    PersonEnteredRoom { person: PersonId, room: String },
    PersonLeftRoom { person: PersonId, room: String },
    PromiseMade { from: PersonId, to: PersonId, content: String },
    PromiseBroken { from: PersonId, to: PersonId, content: String },
    MessageSent { from: PersonId, to: PersonId, content: String },
    AgentAction { actor: PersonId, action: Action },
    Custom { description: String },
}

/// A machine-readable description of a meaningful latent-state mutation caused
/// by an event. This is deliberately generic enough for Observatory while the
/// simulation's psychological model is still evolving.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateEffect {
    RelationshipFieldChanged {
        person: PersonId,
        other: PersonId,
        field: String,
        before: f32,
        after: f32,
    },
    MemoryAdded {
        person: PersonId,
        description: String,
        salience: f32,
    },
    Custom { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    /// Visible to any observer for whom the simulation later generates a valid observation.
    Public,
    /// The event is latent and should not be directly exposed to agents.
    Latent,
    /// Only these simulated people are eligible to perceive it.
    Persons(Vec<PersonId>),
}

impl Default for Visibility {
    fn default() -> Self { Self::Public }
}

/// Authoritative world-level event. The agent normally receives sensory
/// consequences, not this semantic representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timestamp: SimTime,
    /// Kept as a single optional parent for Phase 0 compatibility.
    /// Use `emit_with_parent` to make causal intent explicit.
    pub causal_parent: Option<EventId>,
    pub kind: EventKind,
    /// Explicit consequences are the bridge toward true event-sourced replay.
    #[serde(default)]
    pub effects: Vec<StateEffect>,
    #[serde(default)]
    pub visibility: Visibility,
}
