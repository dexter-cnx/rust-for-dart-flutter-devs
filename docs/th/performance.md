# คู่มือ Performance

[English](../performance.md)

## อย่า Optimize จากภาษา ให้ Optimize ทั้ง Pipeline

วัดเส้นทางเต็ม:

```text
User action
→ Dart preprocessing
→ bridge encoding/copy
→ Rust queue/wait
→ Rust compute
→ bridge return/copy
→ Dart decoding
→ widget update
```

การเปลี่ยน Dart algorithm เป็น Rust แล้ว compute เร็วขึ้น 5 เท่าอาจไม่ช่วย UX ถ้าต้นทุน bridge และ allocation สูง

## Workload ที่ Rust มักคุ้ม

- CPU-heavy image transforms
- compression/decompression
- complex parser
- large deterministic simulation
- crypto ผ่าน library ที่เหมาะสม
- reusable cross-platform native engine

## Workload ที่มักไม่คุ้มย้าย

- JSON model ธรรมดา
- CRUD orchestration
- navigation
- form validation
- networking ทั่วไป
- UI state management

## Large Buffers

Buffer ขนาดใหญ่เป็นจุดที่ bridge design มีผลมาก

ลด conversion/copy ที่ซ้ำโดยไม่จำเป็น แต่ใช้ zero-copy เฉพาะเมื่อเข้าใจ lifecycle guarantee และ profiler ยืนยันว่าคุ้ม

## Benchmark ให้ถูก

- ใช้ release/optimized build
- payload size สมจริง
- ระบุ warm-up behavior
- ระบุ device/hardware
- ระบุ bridge/runtime version

Benchmark ที่ไม่มีเงื่อนไขเหล่านี้เปรียบเทียบข้ามเวลาได้ยาก

## UI Responsiveness

Synchronous native call 50 ms อาจเร็วในมุม throughput แต่ยังทำให้ UI jank ถ้า block UI isolate

Performance ต้องรวม responsiveness

## Memory

วัด memory peak และ allocation churn ด้วย โดยเฉพาะ pipeline ที่ duplicate large buffers

## Thermal/Battery

งาน CPU-heavy ที่เร็วขึ้นแต่ใช้ทุก core ต่อเนื่องอาจกระทบ battery/thermal ควรวัด workload จริงบนอุปกรณ์เป้าหมาย

## Optimization Order

1. หา bottleneck ด้วย profiler
2. ลด algorithmic complexity
3. ลด unnecessary boundary calls
4. ลด copies/allocations ที่วัดแล้วว่าแพง
5. tune concurrency
6. ค่อยพิจารณา advanced zero-copy

อย่าเริ่มที่ข้อ 6
