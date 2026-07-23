# การเชื่อม Riverpod / Cubit กับ Rust

[English](../state_management.md)

ให้ Notifier/Cubit depend on `NativeEngine` interface ไม่ใช่ generated bridge โดยตรง

Adapter รับผิดชอบ generated call, DTO mapping, error mapping, cancellation, progress และ tracing

ควร test state transition ด้วย fake engine รวมถึง success, failure, cancellation, stale result และ progress throttling
