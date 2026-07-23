# Production Readiness Checklist

[ภาษาไทย](th/PRODUCTION_CHECKLIST.md)

- [ ] Rust API is coarse-grained and versioned deliberately.
- [ ] Generated bridge types are isolated behind a Dart adapter.
- [ ] Buffer ownership and maximum payload sizes are documented.
- [ ] UI-critical calls do not synchronously block on expensive native work.
- [ ] Cancellation semantics and stale-result behavior are tested.
- [ ] Errors map to stable Dart failure categories.
- [ ] Progress streams have throttling/backpressure behavior.
- [ ] Rust fmt, Clippy, and tests pass in CI.
- [ ] Flutter analyze/tests pass in the consuming app CI.
- [ ] Generated-code drift policy is enforced.
- [ ] Android/iOS release builds are smoke-tested.
- [ ] Critical workloads are benchmarked in release mode on representative devices.
- [ ] `unsafe` and raw pointer code are reviewed explicitly.
- [ ] Toolchain and bridge compatibility are documented.
- [ ] Logs avoid secrets and raw user payloads.
