//! The simulation lifecycle.
//!
//! `WorldState` owns latent truth and `Run` owns history. `Simulation` is the
//! coordinator: it advances time, executes scheduled events, creates semantic
//! events/effects, exposes observations, and connects external agents back to
//! the world.

use crate::agent::{Action, Agent, Observation};
use crate::effect::StateEffect;
use crate::event::{EventKind, Visibility};
use crate::{Duration, EventId, Person, PersonId, Run, SimTime, WorldState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScheduledEvent {
    timestamp: SimTime,
    kind: EventKind,
    effects: Vec<StateEffect>,
    visibility: Visibility,
}

/// A deliberately small deterministic PRNG.
///
/// Terrarium needs reproducibility, not cryptographic randomness. The state is
/// therefore a single u64 and is captured in every snapshot. If stochastic
/// dynamics become scientifically important later, this abstraction can be
/// replaced with a better generator without changing the rest of the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn state(&self) -> u64 {
        self.state
    }

    fn set_state(&mut self, state: u64) {
        self.state = if state == 0 { 0x9E3779B97F4A7C15 } else { state };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub world: WorldState,
    pub run: Run,
    seed: u64,
    rng: DeterministicRng,
    /// Observation cursors are runtime delivery state, not latent world state.
    /// They are reset when a branch is created.
    observation_cursors: HashMap<PersonId, usize>,
    scheduled: Vec<ScheduledEvent>,
}

impl Simulation {
    pub fn new() -> Self {
        Self::with_seed(0)
    }

