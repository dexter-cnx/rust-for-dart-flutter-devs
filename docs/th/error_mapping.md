# การ Map Error ระหว่าง Rust และ Flutter

[English](../error_mapping.md)

UI ไม่ควรพึ่งข้อความ error string จาก Rust โดยตรง ควรมี stable typed contract ระหว่างสองฝั่ง

## Stable Error Contract

Rust:

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

Dart:

```dart
sealed class EngineFailure {
  const EngineFailure();
}

final class InvalidInputFailure extends EngineFailure {}
final class CancelledFailure extends EngineFailure {}
final class UnsupportedFormatFailure extends EngineFailure {}
final class InternalEngineFailure extends EngineFailure {}
```

Adapter layer มีหน้าที่ map error code/category จาก Rust เป็น domain failure

## แยก Diagnostic Detail ออกจาก User Message

ฝั่ง Rust สามารถเก็บ source chain รายละเอียดไว้สำหรับ log/telemetry ได้ แต่ไม่ควรส่งสิ่งเหล่านี้ให้ผู้ใช้ตรง ๆ:

- filesystem path
- token/secret
- stack trace ภายใน
- arbitrary panic text
- implementation detail

Presentation layer ควรเลือกข้อความ localized จาก failure type

## Localization อยู่ฝั่ง Flutter

Rust ควรคืนประเภทความผิดพลาด ไม่ใช่ข้อความ UI เช่น:

```text
UnsupportedFormat
```

จากนั้น Flutter แปลงเป็น:

```text
TH: ไม่รองรับรูปแบบไฟล์นี้
EN: This file format is not supported
```

ทำให้ Rust engine ใช้ซ้ำได้และไม่ผูกกับ localization framework

## Panic Policy

`panic!` ควรถูกมองเป็น bug หรือ invariant violation ไม่ใช่ recoverable error ปกติ

Public API ควรคืน `Result` สำหรับกรณีที่คาดว่าจะเกิด เช่น invalid input หรือ I/O failure

Production ควรกำหนดว่า unexpected native failure:

- ถูก capture ใน telemetry อย่างไร
- Dart เห็น fallback category อะไร
- app crash หรือ recover ได้หรือไม่

## Compatibility

การเพิ่ม error category ใหม่อาจถือเป็น contract change

Dart side ควรมี fallback เช่น `Unknown` หรือ `Internal` เพื่อให้ client เก่ารับมือ category ใหม่ได้อย่างปลอดภัยเมื่อเป็นไปได้

## อย่าใช้ String เป็น Contract หลัก

ไม่ดี:

```dart
if (error.message.contains('cancelled')) { ... }
```

ดี:

```dart
switch (failure) {
  case CancelledFailure():
    ...
  case UnsupportedFormatFailure():
    ...
}
```

## สิ่งที่ควร Test

- Rust error แต่ละชนิด map ถูกประเภท
- unknown code มี fallback
- internal detail ไม่รั่วสู่ UI
- cancelled ไม่ถูกแสดงเป็น generic failure ที่น่ากลัว
- localization อยู่ presentation layer
