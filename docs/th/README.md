# เอกสารภาษาไทย

[English documentation index](../README.md)

English เป็นภาษาหลักของ repository นี้ ส่วน `docs/th/` เป็นเอกสารภาษาไทยแบบคู่ขนาน โดยตั้งใจรักษาระดับรายละเอียดทางเทคนิคและตัวอย่างสำคัญให้ใกล้กับ English version ไม่ใช่เพียงสรุปย่อ

## Architecture และ Boundary

- [Architecture Patterns](architecture.md) — การวาง Rust เป็น engine หลัง Dart abstraction
- [Production Architecture](production_architecture.md) — layering, state ownership และ API review template
- [Riverpod / Cubit](state_management.md) — วิธีแยก state management ออกจาก generated bridge

## Integration

- [Raw dart:ffi](ffi.md) — ABI, pointer, String, buffer และ ownership contract
- [flutter_rust_bridge](flutter_rust_bridge.md) — FRB v2 workflow, generated code และ production integration

## Runtime และ Performance

- [Async / Cancellation / Backpressure](async_cancellation.md)
- [Buffers / Typed Data / Zero-Copy](buffers_zero_copy.md)
- [Performance](performance.md)
- [Observability](observability.md)

## Reliability และ Release

- [Error Mapping](error_mapping.md)
- [Testing และ CI](testing_ci.md)
- [Packaging และ Release](packaging_release.md)
- [Security](security.md)
- [Migration Checklist](migration_checklist.md)
- [Production Checklist](PRODUCTION_CHECKLIST.md)

สำหรับการเรียนจากพื้นฐานภาษา Rust ให้เริ่มที่ [README ภาษาไทย](../../README_TH.md) แล้วค่อยใช้เอกสารชุดนี้เมื่อเริ่มออกแบบ Flutter ↔ Rust integration จริง
