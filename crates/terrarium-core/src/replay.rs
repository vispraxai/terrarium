use crate::{Event, EventId, SimTime, SnapshotId, WorldState};
use serde::{Deserialize, Serialize};

/// A full immutable copy of world truth at a particular simulation time.
/// Snapshots are intentionally simple in Phase 0: correctness and inspectability
/// matter more than storage efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub time: SimTime,
    pub event_cursor: usize,
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
pub struct Run {
    pub snapshots: Vec<Snapshot>,
    pub branch: BranchInfo,
}

impl Run {
    pub fn new(initial: &WorldState, branch: BranchInfo) -> Self {
        Self {
            snapshots: vec![Snapshot {
                id: SnapshotId(0),
                time: initial.time,
                event_cursor: initial.events.len(),
                reason: "initial".into(),
                world: initial.clone(),
            }],
            branch,
        }
    }

    pub fn capture(&mut self, world: &WorldState, reason: impl Into<String>) -> SnapshotId {
        let id = SnapshotId(self.snapshots.len() as u64);
        self.snapshots.push(Snapshot {
            id,
            time: world.time,
            event_cursor: world.events.len(),
            reason: reason.into(),
            world: world.clone(),
        });
        id
    }

    pub fn latest(&self) -> &Snapshot {
        self.snapshots.last().expect("Run always contains initial snapshot")
    }

    /// Historical state at the latest captured snapshot at or before `time`.
    /// This is deliberately snapshot-based: it never mutates the live simulation.
    pub fn at(&self, time: SimTime) -> Option<&WorldState> {
        self.snapshots
            .iter()
            .rev()
            .find(|snapshot| snapshot.time <= time)
            .map(|snapshot| &snapshot.world)
    }

    pub fn snapshot(&self, id: SnapshotId) -> Option<&Snapshot> {
        self.snapshots.iter().find(|snapshot| snapshot.id == id)
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.snapshots
            .last()
            .into_iter()
            .flat_map(|snapshot| snapshot.world.events.iter())
    }

    pub fn causal_chain(&self, event_id: EventId) -> Vec<&Event> {
        let Some(world) = self.snapshots.last().map(|s| &s.world) else {
            return Vec::new();
        };
        let mut chain = Vec::new();
        let mut current = Some(event_id);
        while let Some(id) = current {
            let Some(event) = world.event(id) else { break };
            current = event.causal_parent;
            chain.push(event);
        }
        chain.reverse();
        chain
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
