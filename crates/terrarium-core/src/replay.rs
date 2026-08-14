use crate::{Action, Event, EventId, Observation, PersonId, SimTime, SnapshotId, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub time: SimTime,
    pub event_cursor: usize,
    pub observation_cursor: usize,
    pub action_cursor: usize,
    pub reason: String,
    pub world: WorldState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub id: u64,
    pub parent_branch_id: Option<u64>,
    pub fork_time: SimTime,
    pub fork_event: Option<EventId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationRecord {
    pub id: u64,
    pub timestamp: SimTime,
    pub observer: Option<PersonId>,
    pub observation: Observation,
    pub caused_by: Vec<EventId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    pub id: u64,
    pub timestamp: SimTime,
    pub actor: PersonId,
    pub action: Action,
    pub caused_by: Vec<EventId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMetadata {
    pub run_id: u64,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub metadata: RunMetadata,
    pub snapshots: Vec<Snapshot>,
    pub events: Vec<Event>,
    pub observations: Vec<ObservationRecord>,
    pub actions: Vec<ActionRecord>,
    pub branch: BranchInfo,
}

impl Run {
    pub fn new(initial: &WorldState, branch: BranchInfo) -> Self {
        let run_id = branch.id;
        Self::new_with_metadata(initial, branch, RunMetadata { run_id, seed: None })
    }

    pub fn new_with_metadata(initial: &WorldState, branch: BranchInfo, metadata: RunMetadata) -> Self {
        Self {
            metadata,
            snapshots: vec![Snapshot {
                id: SnapshotId(0),
                time: initial.time,
                event_cursor: initial.events.len(),
                observation_cursor: 0,
                action_cursor: 0,
                reason: "initial".into(),
                world: initial.clone(),
            }],
            events: initial.events.clone(),
            observations: Vec::new(),
            actions: Vec::new(),
            branch,
        }
    }

    pub fn sync_events(&mut self, world: &WorldState) {
        let cursor = self.events.len().min(world.events.len());
        self.events.extend_from_slice(&world.events[cursor..]);
    }

    pub fn capture(&mut self, world: &WorldState, reason: impl Into<String>) -> SnapshotId {
        self.sync_events(world);
        let id = SnapshotId(self.snapshots.len() as u64);
        self.snapshots.push(Snapshot {
            id,
            time: world.time,
            event_cursor: world.events.len(),
            observation_cursor: self.observations.len(),
            action_cursor: self.actions.len(),
            reason: reason.into(),
            world: world.clone(),
        });
        id
    }

    pub fn capture_if_due(&mut self, world: &WorldState, interval_seconds: u64, reason: impl Into<String>) -> Option<SnapshotId> {
        let elapsed = world.time.0.saturating_sub(self.latest().time.0);
        if elapsed >= interval_seconds { Some(self.capture(world, reason)) } else { None }
    }

    pub fn latest(&self) -> &Snapshot {
        self.snapshots.last().expect("Run always contains initial snapshot")
    }

    pub fn at(&self, time: SimTime) -> Option<&WorldState> {
        self.snapshots.iter().rev().find(|s| s.time <= time).map(|s| &s.world)
    }

    pub fn snapshot(&self, id: SnapshotId) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    pub fn event(&self, id: EventId) -> Option<&Event> { self.events.iter().find(|e| e.id == id) }
    pub fn events(&self) -> impl Iterator<Item = &Event> { self.events.iter() }

    pub fn events_between(&self, start: SimTime, end: SimTime) -> impl Iterator<Item = &Event> {
        self.events.iter().filter(move |e| e.timestamp >= start && e.timestamp <= end)
    }

    pub fn record_observation(&mut self, timestamp: SimTime, observer: Option<PersonId>, observation: Observation, caused_by: impl IntoIterator<Item = EventId>) -> u64 {
        let id = self.observations.len() as u64;
        self.observations.push(ObservationRecord { id, timestamp, observer, observation, caused_by: caused_by.into_iter().collect() });
        id
    }

    pub fn record_action(&mut self, timestamp: SimTime, actor: PersonId, action: Action, caused_by: impl IntoIterator<Item = EventId>) -> u64 {
        let id = self.actions.len() as u64;
        self.actions.push(ActionRecord { id, timestamp, actor, action, caused_by: caused_by.into_iter().collect() });
        id
    }

    pub fn causal_chain(&self, event_id: EventId) -> Vec<&Event> {
        let mut chain = Vec::new();
        let mut current = Some(event_id);
        while let Some(id) = current {
            let Some(event) = self.event(id) else { break };
            current = event.causal_parent;
            chain.push(event);
        }
        chain.reverse();
        chain
    }

    pub fn fork(&self, snapshot_id: SnapshotId, branch: BranchInfo) -> Option<Self> {
        let index = self.snapshots.iter().position(|s| s.id == snapshot_id)?;
        let snapshot = &self.snapshots[index];
        Some(Self {
            metadata: RunMetadata { run_id: branch.id, seed: self.metadata.seed },
            snapshots: self.snapshots[..=index].to_vec(),
            events: self.events[..snapshot.event_cursor.min(self.events.len())].to_vec(),
            observations: self.observations[..snapshot.observation_cursor.min(self.observations.len())].to_vec(),
            actions: self.actions[..snapshot.action_cursor.min(self.actions.len())].to_vec(),
            branch,
        })
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> { serde_json::to_string_pretty(self) }
}
