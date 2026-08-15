# Terrarium

Terrarium is the developmental synthetic-world platform described in the Visprax specification.

This iteration makes the Phase 0 simulation history more rigorous before we build the Observatory UI.

## What is new

The core now treats meaningful transitions as explicit, replayable state changes:

- semantic world events
- explicit state effects
- observations and agent actions as first-class history records
- deterministic snapshots
- replay from a snapshot
- exact event cursors for branching
- causal parent links
- JSON-serializable run history

The guiding rule is:

> An event says **what happened**. Effects say **what changed because it happened**.

That separation is important for the eventual Observatory: it lets us inspect a timeline, reconstruct state, distinguish latent truth from observations, and follow consequences without asking the UI to infer anything from the current world state.

## Architecture

```text
                         EVENT
                           |
                 +---------+---------+
                 |                   |
                 v                   v
            STATE EFFECTS       OBSERVATION
                 |                   |
                 v                   v
             WORLD STATE           AGENT
                 |                   |
                 +---------+---------+
                           |
                           v
                         ACTION
                           |
                           v
                         EVENT
```

A run is a durable history:

```text
Run
├── events[]
│   ├── semantic kind
│   ├── timestamp
│   ├── causal parents
│   ├── visibility
│   └── effects[]
├── observations[]
├── actions[]
└── snapshots[]
    ├── simulation time
    ├── event cursor
    ├── observation cursor
    └── action cursor
```

## Why explicit effects?

The earlier Phase 0 implementation changed psychological state directly inside `Simulation::promise_broken()`. That is convenient, but it makes historical reconstruction difficult: an event exists, but the exact state mutation is implicit in arbitrary code.

Now the intended flow is:

```text
simulation rule
      |
      +--> construct Event + StateEffects
      |
      +--> apply effects to WorldState
      |
      +--> append Event to Run history
```

A replay therefore does not rerun arbitrary simulation code. It starts from a known snapshot and applies the recorded effects in order.

## Replay

Snapshots store **cursors**, not just timestamps. Multiple events may share the same simulation time, so a timestamp cannot uniquely identify a historical position.

```text
snapshot @ event 20
        |
        +-- event 21
        +-- event 22
        +-- event 23
        |
        v
    reconstructed state
```

## Branching

A branch is a counterfactual continuation of a run. It shares the history up to an exact event cursor and then diverges.

```text
                     event 100
                         |
               +---------+---------+
               |                   |
            baseline          counterfactual
               |                   |
        Alice leaves          Alice stays
               |                   |
              ...                 ...
```

This is deliberately different from a Git branch: it is a branch of a simulated world.

## Source-reading order

The code is intentionally commented because the architecture is becoming more subtle. If you are learning it, read in this order:

1. `event.rs` — what counts as something that happened.
2. `effect.rs` — what can change in authoritative state.
3. `world.rs` — how effects become state.
4. `replay.rs` — how history is captured and reconstructed.
5. `simulation.rs` — how domain rules create events/effects.
6. `agent.rs` — the latent-world/agent boundary.

## Validation

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo run -p terrarium-cli
```

## Next: Observatory

Once this layer is stable, the Observatory can consume the run history without owning simulation logic:

```text
Rust Terrarium
      |
      +---- event/run data ----> Observatory backend ----> browser UI
```

The UI will then be able to show the event stream against time, latent truth vs observations, state deltas, causal chains, replay position, and counterfactual branches.


## Pre-Observatory Core

The current implementation is moving toward a complete, replayable simulation
trace before adding a graphical Observatory.

The core loop is:

```text
latent world
    ↓
semantic event
    ↓
explicit state effects
    ↓
world state
    ↓
observation boundary
    ↓
agent
    ↓
action
    ↓
world event
```

### Event history

Every event contains:

- simulation timestamp
- causal parent ids
- semantic event kind
- visibility
- explicit state effects

An effect is the deterministic state delta needed to reproduce a transition.

### Observations and actions

Observations are deliberately separated from latent events. An event can be
`Public`, `Latent`, or visible only to selected people.

`Simulation::step_agent()` demonstrates the complete Phase 0 loop:

```text
observation
    → agent
    → action
    → action event
```

The `Run` records observations and actions alongside world events.

### Deterministic replay

Snapshots contain:

- world state
- event cursor
- observation cursor
- action cursor
- deterministic RNG state

Replay begins at the nearest snapshot and applies explicit effects in order.

### Counterfactual branches

A branch is created at an exact event cursor. The child world is reconstructed
at that cursor rather than cloning the parent's current state. This avoids the
subtle bug where a branch's history is truncated but its world still contains
future state.

### Deterministic randomness

`Simulation::with_seed(seed)` establishes reproducible pseudo-random state.
This is simulation randomness, not cryptographic randomness.

### Declarative experiments

`crates/terrarium-core/src/experiment.rs` defines the first typed experiment
schema. `experiments/promise.yaml` remains the human-readable seed for the
future declarative experiment loader.

### Source-reading order

If the code feels increasingly complicated, read it in this order:

1. `event.rs` — what can happen and who can see it.
2. `effect.rs` — what can change.
3. `person.rs` — persistent psychological state.
4. `world.rs` — where effects become authoritative state.
5. `replay.rs` — how history is stored and reconstructed.
6. `simulation.rs` — how time, scheduling, observations, and agents connect.
7. `agent.rs` — the world/agent boundary.
8. `experiment.rs` — how experiments will eventually become declarative.

The comments in these files explain the design decisions as well as the code.

### Validation

From the repository root:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo run -p terrarium-cli
```

The CLI writes `terrarium-run.json`, which is intended to become the first
data contract consumed by Observatory.

### Current limitations

This is still Phase 0.

The observation model is textual and event-based, the scheduler is simple, and
the deterministic RNG is only plumbing for future stochastic dynamics. A full
sensor model, richer causal graph traversal, developmental epochs, and physical
environments remain later phases.

### Core APIs added in this milestone

- `Simulation::schedule_event()` and scheduled-event processing in `advance()`.
- `Simulation::observe()` for a first event-based observation boundary.
- `Simulation::step_agent()` for observation → agent → action → world-event closure.
- `Simulation::with_seed()` and deterministic RNG state in snapshots.
- `Simulation::enter_room()`, `leave_room()`, `set_belief()`, and `set_affect()` as explicit effect-producing mutations.
- `Simulation::branch()` now reconstructs the child world at the fork cursor.

## Stable run artifact

`Run::to_json_pretty()` emits a versioned `RunArtifact` containing both the
canonical run data and a chronological `timeline` read model. The Observatory
can consume this artifact without linking against the Rust simulation crate.

The canonical data remains normalized (`events`, `observations`, `actions`,
and `snapshots`); `timeline` is intentionally denormalized for visualization.

## Experiment API

`Experiment::instantiate()` builds a deterministic initial world from a typed
experiment definition. `Experiment::run_duration()` advances the declared
simulation duration. Intervention semantics remain explicit rather than being
silently inferred from a duration string.

The YAML file under `experiments/` is still a human-readable seed. A parser for
that YAML format is intentionally deferred until the schema stabilizes; the
core API accepts the typed `Experiment` directly.
