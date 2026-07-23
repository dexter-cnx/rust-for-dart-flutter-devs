# รูปแบบสถาปัตยกรรมสำหรับ Flutter + Rust

[English](../architecture.md)

เอกสารนี้อธิบายวิธีวาง Rust ในแอป Flutter โดยไม่ทำให้ generated bridge หรือ native detail ไหลเข้าไปปนกับ Widget, state management และ business logic ของฝั่ง Dart

## หลักการที่ 1: ให้ Rust เป็น Engine ไม่ใช่ Flutter UI layer

โดยทั่วไปควรเก็บสิ่งเหล่านี้ไว้ฝั่ง Dart/Flutter:

- Rendering และ Widget tree
- Navigation และ route lifecycle
- Platform UX ที่เปลี่ยนตาม product requirement บ่อย
- State ที่ผูกกับหน้าจอและ interaction
- Localization และข้อความที่ผู้ใช้เห็น

Rust เหมาะกับส่วนที่เป็น computational engine, parser, codec, algorithm, reusable native core หรือ logic ที่ต้องแชร์ข้ามหลาย platform มากกว่า

แนวคิดสำคัญคือ **อย่าย้าย code ไป Rust เพียงเพราะ Rust เร็วกว่าในเชิงภาษา** ให้ย้ายเมื่อ workload นั้นวัดแล้วว่าเป็น bottleneck หรือมีเหตุผลด้าน architecture ที่ชัดเจน

## หลักการที่ 2: สร้าง Dart boundary abstraction เพียงจุดเดียว

ตัวอย่าง:

```dart
abstract interface class ImageEngine {
  Future<ProcessedImage> process(ProcessRequest request);
}
```

จากนั้นสร้าง implementation ที่เรียก Rust:

```dart
final class RustImageEngine implements ImageEngine {
  @override
  Future<ProcessedImage> process(ProcessRequest request) async {
    // เรียก generated bridge ภายในไฟล์ adapter นี้เท่านั้น
    throw UnimplementedError();
  }
}
```

ประโยชน์คือ:

- Presentation ไม่รู้ว่า backend เป็น Rust
- Unit test ใช้ `FakeImageEngine` ได้
- เปลี่ยน bridge library หรือ fallback เป็น pure Dart ได้ง่าย
- Generated types ไม่รั่วเข้า domain layer

## หลักการที่ 3: ใช้ coarse-grained calls

ควรออกแบบให้หนึ่งครั้งที่ข้าม Dart ↔ Rust ทำงานเป็น operation ที่มีความหมายครบถ้วน

ดี:

```text
processImage(bytes, options) -> ProcessedImage
```

ไม่ดี:

```text
for each pixel:
  Dart -> Rust -> Dart
```

Cross-boundary call มีต้นทุนทั้ง serialization, copying, scheduling และ error translation ดังนั้น algorithm ที่เร็วมากอาจช้าลงเมื่อ boundary ละเอียดเกินไป

## หลักการที่ 4: ทำ ownership ของ state ให้ชัด

ในระดับ application architecture ต้องตัดสินใจว่า long-lived state อยู่ที่ใด

ตัวอย่างที่เหมาะกับ Dart เป็น source of truth:

- selected tab
- current route
- UI filter ที่เปลี่ยนถี่
- form state

ตัวอย่างที่อาจเหมาะกับ Rust:

- parsed model ขนาดใหญ่
- expensive cache
- image processing session
- simulation engine
- database/index ที่ใช้ algorithm native

หลีกเลี่ยง mirrored mutable state ที่ทั้ง Dart และ Rust คิดว่าตัวเองเป็น authoritative source เพราะจะเกิด synchronization bug ได้ง่าย

## หลักการที่ 5: วัดก่อนและหลัง

ควรวัดอย่างน้อย:

- latency ที่ผู้ใช้รับรู้ทั้งหมด
- เวลา serialize/copy ผ่าน bridge
- เวลาคิวรอ native execution
- Rust compute time
- memory peak
- Flutter frame timing

อย่าวัดเฉพาะ Rust function แล้วสรุปว่า feature เร็วขึ้น เพราะ bridge design อาจทำให้ end-to-end ช้ากว่าเดิม

## ตัวอย่างตำแหน่งใน Clean Architecture

```text
presentation/
  cubit/
  views/

domain/
  entities/
  repositories/
  usecases/

data/
  repositories/
  rust_gateway/
    rust_gateway.dart
    rust_gateway_impl.dart

rust/
  src/
    api/
    domain/
    processing/
```

Domain layer ไม่ควร import generated bridge code โดยตรง

## Dependency direction ที่แนะนำ

```text
Widget / Notifier / Cubit
        ↓
Domain interface
        ↓
Rust adapter
        ↓
Generated bridge
        ↓
Rust public API
        ↓
Rust internal domain/services
```

ทิศทางนี้ช่วยให้ Rust เป็น implementation detail และยังรักษาความสามารถในการ test ฝั่ง Flutter ได้ดี

## Checklist ตอน review architecture

ก่อน merge feature ที่เพิ่ม Rust ให้ถามว่า:

1. Widget import generated bridge อยู่หรือไม่
2. Boundary call หยาบพอหรือยัง
3. Source of truth ของ mutable state อยู่ฝั่งใด
4. Error ถูก map เป็น typed domain failure หรือยัง
5. มี cancellation/lifecycle semantics หรือไม่
6. วัด performance แบบ end-to-end หรือยัง

ถ้าคำตอบเหล่านี้ไม่ชัด แปลว่า architecture boundary ยังไม่พร้อมสำหรับ production
