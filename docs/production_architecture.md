# Production Architecture for Flutter + Rust

[ภาษาไทย](th/production_architecture.md)

## Goal

Rust should behave like a replaceable engine behind a stable Dart interface. Widgets and presentation state should not know whether the implementation is Rust, pure Dart, a mock, or a remote service.

## Recommended layers

```text
presentation/
  controllers, notifiers, cubits

domain/
  EngineRepository / ImageProcessor interfaces
  Dart domain models

data_or_infrastructure/
  RustEngineAdapter
  generated bridge glue (isolated here)

rust/
  api/       stable boundary API
  domain/    core rules and algorithms
  services/  orchestration
  infra/     codecs, filesystem, native crates
```

## Boundary rules

1. Expose coarse-grained operations.
2. Keep generated bridge types inside the adapter layer.
3. Use explicit public DTOs at the boundary.
4. Do not expose internal Rust references or lifetime-sensitive designs to Dart.
5. Define threading and cancellation semantics per operation.
6. Map errors into stable categories.
7. Make performance characteristics observable.

## Stateful engines

Keeping state in Rust is useful for expensive models, caches, parsers, or incremental processors. Give each stateful object an explicit lifecycle. Dart should know when it is created and disposed. Avoid a global singleton unless process-wide ownership is genuinely required.

## API review template

For each boundary method document:

- input/output types
- maximum expected payload size
- sync vs async
- CPU-bound vs I/O-bound
- cancellation support
- progress semantics
- thread-safety / concurrency limit
- error categories
- ownership of buffers
- whether the operation is idempotent

## Anti-patterns

- Calling generated bridge functions directly from Widgets.
- One FFI call per pixel/item in a large loop.
- Returning arbitrary strings as the only error contract.
- Assuming async Dart automatically means native CPU work is non-blocking.
- Keeping both Dart and Rust as competing sources of truth for the same mutable state.
