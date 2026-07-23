# `flutter_rust_bridge` — Practical Integration Guide

[ภาษาไทย](th/flutter_rust_bridge.md)

`flutter_rust_bridge` (FRB) v2 generates Dart ↔ Rust glue while allowing the Flutter project and Rust crate to remain recognizable native projects.

## Current v2 workflow

Create a new project:

```bash
cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen create my_app
cd my_app
flutter run
```

Integrate an existing Flutter project:

```bash
flutter_rust_bridge_codegen integrate
```

After Rust API changes:

```bash
flutter_rust_bridge_codegen generate
# development convenience
flutter_rust_bridge_codegen generate --watch
```

The current default integration backend is **Cargokit**. A **Native Assets** backend is available for compatible Dart/Flutter SDKs:

```bash
flutter_rust_bridge_codegen integrate --integration-backend native-assets
```

Re-check the upstream quickstart and integration documentation when upgrading because generator commands and backend support can evolve.

## API design

Keep `rust/src/api/` as a deliberately small public boundary. Internal modules can use richer Rust types; boundary types should be stable and easy to reason about from Dart.

```rust
pub struct ResizeRequest {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn resize(request: ResizeRequest) -> Result<Vec<u8>, EngineError> {
    // delegate to domain/service modules
    todo!()
}
```

## Async and CPU work

FRB supports synchronous and asynchronous patterns, but API shape and execution location are separate concerns. For expensive CPU work, verify that it does not block Flutter's UI-critical execution path. Benchmark the real call path.

## Rust calling Dart

FRB v2 supports Rust-to-Dart interactions. Use callbacks selectively. For large architectures, callback-heavy two-way dependencies can become harder to reason about than a clear command/event boundary.

## Streams and progress

Long-running work can expose progress, but apply backpressure or throttling. UI does not need thousands of progress events per second.

## Recommended Flutter architecture

```text
Widget / Notifier / Cubit
        ↓
NativeEngine interface
        ↓
RustNativeEngine adapter
        ↓
Generated FRB API
        ↓
Rust api module
        ↓
Rust domain/services
```

Generated code should not be imported throughout the Widget tree.

## Testing strategy

- Rust unit/integration tests for native behavior.
- Dart adapter tests for mapping and cancellation.
- State-management tests using fake `NativeEngine`.
- Flutter integration tests with the real generated bridge.
- Physical-device smoke/performance tests for shipping platforms.

## Version discipline

Treat runtime package and code generator compatibility as part of the build contract. Record versions in the repository and regenerate deliberately during upgrades.

## References

Use the official flutter_rust_bridge v2 Introduction, Quickstart, and Integration documentation as the source of truth for exact current commands and backend support.
