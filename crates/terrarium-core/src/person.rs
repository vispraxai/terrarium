use crate::{PersonId, SimTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal { pub description: String, pub importance: f32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub proposition: String,
    /// Probability assigned by the simulated person, not objective truth.
    pub confidence: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BeliefState { pub beliefs: Vec<Belief> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub timestamp: SimTime,
    pub description: String,
    pub salience: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStore { pub memories: Vec<Memory> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipState {
    pub trust: f32,
    pub familiarity: f32,
    pub attachment: f32,
    pub perceived_reciprocity: f32,
    pub conflict: f32,
    pub uncertainty: f32,
    pub shared_history: Vec<String>,
}
impl RelationshipState {
    pub fn clamp(&mut self) {
        self.trust = self.trust.clamp(0.0, 1.0);
        self.familiarity = self.familiarity.clamp(0.0, 1.0);
        self.attachment = self.attachment.clamp(0.0, 1.0);
        self.perceived_reciprocity = self.perceived_reciprocity.clamp(0.0, 1.0);
        self.conflict = self.conflict.clamp(0.0, 1.0);
        self.uncertainty = self.uncertainty.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Expectations { pub expectations: Vec<Belief> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectiveState { pub valence: f32, pub arousal: f32 }
impl Default for AffectiveState {
    fn default() -> Self { Self { valence: 0.0, arousal: 0.0 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel { pub capability_estimate: f32, pub uncertainty: f32 }
impl Default for SelfModel {
    fn default() -> Self { Self { capability_estimate: 0.5, uncertainty: 0.5 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: PersonId,
    pub identity: Identity,
    pub goals: Vec<Goal>,
    pub beliefs: BeliefState,
    pub memories: MemoryStore,
    pub expectations: Expectations,
    pub affect: AffectiveState,
    pub relationships: HashMap<PersonId, RelationshipState>,
    pub self_model: SelfModel,
}
impl Person {
    pub fn new(id: PersonId, name: impl Into<String>) -> Self {
        Self {
            id,
            identity: Identity { name: name.into() },
            goals: Vec::new(),
            beliefs: BeliefState::default(),
            memories: MemoryStore::default(),
            expectations: Expectations::default(),
            affect: AffectiveState::default(),
            relationships: HashMap::new(),
            self_model: SelfModel::default(),
        }
    }
    pub fn remember(&mut self, timestamp: SimTime, description: impl Into<String>, salience: f32) {
        self.memories.memories.push(Memory {
            timestamp,
            description: description.into(),
            salience: salience.clamp(0.0, 1.0),
        });
    }
}
