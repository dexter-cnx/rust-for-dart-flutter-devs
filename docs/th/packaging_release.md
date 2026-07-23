# Packaging และ Release

[English](../packaging_release.md)

การเพิ่ม Rust ทำให้ release pipeline มี native toolchain เพิ่มขึ้น ต้องกำหนด build contract ให้ชัดเจน

## Application vs Library

Flutter application สามารถ compile Rust เป็นส่วนหนึ่งของ build pipeline ตัวเองได้

แต่ reusable Flutter package ต้องตัดสินใจว่า consumer จะ:

- compile Rust จาก source
- หรือรับ prebuilt native artifacts

แต่ละแบบมีผลต่อ compatibility, binary size และ release complexity

## FRB Integration Backends

FRB v2 รองรับ workflow ผ่าน `flutter_rust_bridge_codegen create/integrate`

ปัจจุบัน default backend คือ Cargokit และมี Native Assets backend สำหรับ SDK ที่รองรับ build hooks/code assets

เลือกตาม:

- supported Flutter/Dart SDK
- platform matrix
- release model
- CI environment

ควรตรวจ upstream docs เมื่อ upgrade

## Pin Build Contract

บันทึกอย่างน้อย:

- Flutter/Dart SDK range
- Rust toolchain channel/version policy
- FRB runtime package version
- FRB codegen version
- Android NDK/minSdk/ABI
- iOS/macOS deployment target
- desktop architectures

## Generated Code

เลือก policy เดียว:

- commit generated code และตรวจ drift ใน CI
- หรือ generate deterministic ใน CI/build

ห้าม silent mismatch ระหว่าง generator กับ runtime version

## Release Checklist

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-features`
4. `flutter analyze`
5. `flutter test`
6. regenerate bridge code
7. build ทุก release platform
8. device smoke tests
9. benchmark critical pipeline ใน release mode
10. ตรวจว่า native binary/symbol ถูก bundle
11. อัปเดต compatibility notes/changelog

## CI Matrix

อย่าเพิ่ม matrix ทุก OS/architecture โดยไม่มีเหตุผล แต่ platform ที่ ship จริงควรมีอย่างน้อยหนึ่ง real build เพราะ native packaging failure มัก platform-specific

## Reproducibility

สำหรับ application ควร commit `Cargo.lock` เพื่อ lock dependency graph

ส่วน `target/` เป็น build artifacts และควร ignore

## Release Mode สำคัญ

Performance ของ debug Rust/Flutter อาจต่างจาก release มาก จึงไม่ควรใช้ debug benchmark ตัดสิน production architecture
