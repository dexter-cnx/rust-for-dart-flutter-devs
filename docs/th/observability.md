# Observability ที่ Dart ↔ Rust Boundary

[English](../observability.md)

เมื่อ feature ข้ามสอง runtime การ debug จาก log ฝั่งเดียวมักไม่พอ จึงควรออกแบบ correlation ตั้งแต่แรก

## สิ่งที่ควรวัด

สำหรับ native operation สำคัญ ให้เก็บ:

- operation name
- correlation/request ID
- input/output size bucket
- total duration
- compute duration ถ้าแยกได้
- serialization/transfer duration ถ้าแยกได้
- outcome category
- cancellation count
- queue/concurrency saturation

## Logging Ownership

Rust log สิ่งที่เกี่ยวกับ native/domain diagnostics

Dart log user-flow, route และ presentation context

ใช้ correlation ID เชื่อมสองฝั่งแทนการ duplicate context ทุกอย่าง

ตัวอย่าง:

```text
Dart: request_id=42 open preview
Rust: request_id=42 decode started
Rust: request_id=42 decode finished 18ms
Dart: request_id=42 state=success total=31ms
```

## Privacy

ควร log metadata มากกว่าข้อมูล payload จริง

หลีกเลี่ยง:

- user content
- access token/API key
- full file path
- binary dump
- raw image/audio

ยกเว้น diagnostic mode ที่ออกแบบด้าน security โดยเฉพาะ

## Performance Budgets

ตั้ง budget รอบ outcome ที่ผู้ใช้เห็น เช่น:

- frame responsiveness
- preview latency
- import duration
- memory peak

อย่าตั้ง KPI เฉพาะ Rust compute time เพราะผู้ใช้รับรู้ทั้ง pipeline

## Production Troubleshooting

เมื่อ latency สูง ควรแยกดู:

```text
queue wait
bridge transfer
compute
return transfer
Dart rebuild
```

Observability ที่ดีช่วยตอบได้ว่า bottleneck อยู่ฝั่งใดก่อนเริ่ม optimize
