# `flutter_rust_bridge` — คู่มือใช้งานจริง

[English](../flutter_rust_bridge.md)

FRB v2 ช่วย generate glue ระหว่าง Dart/Flutter กับ Rust ทำให้ public Rust API ถูกเรียกจาก Dart ได้โดยไม่ต้องเขียน raw FFI glue จำนวนมาก

## Workflow ปัจจุบัน

```bash
cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen create my_app
```

สำหรับ project เดิม:

```bash
flutter_rust_bridge_codegen integrate
```

เมื่อแก้ Rust API:

```bash
flutter_rust_bridge_codegen generate
flutter_rust_bridge_codegen generate --watch
```

Backend เริ่มต้นปัจจุบันคือ **Cargokit** และมี **Native Assets** เป็นทางเลือกสำหรับ Flutter/Dart SDK ที่รองรับ build hooks/code assets

ใน production ควรซ่อน generated API หลัง `NativeEngine`/adapter, แยก error mapping, cancellation, progress และ observability ออกจาก Widget และ state management
