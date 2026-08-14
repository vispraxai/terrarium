use terrarium_agent::EchoAgent;
use terrarium_core::agent::{Agent, Observation};
use terrarium_core::{
    Duration, Person, PersonId, RelationshipState, Simulation,
};

fn main() {
    let mut sim = Simulation::new();

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

    println!("Terrarium v0");
    println!("time = {:?}", sim.time());

    sim.advance(Duration::hours(17));
    println!("17:00 — Alice leaves work.");

    sim.promise_made(alice.id, PersonId(2), "I'll be home at 18:00");
    sim.advance(Duration::hours(1));
    println!("18:00 — Bob expects Saeid home.");

    sim.advance(Duration::minutes(30));
    println!("18:30 — Alice has not arrived.");

    sim.promise_broken(
        alice.id,
        PersonId(2),
        "I'll be home at 18:00".to_string(),
    );

    let bob = &sim.world.people[&PersonId(2)];
    let relationship = &bob.relationships[&alice.id];

    println!(
        "Bob latent state: trust={:.2}, conflict={:.2}, uncertainty={:.2}",
        relationship.trust, relationship.conflict, relationship.uncertainty
    );

    // Crucially, the agent receives an observation, not Bob's latent state.
    let mut vixir = EchoAgent;
    let observation = Observation::Text {
        timestamp: sim.time(),
        text: "Bob says: I thought Alice was going to be home by now.".into(),
    };

    let action = vixir.observe(observation);
    println!("Vixir action: {:?}", action);

    println!("\nWorld events:");
    for event in &sim.world.events {
        println!("  {:?}", event);
    }

    // Demonstrate counterfactual branching.
    let broken_world = sim.branch();
    let kept_world = sim.branch();

    println!(
        "\nBranches created at simulated time {:?}: broken={:?}, kept={:?}",
        sim.time(),
        broken_world.time(),
        kept_world.time()
    );
}
