use crate::event::{EventKind, StateEffect, Visibility};
use crate::replay::{BranchInfo, Run};
use crate::{Duration, EventId, Person, PersonId, SimTime, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub world: WorldState,
}

impl Simulation {
    pub fn new() -> Self { Self { world: WorldState::new() } }

    pub fn add_person(&mut self, person: Person) { self.world.add_person(person); }

    pub fn advance(&mut self, duration: Duration) { self.world.time += duration; }

    pub fn promise_made(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) -> EventId {
        self.world.emit(EventKind::PromiseMade {
            from, to, content: content.into(),
        })
    }

    pub fn promise_broken(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) -> EventId {
        let content = content.into();
        let mut effects = Vec::new();
        let memory_description = format!("{} broke a promise: {}", from.0, content);
        if let Some(person) = self.world.people.get_mut(&to) {
            person.remember(self.world.time, memory_description.clone(), 0.8);
            effects.push(StateEffect::MemoryAdded {
                person: to,
                description: memory_description,
                salience: 0.8,
            });
            if let Some(rel) = person.relationships.get_mut(&from) {
                let before_trust = rel.trust;
                let before_conflict = rel.conflict;
                let before_uncertainty = rel.uncertainty;
                rel.trust -= 0.15;
                rel.conflict += 0.10;
                rel.uncertainty += 0.10;
                rel.clamp();
                effects.push(StateEffect::RelationshipFieldChanged {
                    person: to, other: from, field: "trust".into(),
                    before: before_trust, after: rel.trust,
                });
                effects.push(StateEffect::RelationshipFieldChanged {
                    person: to, other: from, field: "conflict".into(),
                    before: before_conflict, after: rel.conflict,
                });
                effects.push(StateEffect::RelationshipFieldChanged {
                    person: to, other: from, field: "uncertainty".into(),
                    before: before_uncertainty, after: rel.uncertainty,
                });
            }
        }
        let parent = self.world.events.last().map(|e| e.id);
        self.world.emit_with_details(
            parent,
            EventKind::PromiseBroken { from, to, content },
            effects,
            Visibility::Public,
        )
    }

    pub fn time(&self) -> SimTime { self.world.time }

    pub fn run(&self, branch_id: u64) -> Run {
        Run::new(&self.world, BranchInfo {
            id: branch_id,
            parent_branch_id: None,
            fork_time: self.world.time,
            fork_event: self.world.events.last().map(|e| e.id),
        })
    }

    /// Clone-based counterfactual branch. The fork is independent of the source
    /// simulation after this point.
    pub fn branch(&self) -> Self { self.clone() }

    pub fn branch_with_info(&self, id: u64, parent_branch_id: Option<u64>) -> (Self, BranchInfo) {
        let fork_event = self.world.events.last().map(|e| e.id);
        (
            self.clone(),
            BranchInfo {
                id,
                parent_branch_id,
                fork_time: self.world.time,
                fork_event,
            },
        )
    }
}

impl Default for Simulation {
    fn default() -> Self { Self::new() }
}
