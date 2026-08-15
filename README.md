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
