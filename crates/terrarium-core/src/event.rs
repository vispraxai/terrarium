//! Semantic events form the durable history of the simulated world.
//!
//! The event is intentionally not the whole state mutation. Its `effects`
//! field makes the consequences explicit, which is what later lets replay
//! reconstruct historical state without rerunning arbitrary domain code.

use crate::{EventId, PersonId, SimTime};
use super::effect::StateEffect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    PersonEnteredRoom { person: PersonId, room: String },
    PersonLeftRoom { person: PersonId, room: String },
    PromiseMade { from: PersonId, to: PersonId, content: String },
    PromiseBroken { from: PersonId, to: PersonId, content: String },
    MessageSent { from: PersonId, to: PersonId, content: String },
    Custom { description: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timestamp: SimTime,
    /// More than one parent is supported because a real consequence can
    /// depend on several earlier facts. A single parent remains convenient
    /// for the current Phase 0 examples.
    pub causal_parents: Vec<EventId>,
    pub kind: EventKind,
    pub effects: Vec<StateEffect>,
}
