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

    // Record a snapshot at the start, then after meaningful world transitions.
    let mut run = sim.run(0);

    println!("Terrarium v0 — event/snapshot/replay demo");
    println!("time = {:?}", sim.time());

    sim.advance(Duration::hours(17));
    println!("17:00 — Alice leaves work.");
    run.capture(&sim.world, "Alice leaves work");

    let promise_event = sim.promise_made(alice.id, PersonId(2), "I'll be home at 18:00");
    println!("17:00 — PromiseMade event #{:?}", promise_event);
    run.capture(&sim.world, "promise made");

    sim.advance(Duration::hours(1));
    println!("18:00 — Bob expects Alice home.");
    run.capture(&sim.world, "Bob expectation point");

    sim.advance(Duration::minutes(30));
    println!("18:30 — Alice has not arrived.");

    let broken_event = sim.promise_broken(
        alice.id,
        PersonId(2),
        "I'll be home at 18:00".to_string(),
    );
    println!("18:30 — PromiseBroken event #{:?}", broken_event);
    run.capture(&sim.world, "promise broken and psychological consequence");

    let bob = &sim.world.people[&PersonId(2)];
    let relationship = &bob.relationships[&alice.id];
    println!(
        "Bob latent state: trust={:.2}, conflict={:.2}, uncertainty={:.2}",
        relationship.trust, relationship.conflict, relationship.uncertainty
    );

    let mut vixir = EchoAgent;
    let observation = Observation::Text {
        timestamp: sim.time(),
        text: "Bob says: I thought Alice was going to be home by now.".into(),
    };
    let action = vixir.observe(observation.clone());
    run.record_observation(sim.time(), Some(PersonId(2)), observation, [broken_event]);
    run.record_action(sim.time(), PersonId(2), action.clone(), [broken_event]);
    let action_event = sim.world.apply_agent_action(PersonId(2), &action).expect("known actor");
    run.sync_events(&sim.world);
    println!("AgentAction event #{:?}", action_event);
    println!("Vixir action: {:?}", action);

    println!("\nEvent stream:");
    for event in run.events() {
        println!("  #{:?} @ {:?} {:?}", event.id, event.timestamp, event.kind);
    }

    println!("\nReplay:");
    for t in [sim.world.events.first().map(|e| e.timestamp), Some(sim.time())]
        .into_iter()
        .flatten()
    {
        if let Some(world) = run.at(t) {
            println!("  at {:?}: {} people, {} events", t, world.people.len(), world.events.len());
        }
    }

    println!("\nCausal chain for PromiseBroken:");
    for event in run.causal_chain(broken_event) {
        println!("  #{:?} @ {:?} {:?}", event.id, event.timestamp, event.kind);
    }

    let (broken_world, broken_info) = sim.branch_with_info(1, Some(0));
    let (kept_world, kept_info) = sim.branch_with_info(2, Some(0));
    println!(
        "\nCounterfactual branches at {:?}: broken=#{}, kept=#{}",
        sim.time(), broken_info.id, kept_info.id
    );
    println!("  both are independent clones: {}", broken_world.time() == kept_world.time());
    let counterfactual = run.fork(
        run.latest().id,
        kept_info.clone(),
    ).expect("latest snapshot can be forked");
    println!("  replay fork contains {} events and {} observations", counterfactual.events.len(), counterfactual.observations.len());

    let json = run.to_json_pretty().expect("run should serialize");
    std::fs::write("terrarium-run.json", json).expect("write terrarium-run.json");
    println!("\nWrote terrarium-run.json");
}
