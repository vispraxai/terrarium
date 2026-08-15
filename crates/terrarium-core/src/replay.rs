//! Persistent run history, snapshots, replay, and counterfactual branches.
//!
//! A `Simulation` is the thing that evolves. A `Run` is the historical record
//! of that evolution. Keeping those concepts separate prevents the future UI
//! from becoming coupled to simulation internals.

use crate::{Event, EventId, SimTime, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub time: SimTime,
    /// Cursors are more precise than timestamps: two events may happen at the
    /// same simulated second, but they still have a definite order.
    pub event_cursor: usize,
    pub world: WorldState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub branch_id: String,
    pub parent_branch_id: Option<String>,
    pub fork_event: Option<EventId>,
    pub fork_time: Option<SimTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub branch: BranchInfo,
    pub events: Vec<Event>,
    pub snapshots: Vec<Snapshot>,
}

impl Run {
    pub fn new(branch_id: impl Into<String>) -> Self {
        Self {
            branch: BranchInfo {
                branch_id: branch_id.into(),
                parent_branch_id: None,
                fork_event: None,
                fork_time: None,
            },
            events: Vec::new(),
            snapshots: Vec::new(),
        }
    }

    pub fn record_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn capture(&mut self, world: &WorldState) {
        self.snapshots.push(Snapshot {
            time: world.time,
            event_cursor: self.events.len(),
            world: world.clone(),
        });
    }

    pub fn event(&self, id: EventId) -> Option<&Event> {
        self.events.iter().find(|event| event.id == id)
    }

    /// Return the causal ancestry from oldest parent to the selected event.
    ///
    /// Phase 0 currently follows the first parent. The representation already
    /// supports multiple parents, so the traversal can become a graph walk
    /// when richer causal inference is introduced.
    pub fn causal_chain(&self, id: EventId) -> Vec<&Event> {
        let mut chain = Vec::new();
        let mut current = Some(id);
        while let Some(id) = current {
            let Some(event) = self.event(id) else { break };
            current = event.causal_parents.first().copied();
            chain.push(event);
        }
        chain.reverse();
        chain
    }

    /// Reconstruct the world at a requested simulation time.
    ///
    /// We start from the latest checkpoint at or before the target and then
    /// apply recorded effects. This makes replay deterministic and avoids
    /// rerunning the whole simulation from time zero.
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
        }
        world.time = time;
        Some(world)
    }

    /// Fork at an exact event position, not merely a timestamp.
    pub fn branch(&self, branch_id: impl Into<String>, event_cursor: usize) -> Self {
        let cursor = event_cursor.min(self.events.len());
        let fork_event = cursor.checked_sub(1).and_then(|i| self.events.get(i));

        Self {
            branch: BranchInfo {
                branch_id: branch_id.into(),
                parent_branch_id: Some(self.branch.branch_id.clone()),
                fork_event: fork_event.map(|event| event.id),
                fork_time: fork_event.map(|event| event.timestamp),
            },
            events: self.events[..cursor].to_vec(),
            snapshots: self
                .snapshots
                .iter()
                .filter(|snapshot| snapshot.event_cursor <= cursor)
                .cloned()
                .collect(),
        }
    }
}
