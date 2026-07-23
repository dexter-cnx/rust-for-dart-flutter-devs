# Performance Guide

[ภาษาไทย](th/performance.md)
## Do not optimize language choice; optimize the pipeline

Measure the complete path:

```text
User action
→ Dart preprocessing
→ bridge encoding/copy
→ Rust queue/wait
→ Rust compute
→ bridge return/copy
→ Dart decoding
→ widget update
```

## Typical high-value Rust workloads

- CPU-heavy image transforms
- compression
- complex parsers
- large deterministic simulations
- reusable cross-platform native engines

## Typical low-value migrations

- basic JSON models
- simple CRUD orchestration
- navigation
- form validation
- most ordinary networking

## Large buffers

Large buffers are often where bridge design matters most. Prefer designs that minimize redundant conversion and copying. Use bridge-supported zero-copy or managed buffer mechanisms only after understanding their lifecycle guarantees.

## Benchmark correctly

Benchmark release/optimized builds. Debug Rust and debug Flutter performance can be misleading.

Use realistic payload sizes and warm-up behavior.

## UI responsiveness

A fast 50 ms synchronous call can still be unacceptable on the UI isolate.

Performance includes responsiveness, not only throughput.
