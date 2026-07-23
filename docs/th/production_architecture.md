# Production Architecture สำหรับ Flutter + Rust

[English](../production_architecture.md)

เป้าหมายคือให้ Rust เป็น implementation ที่เปลี่ยนแทนได้หลัง Dart interface

```text
Flutter UI
  ↓
Riverpod / Cubit
  ↓
NativeEngine interface
  ↓
RustNativeEngine adapter
  ↓
Generated bridge
  ↓
Rust public API
  ↓
Rust domain/services
```

ทุก boundary method ควรระบุ input/output, ขนาด payload, sync/async, CPU/I/O, cancellation, progress, concurrency, error categories และ ownership ของ buffer

หลีกเลี่ยง generated bridge ใน Widget และหลีกเลี่ยง source of truth ซ้ำกันทั้ง Dart/Rust
