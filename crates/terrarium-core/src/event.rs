use crate::{EventId, PersonId, SimTime};
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

/// Authoritative world-level event. The agent normally receives sensory
/// consequences, not this semantic representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timestamp: SimTime,
    pub causal_parent: Option<EventId>,
    pub kind: EventKind,
}
