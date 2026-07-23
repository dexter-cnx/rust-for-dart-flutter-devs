# Riverpod and Cubit Integration

[ภาษาไทย](th/state_management.md)

## Rule: depend on an interface

```dart
abstract interface class NativeEngine {
  Future<ProcessResult> process(ProcessRequest request);
  Future<void> cancel(String requestId);
}
```

`RustNativeEngine` implements this interface using generated bridge APIs. Tests use `FakeNativeEngine`.

## Riverpod

Expose the interface through a provider, then let an AsyncNotifier/Notifier own request lifecycle. Keep generated bridge imports in the adapter file only.

```dart
final nativeEngineProvider = Provider<NativeEngine>((ref) {
  return RustNativeEngine();
});
```

For latest-request-wins, store a monotonically increasing generation or request ID in the notifier. Ignore late results after disposal or replacement.

## Cubit

Inject `NativeEngine` into the Cubit constructor. Emit explicit processing/progress/success/failure states. Override `close()` to trigger best-effort cancellation when route-scoped work must stop.

## Progress

Convert native progress into a domain `Stream<Progress>` or callback abstraction in the adapter. Throttle before emitting high-frequency UI states.

## Testing

State tests should use a fake engine and deterministic completers. Test:

- initial → loading → success
- typed failure mapping
- cancellation on disposal
- stale result ignored
- progress throttling/coalescing
