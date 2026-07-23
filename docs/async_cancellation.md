# Async, Cancellation, and Backpressure

[ภาษาไทย](th/async_cancellation.md)

## Three separate concerns

Do not collapse these concepts:

- **Async API:** Dart receives a Future/Stream-like abstraction.
- **Execution location:** work runs on or away from the UI isolate/thread.
- **Cancellation:** in-flight native work can actually stop cooperatively.

An API can be asynchronous while still scheduling too much CPU work, and a cancelled Dart consumer does not necessarily terminate native computation.

## Cooperative cancellation

Use checkpoints in long-running Rust algorithms. A token can be an atomic flag, request registry entry, or runtime-specific cancellation primitive. Check at useful boundaries, not every machine instruction.

```rust
for chunk in chunks {
    if cancelled.load(Ordering::Relaxed) {
        return Err(EngineError::Cancelled);
    }
    process(chunk)?;
}
```

## Latest-request-wins

For search, previews, and filters:

1. Increment a Dart request generation.
2. Start native work with that generation/request ID.
3. Cancel the previous request when possible.
4. Ignore late results whose ID is no longer current.

Ignoring stale results is still necessary because cancellation can race with completion.

## Backpressure

A progress stream or event stream can overwhelm Dart if Rust emits too frequently. Coalesce or throttle progress updates. Prefer meaningful milestones or a bounded update rate.

## Shutdown

On route disposal or app shutdown, define whether you:

- cancel and await completion,
- detach work and ignore the result,
- keep a process-wide engine alive.

Never leave lifecycle behavior accidental.
