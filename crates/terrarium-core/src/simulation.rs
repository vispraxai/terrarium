use crate::event::EventKind;
use crate::{Duration, Person, PersonId, SimTime, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub world: WorldState,
}

impl Simulation {
    pub fn new() -> Self {
        Self { world: WorldState::new() }
    }

    pub fn add_person(&mut self, person: Person) {
        self.world.add_person(person);
    }

    pub fn advance(&mut self, duration: Duration) {
        self.world.time += duration;
    }

    pub fn promise_made(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) {
        self.world.emit(EventKind::PromiseMade {
            from,
            to,
            content: content.into(),
        });
    }

    pub fn promise_broken(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) {
        let content = content.into();

        // World-level psychological consequence: deliberately simple for v0.
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

        self.world.emit(EventKind::PromiseBroken { from, to, content });
    }

    pub fn time(&self) -> SimTime {
        self.world.time
    }

    /// Snapshot/branch primitive. Since the state is owned and serializable,
    /// cloning gives us a deterministic branch in this first implementation.
    pub fn branch(&self) -> Self {
        self.clone()
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
