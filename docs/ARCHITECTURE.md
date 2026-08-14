# Terrarium — Architecture

## 1. High-Level Architecture

```text
                          TERRARIUM

       ┌───────────────────┼───────────────────┐
       │                   │                   │
   PHYSICAL              SOCIAL          PSYCHOLOGICAL
     WORLD                WORLD               WORLD
       │                   │                   │
   objects              people             beliefs
   rooms                relations          goals
   movement             interactions       memories
   physics              events             affect
   constraints                              self-model
       └───────────────────┼───────────────────┘
                           ↓
                       WORLD STATE
                           ↓
                     CAUSAL DYNAMICS
                           ↓
                       SENSOR MODEL
                           ↓
                    MULTIMODAL STREAM
                           ↓
                         VIXIR
                           ↓
                          ACTION
                           ↓
                      WORLD CHANGE
```

The implementation should keep the simulator independent of Vixir.

Terrarium provides the world and interfaces; Vixir is an external cognitive architecture inhabiting that world.

## 2. World State

Terrarium maintains an authoritative world state:

```text
WorldState
├── time
├── locations
├── objects
├── environment
├── agents
├── relationships
├── events
├── goals
├── causal history
└── latent psychological state
```

States should be versioned so researchers can reconstruct development:

```text
WorldState(t)
WorldState(t + Δt)
WorldState(t + 1 day)
WorldState(t + 1 month)
```

The simulator should preserve causal provenance between states.

## 3. Physical World

The physical world can contain:

- rooms
- buildings
- objects
- devices
- locations
- lighting
- temperature
- movement
- physical constraints
- object state
- spatial relationships

The first version should **not** begin with photorealistic 3D.

Start symbolically/textually, then add physical environments through adapters.

Potential external foundations include:

- Habitat-Sim
- AI2-THOR
- iGibson / BEHAVIOR
- VirtualHome

Terrarium should not become permanently dependent on one engine.

## 4. Social World

The social layer contains persistent entities and relationships.

```text
Person
├── identity
├── traits
├── goals
├── preferences
├── beliefs
├── memories
├── expectations
├── affective state
├── relationships
├── social roles
└── self-model
```

Relationships are dynamic state:

```text
Relationship(A, B)
├── trust
├── familiarity
├── attachment
├── expectations
├── perceived reciprocity
├── conflict
├── shared history
└── uncertainty
```

These values are primarily latent ground truth.

## 5. Simulated People

A simulated person should not simply be:

```text
LLM + personality prompt
```

Instead:

```text
PERSON
├── traits
├── goals
├── preferences
├── beliefs
├── memories
├── expectations
├── relationships
├── affect
├── self-model
└── policy/cognition
```

An LLM may be one component of the cognitive/policy layer.

PsychSim is a useful conceptual foundation for explicit beliefs, preferences, relationships and Theory of Mind.

## 6. Psychological Ground Truth

For each simulated person, Terrarium may know:

- beliefs
- goals
- intentions
- preferences
- uncertainty
- memories
- expectations
- relationship valuations
- affective/appraisal state
- self-model
- models of other people

Vixir does not get these variables.

Example:

```text
Ground truth:
belief(Person17, X) = 0.73

Vixir estimate:
belief(Person17, X) = 0.61
```

This turns cognitive evaluation into something measurable rather than merely asking whether an answer "sounds good."

## 7. Sensor Architecture

Vixir should interact with Terrarium through the same conceptual interfaces it will eventually use in reality.

Potential sensors:

```text
camera
microphone
text/message
screen
keyboard/mouse
device state
location
environmental sensors
```

The semantic interpretation should happen inside Vixir wherever possible.

## 8. Ambient Multimodal Stream

The stream must be asynchronous:

```text
TIME →

camera  ──●──●──●──●────●──●──●──●────
audio   ─────●────●●●●●────●●●●───────
screen  ───────────●────────────●──────
text    ─────────────────●──────────────
device  ──●────────────────────●────────
```

Vixir therefore has to solve:

- temporal alignment
- synchronization
- identity tracking
- multimodal fusion
- attention
- salience
- missing data
- uncertainty

Most of the stream should be ordinary and unimportant. This is intentional.

## 9. Sensor Imperfection

Simulation should contain realistic imperfections.

### Vision

- occlusion
- low light
- motion blur
- camera angle
- frame drops
- partial visibility

### Audio

- background noise
- overlapping speakers
- distance
- reverberation
- incomplete speech
- microphone limitations

### Identity

- unknown people
- similar voices
- similar appearances
- partial observations
- people entering/leaving view

### Environment

- objects moved
- unexpected events
- events outside the camera
- incomplete observability

Noise should be configurable but hidden from Vixir.

## 10. Event Model

The simulator maintains semantic world events underneath the sensor layer:

```text
person enters room
object moved
message received
phone rings
conversation starts
conversation ends
person changes plan
unexpected visitor
environment changes
relationship event
```

