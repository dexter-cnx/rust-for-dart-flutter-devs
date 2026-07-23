# Production Architecture สำหรับ Flutter + Rust

[English](../production_architecture.md)

## เป้าหมาย

Rust ควรทำหน้าที่เป็น replaceable engine หลัง Dart interface ที่ stable

Widget และ presentation state ไม่ควรรู้ว่า implementation จริงเป็น:

- Rust
- pure Dart
- mock/fake
- remote service

## Layer ที่แนะนำ

```text
presentation/
  controllers, notifiers, cubits

domain/
  EngineRepository / ImageProcessor interfaces
  Dart domain models

data_or_infrastructure/
  RustEngineAdapter
  generated bridge glue

rust/
  api/       stable boundary API
  domain/    core rules and algorithms
  services/  orchestration
  infra/     codecs, filesystem, native crates
```

## Boundary Rules

1. Expose operation แบบ coarse-grained
2. Generated bridge type อยู่ใน adapter layer
3. ใช้ explicit public DTO
4. ไม่ expose internal Rust references/lifetimes ให้ Dart
5. กำหนด threading/cancellation ต่อ operation
6. Map error เป็น stable category
7. ทำ performance observable

## Stateful Engine

การเก็บ state ใน Rust เหมาะกับ:

- expensive model
- parser state
- cache/index
- incremental processor

แต่ต้องมี lifecycle ชัด:

```text
create → use → cancel pending work → dispose
```

หลีกเลี่ยง global singleton ถ้า ownership จริงเป็น per-route/per-session

## API Review Template

ทุก boundary method ควร document:

- input/output type
- maximum payload size
- sync vs async
- CPU-bound vs I/O-bound
- cancellation support
- progress semantics
- thread-safety/concurrency limit
- error categories
- buffer ownership
- idempotency

## ตัวอย่าง Dependency Flow

```text
ImagePage
  ↓
ImageCubit
  ↓
ProcessImageUseCase
  ↓
ImageProcessor interface
  ↓
RustImageProcessor
  ↓
FRB generated API
  ↓
Rust api::process_image
  ↓
Rust services::processor
```

## Anti-patterns

- Widget เรียก generated bridge ตรง ๆ
- FFI call ต่อ pixel/item ใน loop ใหญ่
- ใช้ error string เป็น contract เดียว
- คิดว่า Dart async หมายถึง native CPU ไม่ block อัตโนมัติ
- Dart และ Rust เป็น source of truth ของ mutable state เดียวกันพร้อมกัน
- expose internal Rust type เพียงเพราะ generator ทำได้

## Production Boundary Checklist

ก่อน approve API ใหม่:

- ownership ชัดหรือไม่
- cancel ได้หรือไม่
- stale result ถูกป้องกันหรือไม่
- input size มี limit หรือไม่
- failure category stable หรือไม่
- metrics/correlation ID มีหรือไม่
- test fake engine ได้หรือไม่

## Versioning

Boundary API เป็น contract ระหว่างสองฝั่ง ควรเปลี่ยนแบบ deliberate

การ rename field, เปลี่ยน enum หรือ semantic ของ error อาจกระทบ generated code และ adapter พร้อมกัน
