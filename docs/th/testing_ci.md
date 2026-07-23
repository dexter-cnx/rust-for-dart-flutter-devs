# Testing และ CI

[English](../testing_ci.md)

Flutter + Rust ต้อง test หลาย layer เพราะ bug อาจอยู่ใน algorithm, adapter, bridge generation หรือ native packaging

## Rust Tests

ควรมี:

- unit tests สำหรับ pure algorithms
- property/fuzz tests สำหรับ parser ที่รับ untrusted input เมื่อคุ้ม
- integration tests สำหรับ public boundary API
- concurrency/cancellation tests

## Dart Tests

ควรมี:

- adapter unit tests
- DTO/error mapping tests
- Riverpod/Cubit state tests
- Widget tests ที่แทน `NativeEngine` ด้วย fake

Presentation tests ไม่ควรต้องโหลด native library

## End-to-End

ควรมี:

- integration test ผ่าน real native bridge
- Android/iOS smoke build
- physical-device performance check สำหรับ critical workload

## CI Baseline

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

ถ้า commit generated bindings ให้ regenerate ใน CI แล้ว fail เมื่อ:

```bash
git diff --exit-code
```

มีการเปลี่ยนแปลง

## Platform Matrix

ไม่จำเป็นต้องสร้าง job ทุก combination แต่ทุก platform ที่ ship ควรมีอย่างน้อยหนึ่ง real build

Native packaging failure มักเฉพาะ platform จึงไม่พอที่จะ test Rust บน Linux อย่างเดียวแล้วสรุปว่า iOS/Android release พร้อม

## Benchmarks

แยก microbenchmark ออกจาก app-level performance test

บันทึก:

- payload size
- warm-up
- release/debug mode
- hardware
- bridge version
- concurrency setting

## Test Pyramid ที่แนะนำ

```text
              device / E2E
            integration bridge
        Dart adapter + state tests
          Rust boundary tests
        Rust pure unit/property tests
```

ชั้นล่างควรเร็วและเยอะ ชั้นบนช้ากว่าแต่จำเป็นสำหรับ packaging/runtime behavior

## Release Gate

ก่อน release อย่างน้อย:

- Rust quality gates ผ่าน
- Flutter analyze/test ผ่าน
- generated code sync
- release build ทุก target ผ่าน
- smoke test ผ่าน
- critical benchmark ไม่มี regression ที่ยอมรับไม่ได้
