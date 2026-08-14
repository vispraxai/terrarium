# Terrarium

## 1. Purpose

Terrarium is a Visprax research platform for constructing **controllable synthetic worlds in which artificial cognitive systems can develop over long periods of simulated time**.

It is not merely a game engine, chatbot simulator, or benchmark. Its defining purpose is to combine:

- physical/environmental state
- persistent simulated people
- social and psychological state
- multimodal sensory streams
- long-horizon development
- causal interventions and counterfactuals
- hidden ground truth
- scientific evaluation

The first major cognitive system to inhabit Terrarium is **Vixir**, but Terrarium should remain independent and general enough to study other architectures.

The fundamental loop is:

```text
WORLD TRUTH
     ↓
causal dynamics
     ↓
sensor model
     ↓
multimodal stream
     ↓
VIXIR
     ↓
action
     ↓
world change
```

## 2. Core Research Question

> **Can we construct controllable synthetic worlds in which artificial minds can develop, while retaining enough ground truth about physical, social, cognitive, and affective state to scientifically measure their internal development?**

A related question is whether development inside Terrarium can help predict which architectures will perform well during long-term real-world interaction.

## 3. Why Terrarium Exists

Long-horizon cognition is difficult to evaluate with short prompts.

Vixir is intended to develop properties such as:

- persistent memory
- identity
- relationships
- affective dynamics
- self-models
- agency
- continual learning
- structural plasticity
- dynamic roles
- multimodal environmental understanding

These properties may require weeks, months, or years to emerge.

Without simulation:

```text
change architecture
→ interact for months
→ evaluate
→ change architecture again
```

With Terrarium:

```text
change architecture
→ simulate months/years
→ evaluate
→ intervene
→ repeat thousands of times
```

Terrarium therefore acts as a **developmental laboratory for artificial cognition**.

## 4. Central Design Principle: Ground Truth

Terrarium knows things Vixir does not.

For example:

```text
WORLD TRUTH

Person:
    identity
    actual beliefs
    goals
    intentions
    memories
    affective state
    relationship state

Environment:
    physical state
    objects
    events
    causal history
```

Vixir instead receives:

```text
camera
microphone
text
screen
device signals
ambient sensors
```

The critical boundary is:

```text
WORLD TRUTH
     │
     │ sensor transformation
     ↓
OBSERVATIONS
     │
     ↓
VIXIR
```

**Never directly expose World Truth to Vixir.**

This makes it possible to distinguish:

1. what actually happened;
2. what Vixir observed;
3. what Vixir inferred;
4. what Vixir remembered;
5. what Vixir predicted.

## 5. Research Proposition

Terrarium should not attempt to create a perfect copy of reality.

It should make **reality experimentally tractable**.

The real world provides:

```text
richness
but little control
```

A synthetic world provides:

```text
control
but potentially less realism
```

Terrarium seeks the useful intersection:

```text
               REALISM
                  ▲
                  │
                  │       Terrarium
                  │          ●
                  │
                  └──────────────────→ CONTROL
```

The ideal Terrarium is sufficiently realistic that cognitive development transfers meaningfully to reality, while remaining sufficiently controlled that researchers can perform experiments impossible with real people.

That is the fundamental research proposition behind Terrarium.
