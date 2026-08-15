//! Declarative experiment descriptions and deterministic instantiation.
//!
//! The experiment type deliberately describes *initial conditions and an
//! intervention*, not implementation details of a cognitive architecture.
//! That keeps experiment specifications portable across agent implementations.

use crate::{Person, PersonId, RelationshipState, Simulation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experiment {
    pub name: String,
    pub duration: String,
    pub people: Vec<ExperimentPerson>,
    pub relationship: Option<RelationshipSetup>,
    pub intervention: Option<Intervention>,
    #[serde(default)]
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExperimentPerson {
    pub id: PersonId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelationshipSetup {
    pub from: PersonId,
    pub to: PersonId,
    pub trust: f32,
    pub attachment: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Intervention {
    #[serde(rename = "type")]
    pub kind: String,
    pub promise: Option<String>,
}

impl Experiment {
    /// Instantiate only the initial world. Running the experiment is kept
    /// separate so callers can inspect or modify the simulation before time
    /// advances.
    pub fn instantiate(&self) -> Result<Simulation, String> {
        let mut simulation = Simulation::with_seed(self.seed);

        for person in &self.people {
            simulation.add_person(Person::new(person.id, person.name.clone()));
        }

        if let Some(relationship) = &self.relationship {
            if !simulation.world.people.contains_key(&relationship.to) {
                return Err(format!("unknown relationship target {:?}", relationship.to));
            }
            let person = simulation
                .world
                .people
                .get_mut(&relationship.from)
                .ok_or_else(|| format!("unknown relationship source {:?}", relationship.from))?;
            person.relationships.insert(
                relationship.to,
                RelationshipState {
                    trust: relationship.trust,
                    familiarity: 0.0,
                    attachment: relationship.attachment,
                    perceived_reciprocity: 0.0,
                    conflict: 0.0,
                    uncertainty: 0.0,
                    shared_history: Vec::new(),
                },
            );
        }

        // The relationship/person setup is initial condition, not a semantic
        // world event. Take one final setup checkpoint after all setup changes.
        simulation.checkpoint();
        Ok(simulation)
    }

    /// Run the declared duration. Interventions remain explicit operations so
    /// their timing is not silently guessed by the schema.
    pub fn run_duration(&self, simulation: &mut Simulation) -> Result<(), String> {
        let duration = crate::Duration::parse(&self.duration)?;
        simulation.advance(duration);
        simulation.checkpoint();
        Ok(())
    }
}
