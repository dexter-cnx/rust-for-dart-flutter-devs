# รูปแบบสถาปัตยกรรม Flutter + Rust

[English](../architecture.md)

## หลักการสำคัญ

ให้ Rust ทำหน้าที่เป็น **engine** หรือ implementation detail หลัง Dart interface ไม่ใช่ UI layer ของ Flutter

```text
Widget → Riverpod/Cubit → Dart Interface → Rust Adapter → Bridge → Rust Engine
```

- Widget ไม่ควร import generated bridge โดยตรง
- ใช้ coarse-grained API ลดจำนวน call ข้าม boundary
- กำหนด ownership ของ buffer ให้ชัด
- มี source of truth ของ mutable state เพียงฝั่งเดียวเมื่อทำได้
- วัด performance แบบ end-to-end ก่อนและหลังย้ายโค้ดไป Rust

Clean Architecture สามารถวาง `NativeEngine` เป็น domain-facing abstraction และให้ `RustNativeEngine` อยู่ใน infrastructure/data layer ได้
