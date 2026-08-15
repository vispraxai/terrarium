//! Explicit state changes produced by simulation events.
//!
//! An `Event` answers "what happened?" while a `StateEffect` answers
//! "what changed because it happened?" Keeping these separate is the key
//! to making Terrarium history replayable and inspectable.

use crate::{PersonId, SimTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateEffect {
    PersonEnteredRoom { person: PersonId, room: String },
    PersonLeftRoom { person: PersonId, room: String },
    MemoryAdded {
        person: PersonId,
        timestamp: SimTime,
        description: String,
        salience: f32,
    },
    RelationshipChanged {
        observer: PersonId,
        target: PersonId,
        trust_delta: f32,
        conflict_delta: f32,
        uncertainty_delta: f32,
    },
}
