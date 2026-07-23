# Migration / Adoption Checklist

[ภาษาไทย](th/migration_checklist.md)

Use this when moving an existing Flutter feature to Rust.

## Before

- Profile the Dart implementation.
- Identify the actual bottleneck.
- Define expected speed/memory/portability benefit.
- Record representative payload sizes.

## Boundary design

- Choose raw FFI or generated bridge.
- Define coarse-grained API.
- Define owned buffer semantics.
- Define error categories.
- Define async/cancellation/progress.
- Define source of truth for mutable state.

## Implementation

- Build pure Rust core first.
- Add Rust tests.
- Add a thin public API layer.
- Add Dart adapter.
- Keep UI/state management bridge-agnostic.

## Validation

- Compare correctness against the old implementation.
- Benchmark release builds on representative devices.
- Test cancellation and stale results.
- Test low-memory/large-input behavior.
- Build every shipping platform.

## Rollout

- Keep a feature flag or fallback when risk justifies it.
- Add telemetry around native failures and latency.
- Document toolchain compatibility.
