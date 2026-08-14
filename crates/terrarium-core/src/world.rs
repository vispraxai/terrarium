use crate::event::EventKind;
use crate::{Action, Event, EventId, Person, PersonId, SimTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Emit an event whose default parent is the previous event.
    /// This preserves the original Phase 0 behavior.
    pub fn emit(&mut self, kind: EventKind) -> EventId {
        let parent = self.events.last().map(|e| e.id);
        self.emit_with_parent(parent, kind)
    }

    /// Emit an event with an explicitly selected causal parent.
    pub fn emit_with_parent(
        &mut self,
        causal_parent: Option<EventId>,
        kind: EventKind,
    ) -> EventId {
        let id = EventId(self.next_event_id);
        self.next_event_id += 1;
        self.events.push(Event {
            id,
            timestamp: self.time,
            causal_parent,
            kind,
        });
        id
    }

    pub fn event(&self, id: EventId) -> Option<&Event> {
        self.events.iter().find(|event| event.id == id)
    }

    pub fn events_since(&self, cursor: usize) -> &[Event] {
        let cursor = cursor.min(self.events.len());
        &self.events[cursor..]
    }

    pub fn apply_agent_action(
        &mut self,
        actor: PersonId,
        action: &Action,
    ) -> Result<(), WorldError> {
        if !self.people.contains_key(&actor) {
            return Err(WorldError::UnknownPerson(actor));
        }
        match action {
            Action::Say(text) => {
                self.emit(EventKind::Custom {
                    description: format!("{} said: {}", self.people[&actor].identity.name, text),
                });
            }
            Action::DoNothing => {}
        }
        Ok(())
    }
}

impl Default for WorldState {
    fn default() -> Self { Self::new() }
}
