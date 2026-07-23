# `flutter_rust_bridge` — คู่มือ Integration เชิงปฏิบัติ

[English](../flutter_rust_bridge.md)

`flutter_rust_bridge` (FRB) v2 ช่วย generate glue code ระหว่าง Dart ↔ Rust โดยยังรักษาโครงสร้าง Flutter project และ Rust crate ให้เป็น native project ที่เข้าใจได้ ไม่จำเป็นต้อง hand-write raw FFI ทุก type

## Workflow ของ v2

สร้าง project ใหม่:

```bash
cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen create my_app
cd my_app
flutter run
```

Integrate เข้า Flutter project เดิม:

```bash
flutter_rust_bridge_codegen integrate
```

หลังแก้ Rust public API:

```bash
flutter_rust_bridge_codegen generate
```

ตอนพัฒนาใช้ watch mode ได้:

```bash
flutter_rust_bridge_codegen generate --watch
```

Integration backend ปัจจุบัน default คือ **Cargokit** และมี **Native Assets** backend สำหรับ Dart/Flutter SDK ที่รองรับ:

```bash
flutter_rust_bridge_codegen integrate --integration-backend native-assets
```

คำสั่งและ backend support อาจเปลี่ยนตาม version จึงควรตรวจ upstream docs เมื่อ upgrade

## API Design

ควรให้ `rust/src/api/` เป็น public boundary ที่ตั้งใจออกแบบให้เล็กและ stable

ตัวอย่าง:

```text
rust/src/
  api/
    image_api.rs
  domain/
    image.rs
  services/
    processor.rs
```

`api/` ทำหน้าที่:

- รับ boundary DTO
- validate input ระดับ API
- เรียก domain/service
- map error เป็น public category

อย่า expose internal type ทุกชนิดเพียงเพราะ generator รองรับ

## Boundary Types

Boundary ที่ดีควรใช้ type ที่:

- ownership ชัด
- serialize/convert ง่าย
- stable ต่อ version
- ไม่ผูกกับ internal lifetime

เช่น DTO แบบ owned value มักปลอดภัยกว่าการพยายาม expose reference-heavy API

## Generated Code ควรอยู่ที่ไหน

อย่า import generated bindings ตรงจาก Widget หรือ Cubit

แนะนำ:

```text
presentation
  ↓
domain interface
  ↓
RustNativeEngine adapter
  ↓
generated FRB API
```

ข้อดีคือ state management test ได้โดยไม่โหลด native library จริง

## Async

FRB ช่วยทำ async bridge ได้สะดวก แต่ยังต้องเข้าใจ execution semantics

คำถามที่ต้องตอบ:

- Rust function เป็น CPU-bound หรือ I/O-bound
- รันบน runtime/thread ใด
- cancel ได้หรือไม่
- progress ส่งกลับอย่างไร
- concurrency limit เท่าใด

`Future` ฝั่ง Dart ไม่ได้แทนคำตอบทั้งหมดนี้

## Error Mapping

อย่าให้ UI พึ่ง arbitrary Rust error strings

แนะนำให้ Rust public API คืน stable enum/category แล้ว adapter map เป็น Dart domain failure

```text
Rust EngineError::UnsupportedFormat
  ↓
FRB generated representation
  ↓
RustNativeEngine adapter
  ↓
UnsupportedFormatFailure
  ↓
localized UI message
```

## Large Buffers

สำหรับ image/audio/binary workload ให้ benchmark transfer cost ด้วย

อย่าสมมติว่า generated bridge ทำ zero-copy ในทุกกรณี เพราะ path จริงขึ้นกับ type, API, version และ platform

## Cancellation

สำหรับ operation ยาว ให้ใช้ request ID/token และ cooperative cancellation

ฝั่ง Dart ยังควรใช้ latest-request-wins เพื่อ ignore stale result เพราะ cancel สามารถ race กับ completion ได้

## Stateful Rust Objects

ถ้าเก็บ expensive state ใน Rust ให้ lifecycle ชัด:

- create
- use
- cancel pending work
- dispose

อย่าพึ่ง global singleton ถ้า engine ควรเป็น route/session scoped

## Cargokit vs Native Assets

เลือกตาม supported SDK และ release model

### Cargokit

เหมาะเมื่อใช้ workflow FRB มาตรฐานและต้องการ integration ที่เป็น default ของ tooling ปัจจุบัน

### Native Assets

เหมาะเมื่อ toolchain/SDK ที่รองรับ build hooks และ code assets อยู่ใน support matrix ของ project

อย่าเปลี่ยน backend ระหว่าง release โดยไม่มี CI/build test ทุก platform

## Version Pinning

Production project ควรบันทึกอย่างน้อย:

- Flutter/Dart SDK range
- Rust toolchain policy
- `flutter_rust_bridge` runtime version
- `flutter_rust_bridge_codegen` version
- integration backend

หลีกเลี่ยง runtime/generator version mismatch

## Generated Code Policy

เลือกอย่างใดอย่างหนึ่ง:

1. commit generated code แล้วตรวจ drift ใน CI
2. generate deterministically ระหว่าง build/CI

อย่ามีบาง developer commit generated code ขณะที่บาง pipeline regenerate ด้วยคนละ version

## Testing

ควรมีอย่างน้อย:

- Rust unit tests
- Rust boundary API tests
- Dart adapter tests
- Riverpod/Cubit tests ด้วย fake engine
- real bridge integration test
- Android/iOS smoke build

## Performance

วัด pipeline เต็ม:

```text
Dart preprocessing
→ bridge transfer
→ native queue
→ Rust compute
→ bridge transfer back
→ Dart state update
```

Rust function เร็วขึ้นไม่ได้แปลว่า feature เร็วขึ้นเสมอ

## Upgrade Strategy

ก่อน upgrade FRB:

1. อ่าน changelog/upstream docs
2. pin version ใน branch แยก
3. regenerate bindings
4. รัน Rust + Flutter tests
5. build ทุก release platform
6. benchmark critical paths

แนวทางนี้ลดความเสี่ยงจาก generator/backend changes
