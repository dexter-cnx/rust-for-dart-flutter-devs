# Rust for Dart & Flutter Developers — คู่มือฉบับภาษาไทย

**ภาษาเริ่มต้นของ repository คือ [English](README.md)** · ภาษาไทยฉบับนี้ใช้เป็นคู่มือคู่ขนานสำหรับ Flutter Developer ที่มีประสบการณ์

> เป้าหมายของ repo นี้คืออธิบาย Rust โดยเริ่มจาก mental model ที่ Dart/Flutter Developer คุ้นเคย แล้วค่อยพาไปถึง production integration ระหว่าง Flutter ↔ Rust

## สิ่งที่ repo นี้ครอบคลุม

### Part I — Rust ผ่านมุมมองของ Dart

- `final` / `var` → `let` / `let mut`
- Dart types → Rust scalar/collection types
- `String` → `String` และ `&str`
- functions, closures และ expression-oriented syntax
- Dart null safety → `Option<T>`
- `class` → `struct` + `impl`
- sealed/enhanced enum → Rust `enum` + `match`

### Part II — Mental model ที่ใหม่จริงสำหรับ Flutter Developer

หัวใจสำคัญที่สุดคือ **Ownership, Borrowing และ Lifetimes** เพราะ Dart มี garbage collector แต่ Rust จัดการ resource lifetime ผ่าน type system และ compiler

```rust
let s1 = String::from("hello");
let s2 = s1; // ownership moves
// s1 ใช้ต่อไม่ได้
```

เมื่อไม่ต้องการย้าย ownership ใช้ borrow:

```rust
fn length(value: &String) -> usize {
    value.len()
}
```

กฎสำคัญ: มี shared borrow `&T` หลายตัวได้ หรือ mutable borrow `&mut T` หนึ่งตัว โดยไม่ให้เกิด aliasing mutable state ที่ไม่ปลอดภัย

### Part III — Error, Trait, Generics และ Iterator

- exception flow → `Result<T, E>`
- nullable/absence → `Option<T>`
- interface/mixin concepts → `trait`
- `List<T>` → `Vec<T>` / slice
- lazy iterator pipeline ใกล้เคียง `map/filter/fold` ของ Dart แต่ ownership ของ `.iter()`, `.iter_mut()`, `.into_iter()` มีความหมายต่างกัน

### Part IV — Async และ Concurrency

Dart ใช้ `Future`, `Stream`, Isolate ส่วน Rust มี `Future`, async runtime เช่น Tokio, OS threads และ ownership-based concurrency

ต้องจำว่า 3 เรื่องนี้ไม่ใช่เรื่องเดียวกัน:

1. API return แบบ async หรือไม่
2. งานรันที่ thread/isolate ไหน
3. งานที่เริ่มแล้วสามารถ cancel จริงได้หรือไม่

### Part V — Flutter ↔ Rust Integration

Repo อธิบายทั้ง:

- raw `dart:ffi`
- C-compatible ABI
- pointer/string/buffer ownership
- `ffigen`
- `flutter_rust_bridge` v2
- Cargokit backend
- Native Assets backend
- generated bindings
- async Rust
- progress/stream
- Rust calling Dart

สำหรับ project ส่วนใหญ่ที่ต้องการ integration ระดับสูง ควรเริ่มจาก generated bridge แล้วใช้ raw FFI เมื่อจำเป็นต้องควบคุม ABI/memory/build packaging โดยตรง

## Production Architecture

แนวทางหลักของ repo คืออย่าให้ Widget เรียก generated bridge ตรง ๆ:

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
Rust domain / services / algorithms
```

ประโยชน์คือ:

- mock Rust engine ได้ใน Dart tests
- เปลี่ยน bridge technology ได้ง่ายกว่า
- error/cancellation/progress อยู่ในจุดเดียว
- bridge-specific types ไม่รั่วเข้า domain/presentation

## Cancellation และ Latest-request-wins

งานเช่น search, image preview หรือ filter อาจมี request ใหม่มาแทนของเดิม ควรใช้ทั้ง cooperative cancellation และ request generation/request ID

```text
request #10 starts
request #11 starts → cancel #10
#10 returns late → ignore
#11 returns → accept
```

การ cancel `Future` ฝั่ง Dart ไม่ได้หมายความว่า native CPU work จะหยุดโดยอัตโนมัติ

## Large Buffer และ Zero-Copy

สำหรับ image/audio/tensor ต้องวัดทั้ง pipeline:

```text
Dart allocation
→ bridge transfer
→ Rust allocation
→ compute
→ bridge transfer
→ Dart allocation
```

อย่า optimize ด้วยคำว่า “zero-copy” ก่อน profiler ยืนยันว่า copy คือ bottleneck และต้องตรวจ implementation จริงของ bridge/platform/version ที่ใช้

## Error Mapping

อย่าให้ UI parse Rust error string:

```text
Rust internal error
→ public EngineError
→ bridge
→ Dart EngineFailure
→ localized UI message
```

Rust เก็บ diagnostic detail ส่วน Flutter presentation เป็นผู้เลือกข้อความตาม locale

## Riverpod / Cubit

ทั้ง Riverpod และ Cubit ควร depend on abstraction:

```dart
abstract interface class NativeEngine {
  Future<ProcessResult> process(ProcessRequest request);
  Future<void> cancel(String requestId);
}
```

`RustNativeEngine` เป็น implementation จริง ส่วน test ใช้ fake

## Testing Pyramid

- Rust unit tests
- Rust integration tests
- Dart adapter tests
- Riverpod/Cubit tests
- Widget tests ด้วย fake engine
- Flutter integration tests ด้วย native bridge จริง
- physical-device smoke/performance tests

CI baseline:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

## เอกสารภาษาไทย

ดูสารบัญทั้งหมดที่ [`docs/th/README.md`](docs/th/README.md)

หัวข้อ production เพิ่มเติม:

- [Production Architecture](docs/th/production_architecture.md)
- [Async และ Cancellation](docs/th/async_cancellation.md)
- [Buffers และ Zero-Copy](docs/th/buffers_zero_copy.md)
- [Error Mapping](docs/th/error_mapping.md)
- [Riverpod / Cubit Integration](docs/th/state_management.md)
- [Testing และ CI](docs/th/testing_ci.md)
- [Packaging และ Release](docs/th/packaging_release.md)
- [Security](docs/th/security.md)
- [Observability](docs/th/observability.md)
- [Migration Checklist](docs/th/migration_checklist.md)

## Runnable Rust Examples

ตัวอย่าง Rust เดิมยังรันแบบเดียวกัน:

```bash
cargo run --example 01_variables
cargo run --example 07_ownership
cargo run --example 08_borrowing
cargo run --example 14_concurrency
```

ลำดับแนะนำสำหรับ Flutter Developer ที่มีประสบการณ์:

```text
Syntax
→ Struct / Enum / Match
→ Ownership
→ Borrowing
→ Lifetimes
→ Result / Trait / Generics
→ Iterators
→ Async / Concurrency
→ dart:ffi mental model
→ flutter_rust_bridge
→ Production boundary design
```

**คำแนะนำสำคัญที่สุด:** อย่ารีบมอง Rust เป็นเพียง “Dart ที่เร็วกว่า” สิ่งที่ต้องเรียนจริงคือ ownership model และการออกแบบ boundary ที่ชัด เมื่อสองเรื่องนี้เข้าใจแล้ว การนำ Rust มาใช้กับ Flutter จะเป็นการตัดสินใจด้าน architecture ที่มีเหตุผลมากกว่าการเพิ่ม native code เพียงเพราะ performance benchmark ดูดี