These are world-level events.

Vixir normally receives their **sensory consequences**, not the semantic event itself.

## 11. Closed-Loop Interaction

Vixir must be able to change the world.

```text
Saeid:
"Where are you?"

        ↓ audio

Vixir:
"I'm in the kitchen."

        ↓ speech action

Terrarium:
Saeid hears the response.

        ↓

Saeid:
"Can you come here?"

        ↓ audio

Vixir:
receives new observation
```

Vixir's behavior therefore affects its future experience.

## 12. Multi-Timescale Development

Terrarium must model:

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

## 13. Affective Dynamics

Terrarium should not require hardcoded semantic emotions.

Instead, it should create situations involving:

```text
prediction
↓
outcome
↓
prediction error
↓
valuation
↓
memory / attention / goals
↓
future behavior
```

For example:

```text
expected interaction probability = 0.91
actual interaction probability   = 0.12
```

The evaluator can inspect whether Vixir:

- updates beliefs
- retrieves memories
- changes attention
- changes expectations
- modifies relationship models
- changes goals
- develops persistent behavioral tendencies

The simulator should never simply tell Vixir:

```text
"you are sad"
```

## 14. Structural Plasticity

Terrarium provides an environment for testing Vixir's proposed dynamic architecture.

Initial:

```text
roles:
    general_reasoner
    memory
    perception
    planner
```

Possible developmental trajectory:

```text
existing architecture
        ↓
repeated cognitive bottleneck
        ↓
new role generated
        ↓
role repeatedly useful
        ↓
role strengthened
        ↓
old role becomes redundant
        ↓
old role pruned
```

This lets us experimentally test:

- role genesis
- pruning
- merging
- splitting
- specialization
- resistance to catastrophic forgetting

## 15. Ground-Truth Data Model

Every run should preserve enough information to reconstruct events:

```text
run/
├── metadata/
├── world_truth/
├── sensor_stream/
├── vixir_state/
├── actions/
├── interventions/
├── predictions/
└── evaluation/
```

Events should preserve provenance:

```text
event_id
timestamp
causal_parent
world_state_before
action
world_state_after
sensor_observations
psychological_state
```

## 16. Agent Interface

The conceptual boundary is:

```text
Terrarium
    │
    ├── world truth
    │
    ├── sensorization
    │
    ↓
Observation
    ↓
Agent / Vixir
    ↓
Action
    ↓
Terrarium
```

An agent should never receive arbitrary access to the simulator's latent state.

This permits identical worlds to be experienced by different architectures.

## 17. Psychological Debugger

Terrarium should eventually provide a debugger exposing Vixir's internal state:

```text
VIXIR INTERNAL STATE

memory
beliefs
identity
relationships
predictions
goals
affective state
active roles
self-model
uncertainty
```

The debugger should synchronize these with:

- world truth
- sensor streams
- Vixir actions
- future consequences

This allows researchers to find where a cognitive failure occurred.

## 18. Error Taxonomy

Terrarium should distinguish:

### Perception error

```text
reality ≠ observation
```

### Inference error

```text
observation ≠ belief
```

### Memory error

```text
past state ≠ recollection
```

### Prediction error

```text
predicted future ≠ actual future
```

This gives a useful decomposition of cognitive failure.

## 19. Repository Boundary

Terrarium should be a Visprax-level project rather than a Vixir subdirectory:

```text
visprax/
├── vixir/
├── terrarium/
├── substrax/
├── criticalpoints/
└── other research projects
```

Terrarium can study Vixir while remaining architecturally independent.

## 20. Proposed Repository Structure

```text
terrarium/
│
├── core/
│   ├── world/
│   ├── time/
│   ├── causality/
│   └── state/
│
├── humans/
│   ├── psychology/
│   ├── beliefs/
│   ├── goals/
│   ├── memory/
│   ├── affect/
│   ├── relationships/
│   └── theory_of_mind/
│
├── environments/
│   ├── concordia/
│   ├── virtualhome/
│   ├── habitat/
│   └── sotopia/
│
├── sensors/
│   ├── audio/
│   ├── vision/
│   ├── text/
│   ├── screen/
│   └── ambient/
│
├── streams/
│   ├── realtime/
│   ├── asynchronous/
│   ├── noise/
│   └── multimodal_sync/
│
├── development/
│   ├── accelerated_time/
│   ├── developmental_epochs/
│   ├── interventions/
│   └── curriculum/
│
├── ground_truth/
│   ├── state/
│   ├── causal_graph/
│   ├── psychological/
│   └── provenance/
│
├── experiments/
│   ├── memory/
│   ├── identity/
│   ├── relationships/
│   ├── affect/
│   ├── plasticity/
│   └── self_model/
│
└── evaluation/
    ├── perception/
    ├── world_model/
    ├── psychology/
    ├── development/
    └── sim_to_real/
```
