//! Durable run history, snapshots, replay, branching, and export.
//!
//! A `Simulation` is the thing that evolves. A `Run` is the durable scientific
//! record of that evolution. Keeping the record independent from the running
//! simulation is what lets Observatory inspect a completed experiment without
//! reimplementing the simulation itself.

use crate::{Action, Event, EventId, Observation, SimTime, WorldState};
use serde::{Deserialize, Serialize};

/// A recorded observation crossing the world/agent boundary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObservationRecord {
    pub timestamp: SimTime,
    pub observer: crate::PersonId,
    pub observation: Observation,
    pub source_events: Vec<EventId>,
    /// Stable local ordering for entries that occur at the same simulation time.
    pub sequence: u64,
}

/// A recorded agent decision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionRecord {
    pub timestamp: SimTime,
    pub actor: crate::PersonId,
    pub action: Action,
    pub caused_by: Vec<EventId>,
    pub event_id: Option<EventId>,
    /// Stable local ordering for entries that occur at the same simulation time.
    pub sequence: u64,
}

/// A complete checkpoint from which deterministic state reconstruction can begin.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub time: SimTime,
    pub event_cursor: usize,
    pub observation_cursor: usize,
    pub action_cursor: usize,
    pub world: WorldState,
    pub rng_state: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BranchInfo {
    pub branch_id: String,
    pub parent_branch_id: Option<String>,
    pub fork_event: Option<EventId>,
    pub fork_time: Option<SimTime>,
}

/// One entry in the chronological stream Observatory will consume.
///
/// Events, observations, and actions remain separate internally because they
/// have different semantics. This enum is the read model that combines them
/// for inspection without forcing the simulation to use one overloaded type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TraceEntry {
    Event(Event),
    Observation(ObservationRecord),
    Action(ActionRecord),
}

