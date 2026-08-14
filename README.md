# Terrarium

Terrarium is the developmental synthetic-world platform described in the Visprax specification.

This repository is intentionally **Phase 0**: a small textual world rather than a 3D simulator.

The first implementation establishes the architectural boundary:

```text
WORLD TRUTH
    ↓
sensor / observation boundary
    ↓
VIXIR / AGENT
    ↓
ACTION
    ↓
WORLD
```

## What exists

- deterministic simulation time
- authoritative `WorldState`
- persistent simulated people
- beliefs, goals, memories, expectations, affect, relationships and self-model
- semantic world events
- causal parent links between events
- agent/observation boundary
- agent actions
- simple psychological consequences
- cloning-based simulation branching
- a tiny executable experiment

## Run

Install Rust, then:

```bash
cargo run -p terrarium-cli
```

You should see a small experiment involving a promise, a broken expectation, a relationship update, an observation delivered to an agent, and two simulation branches.

## Inspect

Start with:

```text
crates/terrarium-core/src/world.rs
crates/terrarium-core/src/event.rs
crates/terrarium-core/src/person.rs
crates/terrarium-core/src/agent.rs
crates/terrarium-core/src/simulation.rs
crates/terrarium-cli/src/main.rs
```

The important thing is not the amount of code. It is the separation of:

1. authoritative latent world state
2. events / causal history
3. observations
4. agent actions
5. resulting world changes

## Deliberate limitations

This is not yet:

- a realistic psychology simulator
- a multimodal environment
- a 3D engine
- a Vixir implementation
- a sophisticated causal graph
- a scientifically calibrated human model

Those should come later.

The next useful step is to make the Phase 0 model more rigorous: event scheduling, snapshots, proper causal branching, observation generation, and experiment definitions.
