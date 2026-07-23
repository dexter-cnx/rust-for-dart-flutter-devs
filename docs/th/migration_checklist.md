# Checklist สำหรับย้าย Feature จาก Dart ไป Rust

[English](../migration_checklist.md)

ก่อนย้ายให้ profile Dart implementation และกำหนด benefit ที่คาดหวัง

ออกแบบ boundary แบบ coarse-grained, error/cancellation/ownership ให้ชัด จากนั้นสร้าง pure Rust core + tests ก่อนทำ Dart adapter

ตรวจ correctness เทียบของเดิม, benchmark release mode บนอุปกรณ์จริง, ทดสอบ large input/cancellation และ build ทุก platform ที่จะส่งจริง
