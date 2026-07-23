# Async, Cancellation และ Backpressure

[English](../async_cancellation.md)

แยก 3 เรื่องออกจากกัน: API เป็น async, งานรันที่ไหน, และงาน native หยุดได้จริงหรือไม่

การ cancel ฝั่ง Dart ไม่ได้หมายความว่า CPU work ใน Rust จะหยุดอัตโนมัติ ควรออกแบบ cooperative cancellation และใช้ request ID/latest-request-wins เพื่อทิ้งผลลัพธ์เก่าที่มาถึงช้า

สำหรับ progress stream ควร throttle/coalesce เพื่อไม่ส่ง event ถี่จน UI/state management ทำงานเกินจำเป็น
