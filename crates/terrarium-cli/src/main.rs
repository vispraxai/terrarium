//! Small executable demonstration of the Phase 0 research loop.
//!
//! The CLI is intentionally boring. Its job is to prove that the core can
//! record a complete trace that Observatory can later consume.

use terrarium_agent::EchoAgent;
use terrarium_core::{
    Duration, Person, PersonId, RelationshipState, Simulation,
};

fn main() {
    let mut sim = Simulation::with_seed(42);

    let alice = Person::new(PersonId(1), "Alice");
    let mut bob = Person::new(PersonId(2), "Bob");

    bob.relationships.insert(
        alice.id,
        RelationshipState {
            trust: 0.8,
            familiarity: 0.7,
            attachment: 0.7,
            perceived_reciprocity: 0.7,
            conflict: 0.0,
            uncertainty: 0.1,
            shared_history: vec![],
        },
    );

    sim.add_person(alice.clone());
    sim.add_person(bob);

    // A checkpoint is a complete replay starting point. It is intentionally
    // owned by Simulation::run rather than by the CLI.
    sim.checkpoint();

    println!("Terrarium Phase 0 — complete trace demo");
    println!("seed = {}", sim.seed());
    println!("time = {:?}", sim.time());

    sim.advance(Duration::hours(17));
    println!("17:00 — Alice leaves work.");
    sim.checkpoint();

    let promise_event =
        sim.promise_made(alice.id, PersonId(2), "I'll be home at 18:00");
    println!("17:00 — PromiseMade event #{promise_event:?}");
    sim.checkpoint();

    sim.advance(Duration::hours(1));
    println!("18:00 — Bob expects Alice home.");
    sim.checkpoint();

    sim.advance(Duration::minutes(30));
    println!("18:30 — Alice has not arrived.");

    let broken_event = sim.promise_broken(
        alice.id,
        PersonId(2),
        "I'll be home at 18:00",
    );
    println!("18:30 — PromiseBroken event #{broken_event:?}");
    sim.checkpoint();

    let bob = &sim.world.people[&PersonId(2)];
    let relationship = &bob.relationships[&alice.id];

    println!(
        "Bob latent state: trust={:0.2}, conflict={:0.2}, uncertainty={:0.2}",
        relationship.trust,
        relationship.conflict,
        relationship.uncertainty
    );

    // The agent never receives `WorldState`. It receives an observation
    // generated through the observation boundary.
    let mut vixir = EchoAgent;
    let action = sim
        .step_agent(PersonId(2), &mut vixir)
        .expect("Bob exists");

    println!("Vixir action: {action:?}");

    println!("\nUnified trace:");
    for entry in sim.run.timeline() {
        println!("  @ {:?} {:?}", entry.timestamp(), entry);
    }

    println!("\nReplay:");
    for time in [sim.run.events.first().map(|e| e.timestamp), Some(sim.time())]
        .into_iter()
        .flatten()
    {
        if let Some(world) = sim.run.at(time) {
            println!(
                "  at {:?}: {} people, {} recorded world events",
                time,
                world.people.len(),
                world.events.len()
            );
        }
    }

    println!("\nCausal chain for PromiseBroken:");
    for event in sim.run.causal_chain(broken_event) {
        println!(
            "  #{:?} @ {:?} {:?}",
            event.id, event.timestamp, event.kind
        );
    }

    // Branch after the PromiseMade event. The child world is reconstructed at
    // the fork point rather than cloning the parent's current state.
    let counterfactual = sim.branch("counterfactual", 1);

    println!(
        "\nCounterfactual branch: {}",
        counterfactual.run.branch.branch_id
    );
    println!("  fork event: {:?}", counterfactual.run.branch.fork_event);
    println!("  child time: {:?}", counterfactual.time());
    println!("  child events: {}", counterfactual.run.events.len());

    sim.run.validate().expect("run invariants must hold before export");
    let json = sim
        .run
        .to_json_pretty()
        .expect("run should serialize");
    std::fs::write("terrarium-run.json", json)
        .expect("write terrarium-run.json");

    println!("\nWrote terrarium-run.json");
}
