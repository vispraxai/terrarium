use terrarium_core::{
    Duration, EventKind, Person, PersonId, RelationshipState, Simulation, SimTime,
};

#[test]
fn event_stream_snapshots_and_replay_are_consistent() {
    let mut sim = Simulation::new();
    let alice = Person::new(PersonId(1), "Alice");
    let mut bob = Person::new(PersonId(2), "Bob");
    bob.relationships.insert(alice.id, RelationshipState {
        trust: 0.8, familiarity: 0.7, attachment: 0.7,
        perceived_reciprocity: 0.7, conflict: 0.0, uncertainty: 0.1,
        shared_history: vec![],
    });
    sim.add_person(alice.clone());
    sim.add_person(bob);

    let mut run = sim.run(0);
    sim.advance(Duration::hours(1));
    let made = sim.promise_made(alice.id, PersonId(2), "home");
    run.capture(&sim.world, "promise");

    assert_eq!(sim.world.events.len(), 1);
    assert!(matches!(sim.world.event(made).unwrap().kind, EventKind::PromiseMade { .. }));
    assert_eq!(run.at(SimTime(0)).unwrap().events.len(), 0);
    assert_eq!(run.at(SimTime(3600)).unwrap().events.len(), 1);

    let (branch, info) = sim.branch_with_info(7, Some(0));
    assert_eq!(info.id, 7);
    assert_eq!(info.parent_branch_id, Some(0));
    assert_eq!(branch.time(), sim.time());
}

#[test]
fn explicit_causal_parent_is_preserved() {
    let mut sim = Simulation::new();
    let first = sim.world.emit(EventKind::Custom { description: "root".into() });
    sim.advance(Duration::seconds(1));
    let second = sim.world.emit_with_parent(
        Some(first),
        EventKind::Custom { description: "child".into() },
    );

    let chain = sim.run(0).causal_chain(second);
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].id, first);
    assert_eq!(chain[1].id, second);
}
