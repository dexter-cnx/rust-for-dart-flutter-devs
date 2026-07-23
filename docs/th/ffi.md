# Raw `dart:ffi` สำหรับ Flutter Developer

[English](../ffi.md)

`dart:ffi` ใช้เรียก native API ผ่าน C-compatible ABI และให้การควบคุมสูง แต่ผู้พัฒนาต้องรับผิดชอบ contract ของ pointer, buffer, string, allocation/deallocation และการ bundle native library เองมากกว่า generated bridge

## คำถาม 4 ข้อที่ต้องตอบทุกครั้ง

1. ใคร allocate memory?
2. ใคร free memory?
3. pointer ใช้ได้นานถึงเมื่อไร?
4. call นี้ block thread/isolate ที่เรียกหรือไม่?

เริ่มจาก primitive และ owned buffer ก่อน แล้วค่อย optimize เมื่อ profiler ยืนยันว่า transfer/copy เป็น bottleneck
