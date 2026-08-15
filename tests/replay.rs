use terrarium_core::{
    Duration, EventKind, Person, PersonId, RelationshipState, Simulation, StateEffect,
};

#[test]
fn promise_break_is_replayable() {
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

    sim.add_person(alice);
    sim.add_person(bob);
    sim.advance(Duration::hours(18));
    sim.promise_broken(PersonId(1), PersonId(2), "home by 18:00");

    let replayed = sim.run.replay_to(sim.time()).unwrap();
    let rel = &replayed.people[&PersonId(2)].relationships[&PersonId(1)];

    assert!((rel.trust - 0.65).abs() < 1e-6);
    assert_eq!(replayed.people[&PersonId(2)].memories.memories.len(), 1);
}

#[test]
fn branch_uses_exact_event_cursor_and_world_state() {
    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));
    sim.add_person(Person::new(PersonId(2), "Bob"));

    sim.promise_made(PersonId(1), PersonId(2), "one");
    sim.checkpoint();
    sim.advance(Duration::seconds(10));
    sim.promise_broken(PersonId(1), PersonId(2), "two");

    let child = sim.branch("counterfactual", 1);

    assert_eq!(child.run.events.len(), 1);
    assert!(matches!(
        child.run.events[0].kind,
        EventKind::PromiseMade { .. }
    ));
    assert_eq!(child.time(), terrarium_core::SimTime(0));
}

#[test]
fn effects_are_serializable() {
    let effect = StateEffect::MemoryAdded {
        person: PersonId(1),
        timestamp: terrarium_core::SimTime(3),
        description: "x".into(),
        salience: 0.5,
    };

    let json = serde_json::to_string(&effect).unwrap();
    let round: StateEffect = serde_json::from_str(&json).unwrap();

    assert_eq!(effect, round);
}

#[test]
fn observation_does_not_expose_latent_events() {
    use terrarium_core::{EventKind, Visibility};

    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));
    sim.add_person(Person::new(PersonId(2), "Bob"));

    sim.schedule_event(
        terrarium_core::SimTime(5),
        EventKind::Custom {
            description: "secret".into(),
        },
        vec![],
        Visibility::Latent,
    );
    sim.advance(Duration::seconds(5));

    let observation = sim.observe(PersonId(1)).unwrap();

    match observation {
        terrarium_core::Observation::Text {
            source_events, ..
        } => assert!(source_events.is_empty()),
    }
}

#[test]
fn agent_step_is_recorded_as_observation_action_and_event() {
    use terrarium_agent::EchoAgent;

    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));

    let mut agent = EchoAgent;
    sim.promise_made(PersonId(1), PersonId(1), "hello");

    let action = sim.step_agent(PersonId(1), &mut agent).unwrap();

    assert!(!sim.run.observations.is_empty());
    assert!(!sim.run.actions.is_empty());
    assert!(!sim.run.events.is_empty());
    assert!(matches!(action, terrarium_core::Action::Say(_)));
}


#[test]
fn state_effects_cover_location_belief_and_affect() {
    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));

    sim.enter_room(PersonId(1), "kitchen");
    sim.set_belief(PersonId(1), "the kitchen is safe", 0.9);
    sim.set_affect(PersonId(1), -0.2, 0.7);

    let replayed = sim.run.replay_to(sim.time()).unwrap();
    let alice = &replayed.people[&PersonId(1)];

    assert_eq!(alice.location.as_deref(), Some("kitchen"));
    assert!((alice.beliefs.beliefs[0].confidence - 0.9).abs() < 1e-6);
    assert!((alice.affect.valence + 0.2).abs() < 1e-6);
    assert!((alice.affect.arousal - 0.7).abs() < 1e-6);
}

#[test]
fn scheduled_events_execute_in_time_order() {
    use terrarium_core::{EventKind, SimTime, Visibility};

    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));

    sim.schedule_event(
        SimTime(20),
        EventKind::Custom {
            description: "later".into(),
        },
        vec![],
        Visibility::Public,
    );
    sim.schedule_event(
        SimTime(10),
        EventKind::Custom {
            description: "earlier".into(),
        },
        vec![],
        Visibility::Public,
    );

    sim.advance(Duration::seconds(30));

    assert_eq!(sim.run.events.len(), 2);
    assert_eq!(sim.run.events[0].timestamp, SimTime(10));
    assert_eq!(sim.run.events[1].timestamp, SimTime(20));
    assert_eq!(sim.time(), SimTime(30));
}

#[test]
fn seeded_randomness_is_reproducible() {
    let mut a = Simulation::with_seed(1234);
    let mut b = Simulation::with_seed(1234);

    assert_eq!(a.random_u64(), b.random_u64());
    assert_eq!(a.random_u64(), b.random_u64());
}

#[test]
fn timeline_is_chronological_and_preserves_agent_loop_order() {
    use terrarium_agent::EchoAgent;
    use terrarium_core::TraceEntry;

    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));
    sim.promise_made(PersonId(1), PersonId(1), "hello");

    let mut agent = EchoAgent;
    sim.step_agent(PersonId(1), &mut agent).unwrap();

    let timeline = sim.run.timeline();
    assert!(timeline.len() >= 4);
    assert!(matches!(timeline[0], TraceEntry::Event(_)));
    assert!(matches!(timeline[1], TraceEntry::Observation(_)));
    assert!(matches!(timeline[2], TraceEntry::Event(_)));
    assert!(matches!(timeline[3], TraceEntry::Action(_)));
}

#[test]
fn run_artifact_contains_versioned_timeline() {
    let mut sim = Simulation::with_seed(7);
    sim.add_person(Person::new(PersonId(1), "Alice"));
    sim.promise_made(PersonId(1), PersonId(1), "hello");

    let json = sim.run.to_json_pretty().unwrap();
    assert!(json.contains("schema_version"));
    assert!(json.contains("timeline"));
    assert!(json.contains("PromiseMade"));
}

#[test]
fn duration_parser_handles_compound_values() {
    assert_eq!(terrarium_core::Duration::parse("2h30m").unwrap(), Duration::minutes(150));
    assert_eq!(terrarium_core::Duration::parse("1d5m").unwrap(), Duration::seconds(86_700));
    assert!(terrarium_core::Duration::parse("2x").is_err());
}

#[test]
fn experiment_instantiation_is_deterministic() {
    use terrarium_core::{Experiment, ExperimentPerson, RelationshipSetup};

    let experiment = Experiment {
        name: "small".into(),
        duration: "2h30m".into(),
        people: vec![
            ExperimentPerson { id: PersonId(1), name: "Alice".into() },
            ExperimentPerson { id: PersonId(2), name: "Bob".into() },
        ],
        relationship: Some(RelationshipSetup {
            from: PersonId(2),
            to: PersonId(1),
            trust: 0.8,
            attachment: 0.7,
        }),
        intervention: None,
        seed: 42,
    };

    let a = experiment.instantiate().unwrap();
    let b = experiment.instantiate().unwrap();
    assert_eq!(a.world.people.len(), 2);
    assert_eq!(a.world.people, b.world.people);
}


#[test]
fn run_validation_accepts_a_complete_trace() {
    let mut sim = Simulation::new();
    sim.add_person(Person::new(PersonId(1), "Alice"));
    sim.promise_made(PersonId(1), PersonId(1), "hello");
    sim.checkpoint();
    sim.run.validate().unwrap();
}
