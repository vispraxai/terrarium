//! Authoritative latent world state.
//!
//! This module is the only place where recorded `StateEffect`s are applied.
//! That invariant is important: if a meaningful mutation bypasses
//! `apply_effect`, replay cannot reproduce it later.

use crate::effect::StateEffect;
use crate::{Action, Event, EventId, EventKind, Person, PersonId, SimTime, Visibility};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    pub time: SimTime,
    pub people: HashMap<PersonId, Person>,
    pub locations: HashMap<String, Location>,
    pub events: Vec<Event>,
    next_event_id: u64,
}

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("person {0:?} does not exist")]
    UnknownPerson(PersonId),
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            time: SimTime(0),
            people: HashMap::new(),
            locations: HashMap::new(),
            events: Vec::new(),
            next_event_id: 0,
        }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.insert(person.id, person);
    }

    /// Reconcile the private event-id allocator after replaying historical
    /// events. Replay copies event records but does not call `emit`, so the
    /// allocator must be restored explicitly before the branch continues.
    pub(crate) fn sync_next_event_id(&mut self) {
        self.next_event_id = self
            .events
            .iter()
            .map(|event| event.id.0)
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);
    }

    /// Apply exactly one deterministic state delta.
    pub fn apply_effect(&mut self, effect: &StateEffect) {
        match effect {
            StateEffect::PersonEnteredRoom { person, room } => {
                if let Some(p) = self.people.get_mut(person) {
                    p.location = Some(room.clone());
                }
            }
            StateEffect::PersonLeftRoom { person, room: _ } => {
                if let Some(p) = self.people.get_mut(person) {
                    p.location = None;
                }
            }
            StateEffect::MemoryAdded {
                person,
                timestamp,
                description,
                salience,
            } => {
                if let Some(p) = self.people.get_mut(person) {
                    p.remember(*timestamp, description.clone(), *salience);
                }
            }
            StateEffect::BeliefChanged {
                person,
                proposition,
                new_confidence,
                ..
            } => {
                if let Some(p) = self.people.get_mut(person) {
                    if let Some(belief) = p
                        .beliefs
                        .beliefs
                        .iter_mut()
                        .find(|belief| belief.proposition == *proposition)
                    {
                        belief.confidence = new_confidence.clamp(0.0, 1.0);
                    } else {
                        p.beliefs.beliefs.push(crate::Belief {
                            proposition: proposition.clone(),
                            confidence: new_confidence.clamp(0.0, 1.0),
                        });
                    }
                }
            }
            StateEffect::AffectChanged {
                person,
                new_valence,
                new_arousal,
                ..
            } => {
                if let Some(p) = self.people.get_mut(person) {
                    p.affect.valence = *new_valence;
                    p.affect.arousal = *new_arousal;
                }
            }
            StateEffect::RelationshipChanged {
                observer,
                target,
                trust_delta,
                conflict_delta,
                uncertainty_delta,
            } => {
                if let Some(p) = self.people.get_mut(observer) {
                    if let Some(r) = p.relationships.get_mut(target) {
                        r.trust += trust_delta;
                        r.conflict += conflict_delta;
                        r.uncertainty += uncertainty_delta;
                        r.clamp();
                    }
                }
            }
        }
    }

    /// Emit a public event.  This keeps the old convenient API for ordinary
    /// world events while the more explicit method handles hidden events.
    pub fn emit(&mut self, kind: EventKind, effects: Vec<StateEffect>) -> EventId {
        self.emit_with_visibility(kind, effects, Visibility::Public)
    }

    pub fn emit_with_visibility(
        &mut self,
        kind: EventKind,
        effects: Vec<StateEffect>,
        visibility: Visibility,
    ) -> EventId {
        let parents = self.events.last().map(|event| event.id).into_iter().collect();
        self.emit_with_parents(kind, effects, visibility, parents)
    }

    /// Emit an event with explicit causal parents.  This is the escape hatch
    /// for consequences that genuinely depend on multiple earlier events.
    pub fn emit_with_parents(
        &mut self,
        kind: EventKind,
        effects: Vec<StateEffect>,
        visibility: Visibility,
        causal_parents: Vec<EventId>,
    ) -> EventId {
        let id = EventId(self.next_event_id);
        self.next_event_id += 1;

        for effect in &effects {
            self.apply_effect(effect);
        }

        self.events.push(Event {
            id,
            timestamp: self.time,
            trace_sequence: 0,
            causal_parents,
            kind,
            visibility,
            effects,
        });

        id
    }

    /// Convert an agent action into a world event.  Returning the event id lets
    /// the caller connect the action to the following causal chain.
    pub fn apply_agent_action(
        &mut self,
        actor: PersonId,
        action: &Action,
    ) -> Result<EventId, WorldError> {
        self.apply_agent_action_with_parents(actor, action, Vec::new())
    }

    pub fn apply_agent_action_with_parents(
        &mut self,
        actor: PersonId,
        action: &Action,
        parents: Vec<EventId>,
    ) -> Result<EventId, WorldError> {
        if !self.people.contains_key(&actor) {
            return Err(WorldError::UnknownPerson(actor));
        }

        let kind = match action {
            Action::Say(text) => {
                let name = self.people[&actor].identity.name.clone();
                EventKind::AgentAction {
                    actor,
                    description: format!("{name} said: {text}"),
                }
            }
            Action::DoNothing => EventKind::AgentAction {
                actor,
                description: "did nothing".into(),
            },
        };

        let id = EventId(self.next_event_id);
        self.next_event_id += 1;
        self.events.push(Event {
            id,
            timestamp: self.time,
            trace_sequence: 0,
            causal_parents: if parents.is_empty() {
                self.events.last().map(|e| e.id).into_iter().collect()
            } else {
                parents
            },
            kind,
            visibility: Visibility::Public,
            effects: Vec::new(),
        });

        Ok(id)
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
