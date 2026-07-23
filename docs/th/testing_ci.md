# Testing และ CI

[English](../testing_ci.md)

แบ่ง test เป็น Rust unit/integration tests, Dart adapter tests, Riverpod/Cubit tests, Widget tests ที่ใช้ fake engine, integration tests ที่ใช้ bridge จริง และ device smoke tests

Baseline CI:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

ถ้า commit generated code ควร regenerate ใน CI และตรวจว่าไม่มี diff
