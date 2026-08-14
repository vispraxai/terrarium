# Terrarium — Development Phases

The development strategy is deliberately incremental. Terrarium should first establish its causal and scientific substrate, then progressively add richer sensory and physical realizations.

## Phase 0 — Textual World

```text
world
 ↓
text observations
 ↓
Vixir
 ↓
text actions
 ↓
world
```

Implement:

- persistent people
- relationships
- goals
- memories
- latent psychological state
- events
- time
- ground truth
- counterfactual branching

The first implementation should be small enough to inspect and reason about directly.

A minimal experiment should demonstrate:

```text
world state
    ↓
event
    ↓
psychological consequence
    ↓
observation
    ↓
agent action
    ↓
world consequence
```

## Phase 1 — Audio

Add:

- speech
- background noise
- multiple speakers
- speaker identity
- silence
- temporal audio streams

The world model should remain unchanged; audio is a richer realization of the observation boundary.

## Phase 2 — Vision

Add:

- camera
- faces
- objects
- motion
- partial visibility
- spatial relationships

Again, Vixir receives observations rather than semantic world truth.

## Phase 3 — Ambient Multimodality

Combine:

```text
audio + video + text + screen + device state
```

as asynchronous streams.

Vixir must handle:

- temporal alignment
- synchronization
- identity tracking
- multimodal fusion
- attention
- salience
- missing data
- uncertainty

## Phase 4 — Physical Environments

Integrate external physical/3D environments through adapters.

Potential foundations include:

- Habitat-Sim
- AI2-THOR
- iGibson / BEHAVIOR
- VirtualHome

Only build custom physical simulation where existing environments fail to satisfy Terrarium's needs.

The core Terrarium model should not become permanently coupled to a single environment engine.

## Developmental Time

Across the phases, Terrarium must support multiple timescales:

```text
milliseconds → sensory streams
seconds       → perception
minutes       → interactions
hours         → routines
days          → memory consolidation
weeks         → relationships
months        → development
years         → structural evolution
```

Not everything should be accelerated equally.

Near-real-time:

- speech
- facial expression
- motion
- audio

Compressible:

- sleep
- travel
- repetitive routines
- low-information periods

Highly accelerated:

- long-term relationship evolution
- developmental changes
- repeated learning

## Developmental Epochs

A possible curriculum:

```text
Epoch 0
basic perception

Epoch 1
simple interaction

Epoch 2
persistent individuals

Epoch 3
long-term relationships

Epoch 4
multiple relationships

Epoch 5
conflict / misunderstanding

Epoch 6
novel environments

Epoch 7
changing goals

Epoch 8
long periods without interaction

Epoch 9
unexpected events

Epoch 10
architectural challenges
```

Eventually, the developmental controller itself could decide when an epoch has been mastered.

## Counterfactuals and Interventions

Terrarium's explicit ground truth allows controlled interventions.

Example:

```text
World A:
person keeps a promise

World B:
person breaks the promise
```

Everything else can remain identical.

Potential interventions:

- promise kept/broken
- interaction occurs/doesn't occur
- person leaves/returns
- memory available/unavailable
- environment changes
- relationship conflict
- unexpected event
- social reward
- social rejection
- surprise

This creates experiments that are almost impossible to perform cleanly in real life.

## Counterfactual Branching

```text
                  World(t)
                     │
              ┌──────┴──────┐
              ↓             ↓
         intervention A   intervention B
              ↓             ↓
           World A        World B
              ↓             ↓
           Vixir A        Vixir B
```

This can test questions such as:

- Did Vixir update its relationship model appropriately?
- Did prediction error alter future expectations?
- Did the internal state change persist?
- Did the system retrieve the correct memories?
- Did structural organization change?

## Research Experiment Families

### Memory

- sparse repetition
- long delays
- conflicting memories
- salient memories
- consolidation

### Identity

- ambiguous speakers
- multiple users
- changing appearance
- long separation
- reunion

### Relationships

- trust formation
- conflict
- reconciliation
- absence
- asymmetric relationships
- changing social contexts

### Affect

- expectation violation
- reward
- loss
- uncertainty
- social rejection
- surprise

### Structural Plasticity

