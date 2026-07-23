# Architecture Patterns for Flutter + Rust

[ภาษาไทย](th/architecture.md)
## Principle 1: Rust is an engine, not your Flutter UI layer

Keep rendering, navigation, platform UX, and fast-changing product behavior in Dart unless you have a strong reason otherwise.

## Principle 2: Create one Dart boundary abstraction

Example:

```dart
abstract interface class ImageEngine {
  Future<ProcessedImage> process(ProcessRequest request);
}
```

One implementation calls Rust. Tests can use a fake implementation.

## Principle 3: Use coarse-grained calls

Prefer one call that performs a complete operation over hundreds of tiny cross-boundary calls.

## Principle 4: Keep ownership obvious

At the application architecture level, decide whether long-lived state belongs in Dart or Rust.

Avoid mirrored mutable state where both sides believe they are authoritative.

## Principle 5: Measure before and after

Record:

- total user-perceived latency
- bridge serialization/copy time
- Rust execution time
- memory use
- frame timing

Rust can improve the algorithm while the overall feature becomes slower because the bridge design is poor.

## Example Clean Architecture placement

```text
presentation/
  cubit/
  views/

domain/
  entities/
  repositories/
  usecases/

data/
  repositories/
  rust_gateway/
    rust_gateway.dart
    rust_gateway_impl.dart

rust/
  src/
    api/
    domain/
    processing/
```

The domain layer should not depend directly on generated bridge code.
