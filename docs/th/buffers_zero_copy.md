# Buffer, Typed Data และ Zero-Copy

[English](../buffers_zero_copy.md)

สำหรับ image/audio/tensor ให้ทำ copy budget ของ pipeline ทั้งหมดก่อน optimize

เริ่มจาก owned-value API ที่เข้าใจง่าย แล้ว benchmark end-to-end หากพบว่าการ copy/transfer เป็น bottleneck จึงค่อยใช้กลไกลด copy ที่ bridge/SDK รองรับ

อย่าเรียก design ว่า zero-copy โดยไม่ตรวจ path จริงใน platform/version ที่รองรับ เพราะมักแลกกับ lifetime, pinning หรือ ownership constraint ที่ซับซ้อนขึ้น
