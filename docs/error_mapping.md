# Error Mapping Across Rust and Flutter

[ภาษาไทย](th/error_mapping.md)

## Stable contract

UI code should depend on stable categories, not implementation strings.

```rust
pub enum EngineError {
    InvalidInput,
    Cancelled,
    UnsupportedFormat,
    ResourceExhausted,
    Io,
    Internal,
}
```

Map into Dart failures:

```dart
sealed class EngineFailure {
  const EngineFailure();
}

final class InvalidInputFailure extends EngineFailure {}
final class CancelledFailure extends EngineFailure {}
final class UnsupportedFormatFailure extends EngineFailure {}
final class InternalEngineFailure extends EngineFailure {}
```

Keep the detailed Rust source chain for diagnostics, but do not expose secrets, filesystem paths, or arbitrary internal text directly to users.

## Presentation localization

The adapter/domain layer returns a typed failure. The presentation/localization layer chooses user-facing text. Rust should not own Flutter localization strings.

## Panic policy

Treat panics as bugs, not ordinary recoverable errors. Public Rust APIs should return `Result` for expected failure modes. Decide how unexpected native failures are surfaced and captured in telemetry.

## Compatibility

Adding a new error category can be a contract change. Include an `Unknown/Internal` fallback on the Dart side so older clients degrade safely when practical.