- role emergence
- role pruning
- role merging
- role splitting
- specialization
- catastrophic forgetting resistance

### Self-Model

- self-prediction
- uncertainty awareness
- capability estimation
- self-model updating

## Evaluation

Terrarium should evaluate more than task success.

### Perception

- identity accuracy
- object recognition
- temporal synchronization
- event detection

### World Model

- latent state reconstruction
- belief accuracy
- uncertainty calibration
- causal understanding

### Memory

- retrieval accuracy
- temporal consistency
- consolidation
- interference
- forgetting

### Relationships

- relationship-state estimation
- trust dynamics
- expectation updating
- long-term continuity

### Development

- behavioral change
- internal-state change
- structural change
- specialization
- emergence

### Plasticity

- useful role genesis
- unnecessary role pruning
- stability
- reversibility
- catastrophic forgetting

### Sim-to-Real

- similarity between simulated and real behavioral distributions

## Controlled Architecture Comparisons

Terrarium should allow identical worlds to be experienced by different architectures.

Example:

```text
Vixir A
fixed roles + conventional memory

Vixir B
neural memory

Vixir C
neural memory + dynamic roles

Vixir D
neural memory + structural plasticity

Vixir E
all above + affective dynamics
```

Compare developmental trajectories rather than only final benchmark scores.

## World Replays

Every experiment should eventually be replayable.

Researchers should be able to inspect:

```text
time
↓
sensory stream
↓
Vixir state
↓
Vixir decision
↓
world consequence
↓
new sensory stream
```

A future UI should synchronize:

```text
world truth
camera
audio
text
Vixir beliefs
memory retrieval
relationship state
active roles
predictions
actions
```

## Simulation-to-Real Calibration

Synthetic ground truth is not automatically human truth.

Calibration should be:

```text
human behavioral data
        ↓
psychological model
        ↓
Terrarium simulation
        ↓
compare distributions
        ↓
parameter/model adjustment
        ↓
repeat
```

Potential calibration targets:

- reaction distributions
- decision making
- social behavior
- memory
- personality
- cooperation
- trust
- conflict
- Theory of Mind
- longitudinal behavioral change

The goal is to reproduce appropriate **behavioral distributions**, not one supposedly perfect human.

## Long-Term Direction

Terrarium should eventually support multiple artificial cognitive architectures:

```text
                         TERRARIUM
                              │
             ┌────────────────┼──────────────────┐
             │                │                  │
         VIXIR A           VIXIR B            VIXIR C
             │                │                  │
        architecture      architecture       architecture
             │                │                  │
             └────────────────┼──────────────────┘
                              ↓
                       developmental data
                              ↓
                       comparative analysis
```

The goal is for Terrarium to become a laboratory for artificial cognition rather than a Vixir-specific test harness.

## Core Scientific Loop

```text
                  REAL HUMAN DATA
                       │
                       ↓
                CALIBRATE TERRARIUM
                       │
                       ↓
                   SYNTHETIC WORLD
                       │
                       ↓
                  SENSOR STREAM
                       │
                       ↓
                     VIXIR
                       │
                       ↓
                  INTERNAL STATE
                       │
              ┌────────┴────────┐
              ↓                 ↓
       WORLD GROUND TRUTH    VIXIR MODEL
              │                 │
              └────────┬────────┘
                       ↓
                   DISCREPANCY
                       ↓
                    EXPERIMENT
                       ↓
                 ARCHITECTURE CHANGE
                       ↓
                  SIMULATE AGAIN
```

This creates a closed research loop for artificial cognition.

## Phase Order

The practical implementation order is therefore:

```text
Phase 0
Textual causal world
        ↓
Phase 1
Audio
        ↓
Phase 2
Vision
        ↓
Phase 3
Ambient multimodality
        ↓
Phase 4
Physical environments
        ↓
Long-horizon developmental simulation
        ↓
Counterfactual experimentation
        ↓
Scientific evaluation
        ↓
Structural plasticity / architecture development
        ↓
Sim-to-real calibration
```

The early phases are not throwaway prototypes. They establish the same world-state, causal, observation, action, and evaluation abstractions that later phases should continue to use.
