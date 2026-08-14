use crate::event::EventKind;
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
        if let Some(person) = self.world.people.get_mut(&to) {
            person.remember(
                self.world.time,
                format!("{} broke a promise: {}", from.0, content),
                0.8,
            );
            if let Some(rel) = person.relationships.get_mut(&from) {
                rel.trust -= 0.15;
                rel.conflict += 0.10;
                rel.uncertainty += 0.10;
                rel.clamp();
            }
        }
        self.world.emit(EventKind::PromiseBroken { from, to, content })
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