    pub fn with_seed(seed: u64) -> Self {
        let world = WorldState::new();
        let mut run = Run::new("main");
        let rng = DeterministicRng::new(seed);
        run.seed = seed;
        run.capture(&world, rng.state());

        Self {
            world,
            run,
            seed,
            rng,
            observation_cursors: HashMap::new(),
            scheduled: Vec::new(),
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn time(&self) -> SimTime {
        self.world.time
    }

    /// Capture a deterministic checkpoint containing both world state and RNG
    /// state. Without the latter, replay could diverge after a random choice.
    pub fn checkpoint(&mut self) {
        self.run.capture(&self.world, self.rng.state());
    }

    /// Advance simulation time and execute every scheduled event at or before
    /// the target. Same-time events preserve insertion order.
    pub fn advance(&mut self, duration: Duration) {
        let target = self.world.time + duration;

        while let Some(index) = self.next_scheduled_index(target) {
            let scheduled = self.scheduled.remove(index);
            self.world.time = scheduled.timestamp;
            self.emit_with_visibility(
                scheduled.kind,
                scheduled.effects,
                scheduled.visibility,
            );
        }

        self.world.time = target;
    }

    fn next_scheduled_index(&self, target: SimTime) -> Option<usize> {
        self.scheduled
            .iter()
            .enumerate()
            .filter(|(_, event)| event.timestamp <= target)
            .min_by_key(|(index, event)| (event.timestamp, *index))
            .map(|(index, _)| index)
    }

    /// Schedule a semantic event without executing it immediately.
    pub fn schedule_event(
        &mut self,
        timestamp: SimTime,
        kind: EventKind,
        effects: Vec<StateEffect>,
        visibility: Visibility,
    ) {
        assert!(
            timestamp >= self.world.time,
            "scheduled event cannot be placed before current simulation time"
        );
        self.scheduled.push(ScheduledEvent {
            timestamp,
            kind,
            effects,
            visibility,
        });
    }

    pub fn add_person(&mut self, person: Person) {
        self.world.add_person(person);
        // Person creation is setup state in Phase 0. Checkpointing here means
        // replay begins with the populated world rather than an empty one.
        self.checkpoint();
    }

    pub fn enter_room(&mut self, person: PersonId, room: impl Into<String>) -> EventId {
        let room = room.into();
        self.emit(
            EventKind::PersonEnteredRoom {
                person,
                room: room.clone(),
            },
            vec![StateEffect::PersonEnteredRoom { person, room }],
        )
    }

    pub fn leave_room(&mut self, person: PersonId, room: impl Into<String>) -> EventId {
        let room = room.into();
        self.emit(
            EventKind::PersonLeftRoom {
                person,
                room: room.clone(),
            },
            vec![StateEffect::PersonLeftRoom { person, room }],
        )
    }

    pub fn set_belief(
        &mut self,
        person: PersonId,
        proposition: impl Into<String>,
        confidence: f32,
    ) -> EventId {
        let proposition = proposition.into();
        let old = self
            .world
            .people
            .get(&person)
            .and_then(|p| {
                p.beliefs
                    .beliefs
                    .iter()
                    .find(|belief| belief.proposition == proposition)
                    .map(|belief| belief.confidence)
            })
            .unwrap_or(0.0);

        let new_confidence = confidence.clamp(0.0, 1.0);
        self.emit(
            EventKind::Custom {
                description: format!("{person:?} belief changed: {proposition}"),
            },
            vec![StateEffect::BeliefChanged {
                person,
                proposition,
                old_confidence: old,
                new_confidence,
            }],
        )
    }

    pub fn set_affect(&mut self, person: PersonId, valence: f32, arousal: f32) -> EventId {
        let (old_valence, old_arousal) = self
            .world
            .people
            .get(&person)
            .map(|p| (p.affect.valence, p.affect.arousal))
            .unwrap_or((0.0, 0.0));

        self.emit(
            EventKind::AffectChanged { person },
            vec![StateEffect::AffectChanged {
                person,
                old_valence,
                new_valence: valence,
                old_arousal,
                new_arousal: arousal,
            }],
        )
    }

    pub fn promise_made(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) -> EventId {
        self.emit(
            EventKind::PromiseMade {
                from,
                to,
                content: content.into(),
            },
            Vec::new(),
        )
    }

    /// Psychological consequences are explicit effects, so replay does not
    /// call this domain method again.
    pub fn promise_broken(
        &mut self,
        from: PersonId,
        to: PersonId,
        content: impl Into<String>,
    ) -> EventId {
        let content = content.into();
        let description = format!("{} broke a promise: {}", from.0, content);

        self.emit(
            EventKind::PromiseBroken {
                from,
                to,
                content,
            },
            vec![
                StateEffect::MemoryAdded {
                    person: to,
                    timestamp: self.world.time,
                    description,
                    salience: 0.8,
                },
                StateEffect::RelationshipChanged {
                    observer: to,
                    target: from,
                    trust_delta: -0.15,
                    conflict_delta: 0.10,
                    uncertainty_delta: 0.10,
                },
            ],
        )
    }

    fn emit(&mut self, kind: EventKind, effects: Vec<StateEffect>) -> EventId {
        self.emit_with_visibility(kind, effects, Visibility::Public)
    }

    fn emit_with_visibility(
        &mut self,
        kind: EventKind,
        effects: Vec<StateEffect>,
        visibility: Visibility,
    ) -> EventId {
        let id = self
            .world
            .emit_with_visibility(kind, effects, visibility);
        let event = self
            .world
            .events
            .last()
            .cloned()
            .expect("emit_with_visibility always appends an event");
        debug_assert_eq!(event.id, id);
        self.run.record_event(event);
        // Keep the world copy and durable-run copy aligned for snapshots.
        if let (Some(world_event), Some(run_event)) =
            (self.world.events.last_mut(), self.run.events.last())
        {
            world_event.trace_sequence = run_event.trace_sequence;
        }
        id
    }

    /// Generate the current event-based observation for one person.
    ///
    /// The cursor advances through the latent event stream even when events
    /// are hidden. This prevents an invisible event from being reconsidered on
    /// every observation call.
    pub fn observe(&mut self, observer: PersonId) -> Option<Observation> {
        if !self.world.people.contains_key(&observer) {
            return None;
        }

        let cursor = *self.observation_cursors.get(&observer).unwrap_or(&0);
        let visible: Vec<_> = self
            .world
            .events
            .iter()
            .skip(cursor)
            .filter(|event| event.visibility.visible_to(observer))
            .collect();

        self.observation_cursors
            .insert(observer, self.world.events.len());

        let source_events = visible.iter().map(|event| event.id).collect::<Vec<_>>();
        let text = if visible.is_empty() {
            "Nothing new is observable.".to_string()
        } else {
            visible
                .iter()
                .map(|event| format!("{:?}", event.kind))
                .collect::<Vec<_>>()
                .join("\n")
        };

        Some(Observation::Text {
            timestamp: self.world.time,
            observer,
            text,
            source_events,
        })
    }

    /// Run one closed-loop agent step:
    /// latent world -> observation -> agent -> action -> world event.
    pub fn step_agent<A: Agent>(
        &mut self,
        actor: PersonId,
        agent: &mut A,
    ) -> Result<Action, crate::world::WorldError> {
        let observation = self
            .observe(actor)
            .ok_or(crate::world::WorldError::UnknownPerson(actor))?;
        let source_events = match &observation {
            Observation::Text { source_events, .. } => source_events.clone(),
        };

        self.run.record_observation(
            self.world.time,
            actor,
            observation.clone(),
            source_events.clone(),
        );

        let action = agent.observe(observation);
        let event_id = self
            .world
            .apply_agent_action_with_parents(actor, &action, source_events.clone())?;
        let event = self
            .world
            .events
            .last()
            .cloned()
            .expect("agent action appends an event");

        debug_assert_eq!(event.id, event_id);
        self.run.record_event(event);
        if let (Some(world_event), Some(run_event)) =
            (self.world.events.last_mut(), self.run.events.last())
        {
            world_event.trace_sequence = run_event.trace_sequence;
        }
        self.run.record_action(
            self.world.time,
            actor,
            action.clone(),
            source_events,
            Some(event_id),
        );

        Ok(action)
    }

    /// Draw deterministic randomness. Callers that need replayable branching
    /// should checkpoint after stochastic transitions; ordinary snapshots still
    /// capture the current generator state.
    pub fn random_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    /// Create a counterfactual from an exact event cursor.
    ///
    /// The child world is reconstructed from history rather than copied from
    /// the parent's current state. The checked form additionally requires an
    /// RNG checkpoint at the fork cursor, because a deterministic counterfactual
    /// is not valid if the random trajectory between the checkpoint and fork is
    /// unknown.
    pub fn try_branch(
        &self,
        branch_id: impl Into<String>,
        event_cursor: usize,
    ) -> Result<Self, String> {
        let cursor = event_cursor.min(self.run.events.len());
        let child_run = self.run.branch(branch_id, cursor);
        let child_world = self
            .run
            .replay_to_cursor(cursor)
            .ok_or_else(|| "no snapshot exists from which to replay this cursor".to_string())?;

        let snapshot = child_run
            .snapshots
            .iter()
            .find(|snapshot| snapshot.event_cursor == cursor)
            .ok_or_else(|| {
                format!(
                    "branch cursor {cursor} has no RNG checkpoint; call checkpoint() at the intervention boundary first"
                )
            })?;

        let mut rng = DeterministicRng::new(self.seed);
        rng.set_state(snapshot.rng_state);

        Ok(Self {
            world: child_world,
            run: child_run,
            seed: self.seed,
            rng,
            observation_cursors: HashMap::new(),
            scheduled: Vec::new(),
        })
    }

    /// Convenience wrapper for callers that know they checkpoint at the fork.
    /// Research/experiment code should prefer `try_branch` so an invalid RNG
    /// boundary becomes an explicit error rather than silent nondeterminism.
    pub fn branch(&self, branch_id: impl Into<String>, event_cursor: usize) -> Self {
        self.try_branch(branch_id, event_cursor)
            .expect("counterfactual branch requires a snapshot at the exact fork cursor")
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}
