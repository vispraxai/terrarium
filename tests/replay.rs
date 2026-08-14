use terrarium_core::{
    Action, Duration, EventKind, Observation, Person, PersonId, RelationshipState, Simulation,
    SimTime, SnapshotId,
};

fn simulation() -> Simulation {
    let mut sim = Simulation::new();
    let alice = Person::new(PersonId(1), "Alice");
    let mut bob = Person::new(PersonId(2), "Bob");
    bob.relationships.insert(alice.id, RelationshipState {
        trust: 0.8, familiarity: 0.7, attachment: 0.7,
        perceived_reciprocity: 0.7, conflict: 0.0, uncertainty: 0.1,
        shared_history: vec![],
    });
    sim.add_person(alice);
    sim.add_person(bob);
    sim
}

#[test]
fn event_stream_snapshots_and_replay_are_consistent() {
    let mut sim = simulation();
    let alice = PersonId(1);
    let bob = PersonId(2);
    let mut run = sim.run(0);

    sim.advance(Duration::hours(1));
    let made = sim.promise_made(alice, bob, "home");
    run.capture(&sim.world, "promise");

    assert_eq!(run.events().count(), 1);
    assert!(matches!(run.event(made).unwrap().kind, EventKind::PromiseMade { .. }));
    assert_eq!(run.at(SimTime(0)).unwrap().events.len(), 0);
    assert_eq!(run.at(SimTime(3600)).unwrap().events.len(), 1);
}

#[test]
fn events_are_synced_even_when_no_snapshot_is_taken_after_event() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    sim.world.emit(EventKind::Custom { description: "unsnapped".into() });
    run.sync_events(&sim.world);
    assert_eq!(run.events().count(), 1);
    assert_eq!(run.latest().world.events.len(), 0);
}

#[test]
fn observations_and_actions_are_recorded_separately_from_latent_truth() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    let event = sim.world.emit(EventKind::Custom { description: "door opened".into() });

    let observation = Observation::Text {
        timestamp: sim.time(),
        text: "You hear a door open.".into(),
    };
    run.record_observation(sim.time(), Some(PersonId(2)), observation, [event]);

    let action = Action::Say("Hello?".into());
    run.record_action(sim.time(), PersonId(2), action, [event]);

    assert_eq!(run.observations.len(), 1);
    assert_eq!(run.observations[0].caused_by, vec![event]);
    assert_eq!(run.actions.len(), 1);
    assert_eq!(run.actions[0].caused_by, vec![event]);
}

#[test]
fn explicit_causal_parent_is_preserved() {
    let mut sim = simulation();
    let first = sim.world.emit(EventKind::Custom { description: "root".into() });
    sim.advance(Duration::seconds(1));
    let second = sim.world.emit_with_parent(
        Some(first),
        EventKind::Custom { description: "child".into() },
    );

    let mut run = sim.run(0);
    run.sync_events(&sim.world);
    let chain = run.causal_chain(second);
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].id, first);
    assert_eq!(chain[1].id, second);
}

#[test]
fn run_can_fork_from_a_snapshot_for_counterfactual_history() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    sim.advance(Duration::hours(1));
    sim.promise_made(PersonId(1), PersonId(2), "home");
    let fork_snapshot = run.capture(&sim.world, "counterfactual fork point");

    sim.advance(Duration::hours(1));
    sim.promise_broken(PersonId(1), PersonId(2), "home");
    run.capture(&sim.world, "actual outcome");

    let branch = run.fork(
        fork_snapshot,
        terrarium_core::BranchInfo {
            id: 1,
            parent_branch_id: Some(0),
            fork_time: SimTime(3600),
            fork_event: Some(terrarium_core::EventId(0)),
        },
    ).expect("fork snapshot exists");

    assert_eq!(branch.branch.id, 1);
    assert_eq!(branch.snapshots.len(), 2);
    assert_eq!(branch.events.len(), 1);
    assert_eq!(branch.events[0].id, terrarium_core::EventId(0));
}

#[test]
fn event_range_query_is_time_ordered() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    sim.world.emit(EventKind::Custom { description: "a".into() });
    sim.advance(Duration::seconds(10));
    sim.world.emit(EventKind::Custom { description: "b".into() });
    run.sync_events(&sim.world);

    let events: Vec<_> = run.events_between(SimTime(10), SimTime(10)).collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].id, terrarium_core::EventId(1));
    assert!(run.snapshot(SnapshotId(0)).is_some());
}

#[test]
fn promise_break_event_contains_latent_state_effects() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    sim.advance(Duration::hours(1));
    let id = sim.promise_broken(PersonId(1), PersonId(2), "home");
    run.capture(&sim.world, "broken promise");
    let event = run.event(id).unwrap();
    assert!(event.effects.len() >= 4);
    assert!(matches!(event.visibility, terrarium_core::Visibility::Public));
}

#[test]
fn fork_uses_snapshot_cursors_not_timestamps() {
    let mut sim = simulation();
    let mut run = sim.run(0);
    let first = sim.world.emit(EventKind::Custom { description: "first".into() });
    let snap = run.capture(&sim.world, "fork");
    let second = sim.world.emit(EventKind::Custom { description: "second at same time".into() });
    run.sync_events(&sim.world);
    assert_eq!(first, terrarium_core::EventId(0));
    assert_eq!(second, terrarium_core::EventId(1));
    let branch = run.fork(snap, terrarium_core::BranchInfo { id: 3, parent_branch_id: Some(0), fork_time: SimTime(0), fork_event: Some(first) }).unwrap();
    assert_eq!(branch.events.len(), 1);
    assert_eq!(branch.events[0].id, first);
}
