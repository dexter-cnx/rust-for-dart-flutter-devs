# Testing and CI

[ภาษาไทย](th/testing_ci.md)

## Test layers

### Rust

- unit tests for pure algorithms
- property/fuzz tests for parsers when justified
- integration tests for public boundary APIs
- concurrency/cancellation tests

### Dart

- adapter unit tests using a fake/generated API facade
- DTO and error mapping tests
- Riverpod/Cubit state tests
- widget tests that replace `NativeEngine` with a fake

### End-to-end

- integration test against the real native bridge
- Android and iOS smoke build
- physical-device performance checks for critical workloads

## CI baseline

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

If generated bindings are committed, regenerate them in CI and fail when `git diff --exit-code` is non-empty.

## Platform matrix

Do not multiply CI jobs without purpose. Test the platforms you ship. Native packaging failures are platform-specific, so at least one real build per release target is valuable.

## Benchmarks

Keep microbenchmarks separate from app-level performance tests. Record payload sizes, warm-up behavior, release mode, hardware, and bridge version. A benchmark without these conditions is hard to compare later.
