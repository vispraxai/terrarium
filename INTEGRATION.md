# Integration note

This is the next core layer as a small, source-readable workspace. It is intentionally isolated from the rest of the Terrarium repository because the local checkout may already contain the previous event/replay iteration.

Merge these concepts into your current checkout rather than blindly replacing unrelated files:

1. add `effect.rs`;
2. add `replay.rs`;
3. extend `Event` with `causal_parents` and `effects`;
4. expose `EventKind` and the new replay types from `lib.rs`;
5. move existing `promise_broken` mutations into explicit effects;
6. make `WorldState` apply those effects centrally;
7. make `Simulation` own a `Run` or otherwise record every event;
8. update the CLI to demonstrate replay and branching.

The README describes the intended architecture and the source files are heavily commented for learning.
