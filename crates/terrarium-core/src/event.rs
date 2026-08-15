//! Semantic events form Terrarium's durable history.
//!
//! An event answers "what happened?" while its `effects` answer
//! "what changed because it happened?".  Keeping those concepts separate is
//! what makes replay, debugging, and the future Observatory possible.

use crate::effect::StateEffect;
use crate::{EventId, PersonId, SimTime};
use serde::{Deserialize, Serialize};

/// Controls which agents are allowed to receive an event through the
/// observation layer.  The latent world may know much more than any one agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Latent,
    Persons(Vec<PersonId>),
}

impl Visibility {
    pub fn visible_to(&self, observer: PersonId) -> bool {
        match self {
            Self::Public => true,
            Self::Latent => false,
            Self::Persons(ids) => ids.contains(&observer),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventKind {
    PersonEnteredRoom {
        person: PersonId,
        room: String,
    },
    PersonLeftRoom {
        person: PersonId,
        room: String,
    },
    PromiseMade {
        from: PersonId,
        to: PersonId,
        content: String,
    },
    PromiseBroken {
        from: PersonId,
        to: PersonId,
        content: String,
    },
    MessageSent {
        from: PersonId,
        to: PersonId,
        content: String,
    },
    AgentAction {
        actor: PersonId,
        description: String,
    },
    /// A meaningful affective transition. Exact old/new values live in the effect.
    AffectChanged {
        person: PersonId,
    },
    Custom {
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub timestamp: SimTime,

    /// Ordering within a durable run. WorldState assigns event ids; Run assigns
    /// this trace sequence so events, observations, and actions share one order.
    #[serde(default)]
    pub trace_sequence: u64,

    /// More than one parent is supported because a consequence may depend on
    /// several earlier facts.  Phase 0 mostly uses one parent.
    pub causal_parents: Vec<EventId>,

    pub kind: EventKind,

    /// Visibility belongs to the event, not the observation.  This lets the
    /// same latent event produce different observations for different agents.
    pub visibility: Visibility,

    pub effects: Vec<StateEffect>,
}
