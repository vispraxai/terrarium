//! Explicit state changes produced by simulation events.
//!
//! An `Event` is history. A `StateEffect` is the deterministic state delta
//! needed to reproduce that history.  Important state mutations should pass
//! through this vocabulary instead of being hidden inside arbitrary methods.

use crate::{PersonId, SimTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateEffect {
    PersonEnteredRoom {
        person: PersonId,
        room: String,
    },
    PersonLeftRoom {
        person: PersonId,
        room: String,
    },
    MemoryAdded {
        person: PersonId,
        timestamp: SimTime,
        description: String,
        salience: f32,
    },
    BeliefChanged {
        person: PersonId,
        proposition: String,
        old_confidence: f32,
        new_confidence: f32,
    },
    AffectChanged {
        person: PersonId,
        old_valence: f32,
        new_valence: f32,
        old_arousal: f32,
        new_arousal: f32,
    },
    RelationshipChanged {
        observer: PersonId,
        target: PersonId,
        trust_delta: f32,
        conflict_delta: f32,
        uncertainty_delta: f32,
    },
}
