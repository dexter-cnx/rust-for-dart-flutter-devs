# Async, Cancellation และ Backpressure

[English](../async_cancellation.md)

การที่ API ฝั่ง Dart คืน `Future` ไม่ได้หมายความว่างาน native ถูกยกเลิกได้ หรือไม่ block UI โดยอัตโนมัติ ต้องแยกสามเรื่องนี้ออกจากกัน

## สามเรื่องที่ต้องแยกให้ชัด

- **Async API** — Dart ได้ abstraction แบบ `Future` หรือ `Stream`
- **Execution location** — งานจริงรันบน UI isolate/thread หรือ worker/runtime อื่น
- **Cancellation** — งาน native ที่กำลังรันสามารถหยุดแบบ cooperative ได้จริงหรือไม่

API อาจเป็น async แต่ยัง queue CPU work จำนวนมากจนเครื่องหน่วงได้ และการที่ Dart consumer เลิกสนใจ `Future` ก็ไม่ได้ทำให้ Rust computation หยุดเอง

## Cooperative cancellation

งาน Rust ที่ใช้เวลานานควรมี cancellation checkpoint ในจุดที่เหมาะสม

```rust
for chunk in chunks {
    if cancelled.load(Ordering::Relaxed) {
        return Err(EngineError::Cancelled);
    }
    process(chunk)?;
}
```

Token อาจเป็น:

- atomic flag
- entry ใน request registry
- cancellation primitive ของ async runtime

ไม่จำเป็นต้องตรวจทุก instruction แต่ควรตรวจที่ boundary ของงานย่อย เช่น ทุก chunk, ทุก frame หรือทุก batch

## Latest-request-wins

เหมาะกับ search, preview, filter และ operation ที่ request ใหม่ทำให้ request เก่าไม่สำคัญแล้ว

ลำดับที่แนะนำ:

1. Dart เพิ่ม generation/request ID
2. เริ่ม native work พร้อม ID นั้น
3. พยายาม cancel request ก่อนหน้า
4. เมื่อผลลัพธ์กลับมา ตรวจว่า ID ยังเป็น current หรือไม่
5. ถ้าไม่ใช่ ให้ ignore ผลเก่า

แม้มี cancellation ก็ยังต้อง ignore stale result เพราะ cancellation อาจ race กับ completion ได้

ตัวอย่าง mental model:

```text
request #10 starts
request #11 starts → cancel #10
#10 finishes late → ignore
#11 finishes → accept
```

## อย่าสับสน Cancellation กับ Disposal

เมื่อ route ถูก dispose คุณต้องกำหนด behavior ให้ชัดว่า:

- cancel และรอ completion
- best-effort cancel แล้วไม่รอ
- detach งานและ ignore result
- เก็บ engine แบบ process-wide ต่อไป

ไม่มีคำตอบเดียวสำหรับทุก feature แต่ lifecycle ต้องเป็น intentional design

## Backpressure

ถ้า Rust ส่ง progress event ถี่เกินไป Dart อาจเสียเวลาสร้าง state/widget update มากกว่าทำงานจริง

แนวทาง:

- coalesce progress
- throttle ให้เหลือ rate ที่ UI ใช้ได้จริง
- ส่งเฉพาะ milestone ที่มีความหมาย
- ใช้ bounded queue เมื่อมี producer เร็วกว่าผู้บริโภค

ตัวอย่าง การส่ง progress ทุก pixel แย่กว่าการส่งทุก 1–5% อย่างมาก

## Streams และ long-lived operations

สำหรับ stream จาก Rust ให้กำหนด:

- ใครเป็น owner ของ subscription
- cancel subscription แล้ว native producer หยุดหรือไม่
- reconnect/retry semantics
- buffer size
- error termination behavior

## Shutdown

ตอน app shutdown หรือ engine dispose ควรกำหนดว่า:

- ปิด worker threads หรือ runtime หรือไม่
- flush pending data หรือไม่
- request ที่ยังค้างถูก cancel อย่างไร
- callback ฝั่ง Dart จะยังถูกเรียกหลัง dispose หรือไม่

## Test cases ที่ควรมี

- cancel ก่อนเริ่ม compute
- cancel ระหว่าง compute
- complete พร้อมกับ cancel
- request เก่ากลับมาหลัง request ใหม่
- dispose controller ระหว่างงานกำลังรัน
- progress burst ที่เร็วกว่าฝั่ง UI

จุดสำคัญคือ cancellation เป็น **protocol ระหว่าง Dart และ Rust** ไม่ใช่แค่การเรียก method ชื่อ `cancel()`
