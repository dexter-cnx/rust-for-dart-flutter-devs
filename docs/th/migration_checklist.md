# Checklist สำหรับย้าย Feature จาก Dart ไป Rust

[English](../migration_checklist.md)

การย้าย code ไป Rust ควรเริ่มจากปัญหาที่วัดได้ ไม่ใช่จากความต้องการ rewrite

## ก่อนย้าย

- ระบุ bottleneck หรือเหตุผลด้าน portability/security ให้ชัด
- เก็บ baseline latency, throughput, memory และ frame timing
- ระบุ input/output contract
- ประเมิน payload size และ copy cost
- ระบุ platform ที่ต้องรองรับ

## ออกแบบ Boundary

- ใช้ coarse-grained operation
- สร้าง Dart interface ก่อน
- ให้ generated bridge อยู่ใน adapter
- กำหนด error categories
- กำหนด cancellation semantics
- กำหนด buffer ownership
- ระบุ sync/async และ concurrency behavior

## ระหว่าง Implementation

- เขียน Rust core ให้ test ได้โดยไม่ผ่าน Flutter
- จำกัด `unsafe` ไว้ที่ boundary
- เพิ่ม structured errors
- เพิ่ม request/correlation ID ถ้าต้อง trace ข้าม boundary
- อย่า optimize zero-copy ก่อนมี benchmark

## Integration

- ใช้ fake engine ทดสอบ presentation
- เพิ่ม real bridge integration test
- ทดสอบ stale result และ cancellation race
- ทดสอบ dispose/lifecycle
- ตรวจว่า UI isolate ไม่ถูก block

## ก่อน Release

- Benchmark release build บนอุปกรณ์จริง
- ทดสอบ input ขนาดใหญ่/low-memory
- build ทุก shipping platform
- ตรวจ native binary packaging
- รัน smoke test
- ตรวจ telemetry/log privacy

## Rollout

ถ้าความเสี่ยงสูง:

- ใช้ feature flag
- เก็บ fallback implementation ชั่วคราว
- เพิ่ม telemetry รอบ native failure/latency
- rollout แบบเป็นช่วง

## หลัง Release

เปรียบเทียบกับ baseline เดิม:

- user-perceived latency ดีขึ้นหรือไม่
- crash/native failure เพิ่มหรือไม่
- memory peak เปลี่ยนอย่างไร
- battery/thermal behavior เปลี่ยนหรือไม่

ถ้า Rust compute เร็วขึ้นแต่ UX ไม่ดีขึ้น ให้กลับมาดู bridge/copy/scheduling แทนที่จะ optimize algorithm ต่ออย่างเดียว
