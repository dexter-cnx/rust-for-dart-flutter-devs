# Packaging and Release

[ภาษาไทย](th/packaging_release.md)

## Application vs library

An application can build Rust as part of its own build pipeline. A reusable Flutter package must decide whether consumers compile Rust from source or receive prebuilt native artifacts.

## flutter_rust_bridge integration backends

The v2 tooling supports integration via `flutter_rust_bridge_codegen create/integrate`. The current default backend is Cargokit. A Native Assets backend is also available for compatible Dart/Flutter SDKs that support build hooks and code assets.

Choose based on your supported SDK range and release model. Re-check upstream documentation when upgrading because this area evolves.

## Pin the build contract

Document:

- Flutter/Dart SDK range
- Rust toolchain channel/version policy
- `flutter_rust_bridge` runtime package version
- `flutter_rust_bridge_codegen` version
- Android NDK / minSdk / ABI targets
- iOS/macOS deployment targets
- supported desktop architectures

## Generated code

Choose one policy:

- commit generated code and verify drift in CI, or
- generate deterministically during the build.

Do not silently mix generator/runtime versions.

## Release checklist

1. Run Rust fmt/clippy/tests.
2. Run Flutter analyze/tests.
3. Regenerate bridge code.
4. Build each release platform.
5. Run device smoke tests.
6. Benchmark critical pipelines in release mode.
7. Verify symbols/binaries are bundled correctly.
8. Update compatibility notes and changelog.