impl TraceEntry {
    pub fn timestamp(&self) -> SimTime {
        match self {
            Self::Event(event) => event.timestamp,
            Self::Observation(record) => record.timestamp,
            Self::Action(record) => record.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunArtifact {
    /// Bumped when the JSON shape changes incompatibly for Observatory.
    pub schema_version: u32,
    pub run: Run,
    /// A denormalized chronological read model. The canonical data remains in
    /// `run`; this field simply makes the first Observatory implementation easy.
    pub timeline: Vec<TraceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Run {
    pub seed: u64,
    pub branch: BranchInfo,
    pub events: Vec<Event>,
    pub observations: Vec<ObservationRecord>,
    pub actions: Vec<ActionRecord>,
    pub snapshots: Vec<Snapshot>,
    next_trace_sequence: u64,
}

impl Run {
    pub fn new(branch_id: impl Into<String>) -> Self {
        Self {
            seed: 0,
            branch: BranchInfo {
                branch_id: branch_id.into(),
                parent_branch_id: None,
                fork_event: None,
                fork_time: None,
            },
            events: Vec::new(),
            observations: Vec::new(),
            actions: Vec::new(),
            snapshots: Vec::new(),
            next_trace_sequence: 0,
        }
    }

    /// Return the causal ancestry of an event, including the event itself.
    ///
    /// The result is ordered from oldest ancestor to the requested event.
    /// Causality is followed through explicit `causal_parents`; timestamps are
    /// never used to infer causal relationships.
    pub fn causal_chain(&self, event_id: EventId) -> Vec<&Event> {
        let mut chain = Vec::new();
        let mut current = vec![event_id];
        let mut visited = std::collections::HashSet::new();

        while let Some(id) = current.pop() {
            if !visited.insert(id) {
                continue;                                    }

            if let Some(event) = self.event(id) {
                for parent in &event.causal_parents {
                    current.push(*parent);
                }

                chain.push(event);
            }
        }

        chain.sort_by_key(|event| event.trace_sequence);
        chain
    }

    pub fn record_event(&mut self, mut event: Event) {
        event.trace_sequence = self.next_trace_sequence;
        self.next_trace_sequence += 1;
        self.events.push(event);
    }

    pub fn record_observation(
        &mut self,
        timestamp: SimTime,
        observer: crate::PersonId,
        observation: Observation,
        source_events: Vec<EventId>,
    ) {
        let sequence = self.next_trace_sequence;
        self.next_trace_sequence += 1;
        self.observations.push(ObservationRecord {
            timestamp,
            observer,
            observation,
            source_events,
            sequence,
        });
    }

    pub fn record_action(
        &mut self,
        timestamp: SimTime,
        actor: crate::PersonId,
        action: Action,
        caused_by: Vec<EventId>,
        event_id: Option<EventId>,
    ) {
        let sequence = self.next_trace_sequence;
        self.next_trace_sequence += 1;
        self.actions.push(ActionRecord {
            timestamp,
            actor,
            action,
            caused_by,
            event_id,
            sequence,
        });
    }

    pub fn capture(&mut self, world: &WorldState, rng_state: u64) {
        self.snapshots.push(Snapshot {
            time: world.time,
            event_cursor: self.events.len(),
            observation_cursor: self.observations.len(),
            action_cursor: self.actions.len(),
            world: world.clone(),
            rng_state,
        });
    }

    pub fn event(&self, id: EventId) -> Option<&Event> {
        self.events.iter().find(|event| event.id == id)
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    pub fn latest(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }

    /// Produce the unified chronological read model used by Observatory.
    ///
    /// Events are ordered first at a timestamp because they change the world;
    /// observations follow because they are derived from that world; actions
    /// follow observations because they are decisions made from them.
    pub fn timeline(&self) -> Vec<TraceEntry> {
        let mut entries = Vec::with_capacity(
            self.events.len() + self.observations.len() + self.actions.len(),
        );

        entries.extend(self.events.iter().cloned().map(TraceEntry::Event));
        entries.extend(
            self.observations
                .iter()
                .cloned()
                .map(TraceEntry::Observation),
        );
        entries.extend(self.actions.iter().cloned().map(TraceEntry::Action));

        entries.sort_by_key(|entry| match entry {
            TraceEntry::Event(event) => (event.timestamp, event.trace_sequence),
            TraceEntry::Observation(record) => (record.timestamp, record.sequence),
            TraceEntry::Action(record) => (record.timestamp, record.sequence),
        });

        entries
    }

    /// Reconstruct state at an exact event cursor.
    pub fn replay_to_cursor(&self, cursor: usize) -> Option<WorldState> {
        let cursor = cursor.min(self.events.len());
        let snapshot = self
            .snapshots
            .iter()
            .filter(|snapshot| snapshot.event_cursor <= cursor)
            .max_by_key(|snapshot| snapshot.event_cursor)?;

        let mut world = snapshot.world.clone();
        for event in self
            .events
            .iter()
            .skip(snapshot.event_cursor)
            .take(cursor.saturating_sub(snapshot.event_cursor))
        {
            for effect in &event.effects {
                world.apply_effect(effect);
            }
            world.events.push(event.clone());
        }

        if let Some(event) = self.events.get(cursor.saturating_sub(1)) {
            world.time = event.timestamp;
        } else {
            world.time = snapshot.time;
        }
        world.sync_next_event_id();

        Some(world)
    }

    /// Reconstruct state at a simulation time. Events at exactly `time` are
    /// included, matching the semantics of `Simulation::advance`.
    pub fn replay_to(&self, time: SimTime) -> Option<WorldState> {
        let snapshot = self
            .snapshots
            .iter()
            .filter(|snapshot| snapshot.time <= time)
            .max_by_key(|snapshot| snapshot.event_cursor)?;

        let mut world = snapshot.world.clone();
        for event in self.events.iter().skip(snapshot.event_cursor) {
            if event.timestamp > time {
                break;
            }
            for effect in &event.effects {
                world.apply_effect(effect);
            }
            world.events.push(event.clone());
        }

        world.time = time;
        world.sync_next_event_id();
        Some(world)
    }

    pub fn at(&self, time: SimTime) -> Option<WorldState> {
        self.replay_to(time)
    }

    /// Fork history at an exact event cursor. Future observations/actions and
    /// future snapshots are excluded. The child is intentionally only a history
    /// object; Simulation reconstructs the corresponding world and RNG state.
    pub fn branch(&self, branch_id: impl Into<String>, event_cursor: usize) -> Self {
        let cursor = event_cursor.min(self.events.len());
        let fork_event = cursor.checked_sub(1).and_then(|i| self.events.get(i));
        let retained_ids: std::collections::HashSet<EventId> =
            self.events[..cursor].iter().map(|event| event.id).collect();

        let mut child = Self {
            seed: self.seed,
            branch: BranchInfo {
                branch_id: branch_id.into(),
                parent_branch_id: Some(self.branch.branch_id.clone()),
                fork_event: fork_event.map(|event| event.id),
                fork_time: fork_event.map(|event| event.timestamp),
            },
            events: self.events[..cursor].to_vec(),
            observations: Vec::new(),
            actions: Vec::new(),
            snapshots: self
                .snapshots
                .iter()
                .filter(|snapshot| snapshot.event_cursor <= cursor)
                .cloned()
                .collect(),
            next_trace_sequence: 0,
        };

        child.observations = self
            .observations
            .iter()
            .filter(|record| record.source_events.iter().all(|id| retained_ids.contains(id)))
            .cloned()
            .collect();
        child.actions = self
            .actions
            .iter()
            .filter(|record| {
                record
                    .event_id
                    .map(|id| retained_ids.contains(&id))
                    .unwrap_or_else(|| record.caused_by.iter().all(|id| retained_ids.contains(id)))
            })
            .cloned()
            .collect();

        child.next_trace_sequence = child
            .events
            .iter()
            .map(|event| event.trace_sequence)
            .chain(child.observations.iter().map(|record| record.sequence))
            .chain(child.actions.iter().map(|record| record.sequence))
            .max()
            .map(|sequence| sequence + 1)
            .unwrap_or(0);

        child
    }

    /// Check the invariants that make a run safe to hand to Observatory.
    ///
    /// This is intentionally a lightweight structural validator. Scientific
    /// validity is a higher-level concern, but broken cursors or dangling causal
    /// references should never reach the visualization layer.
    pub fn validate(&self) -> Result<(), String> {
        let mut ids = std::collections::HashSet::new();
        let mut previous_id = None;
        for event in &self.events {
            if !ids.insert(event.id) {
                return Err(format!("duplicate event id {:?}", event.id));
            }
            if let Some(previous) = previous_id {
                if event.id <= previous {
                    return Err("event ids are not strictly increasing".into());
                }
            }
            previous_id = Some(event.id);
            for parent in &event.causal_parents {
                if !ids.contains(parent) {
                    return Err(format!("event {:?} has dangling parent {:?}", event.id, parent));
                }
            }
        }

        for snapshot in &self.snapshots {
            if snapshot.event_cursor > self.events.len()
                || snapshot.observation_cursor > self.observations.len()
                || snapshot.action_cursor > self.actions.len()
            {
                return Err("snapshot cursor exceeds run length".into());
            }
            if snapshot.world.events.len() != snapshot.event_cursor {
                return Err("snapshot world/event cursor mismatch".into());
            }
        }

        for observation in &self.observations {
            for source in &observation.source_events {
                if !ids.contains(source) {
                    return Err(format!("observation references unknown event {:?}", source));
                }
            }
        }

        for action in &self.actions {
            for source in &action.caused_by {
                if !ids.contains(source) {
                    return Err(format!("action references unknown event {:?}", source));
                }
            }
            if let Some(event_id) = action.event_id {
                if !ids.contains(&event_id) {
                    return Err(format!("action references unknown result event {:?}", event_id));
                }
            }
        }

        let timeline = self.timeline();
        for pair in timeline.windows(2) {
            if pair[0].timestamp() > pair[1].timestamp() {
                return Err("timeline is not chronological".into());
            }
        }

        Ok(())
    }

    pub fn artifact(&self) -> RunArtifact {
        RunArtifact {
            schema_version: 1,
            run: self.clone(),
            timeline: self.timeline(),
        }
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.artifact())
    }
}
