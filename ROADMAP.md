# Roadmap

This file tracks *beyond-1.0* items that are intentionally out-of-scope for the initial release.

## First-order masking

**Status**: Deferred (non-trivial; requires design & audit)

### Why it’s deferred
- Masked NTT and masked samplers (η=2/3) significantly complicate the implementation.
- Robust masking requires domain-specific gadgetry (shares refresh, masked butterflies, constant-time randomness usage) and a thorough leakage assessment.

### Proposed approach
- Feature-gated `masking` module implementing:
  - Masked CBD samplers with share-splitting and refresh.
  - Masked NTT butterflies based on arithmetic masking with randomness schedule.
  - Integration points guarded by `#[cfg(feature = "masking")]`.

### Requirements
- Independent leakage assessment and formal proofs where feasible.
- Microarchitectural evaluations across targets (x86-64, ARM64).
- Explicit trade-off documentation (performance vs. leakage).

Contributions welcome; see `SECURITY.md` for the current threat model and hardening guidance.
