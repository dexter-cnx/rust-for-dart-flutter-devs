# Buffer, Typed Data และการออกแบบ Zero-Copy

[English](../buffers_zero_copy.md)

Workload อย่าง image, audio, video, compression หรือ binary parsing มักส่งข้อมูลขนาดใหญ่ข้าม Dart ↔ Rust ต้นทุนของการ copy จึงอาจมากกว่า algorithm เอง

## เริ่มจาก Copy Budget

สำหรับ operation ที่ใช้ buffer ใหญ่ ให้เขียนเส้นทางข้อมูลก่อน:

```text
Dart bytes
  → bridge representation
  → Rust input
  → Rust output
  → bridge representation
  → Dart bytes
```

จำนวน physical copies จริงขึ้นกับ bridge, codec, platform และ version ที่ใช้อยู่ จึงต้อง benchmark implementation จริง ไม่ควรเดาจาก API signature อย่างเดียว

## ใช้ Coarse-Grained Pipeline

ดี:

```text
processImage(encodedBytes, options) -> processedBytes
```

แย่:

```text
for every pixel:
  Dart -> native call -> Dart
```

ถึง Rust จะประมวลผล pixel เร็ว แต่ค่า call overhead หลายล้านครั้งจะทำลาย performance ทั้งระบบ

## Ownership Contract

สำหรับ raw FFI ทุก pointer/buffer ต้องตอบได้ว่า:

1. ใคร allocate
2. หลัง call ใครเป็น owner
3. ใคร free
4. valid นานเท่าใด
5. length/capacity เก็บที่ไหน

Pointer ที่ไม่มี length และ ownership rule ถือว่า API contract ยังไม่สมบูรณ์

ห้าม allocate ด้วย allocator หนึ่งแล้ว free ด้วย allocator อีกฝั่งโดยไม่มี ABI contract ที่รองรับ

## Typed Data

Dart FFI สามารถทำงานกับ native memory และ Dart/Flutter รุ่นใหม่มี API บางเส้นทางที่ช่วยลด copy สำหรับ typed data ได้ แต่ behavior แตกต่างตาม SDK และ binding mechanism

ดังนั้น production rule คือ:

- pin SDK/toolchain range
- benchmark platform จริง
- test lifecycle
- อย่าเขียนเอกสารว่า zero-copy โดยไม่พิสูจน์ path จริง

## Zero-Copy ไม่ได้ฟรี

การลด copy มักแลกกับ:

- lifetime ที่เข้มงวดขึ้น
- ownership transfer
- memory pinning
- synchronization
- complexity ตอน dispose

ถ้า payload เล็ก การ copy ปกติอาจง่ายกว่าและเร็วพอ

## ตัวอย่างการคิด Copy Budget

สมมติภาพ 20 MB:

```text
Dart Uint8List 20 MB
→ copy เข้า native 20 MB
→ Rust สร้าง output 20 MB
→ copy กลับ Dart 20 MB
```

แม้ compute ใช้ 5 ms แต่ memory traffic และ allocation อาจกินเวลามากกว่า ดังนั้นต้องวัด end-to-end

## วิธีทำงานที่แนะนำ

1. เริ่มจาก API แบบ owned value ที่ง่ายและปลอดภัย
2. Benchmark release mode
3. แยก compute time กับ transfer time
4. หา hot path จริง
5. optimize เฉพาะจุดที่คุ้ม
6. เพิ่ม test ownership/lifecycle

## เมื่อใดควรพิจารณา Zero-Copy

เหมาะเมื่อ:

- payload ใหญ่มาก
- operation ถูกเรียกบ่อย
- profiler ชี้ว่า copy เป็น bottleneck
- lifetime contract สามารถอธิบายและ test ได้

ไม่ควรทำเพราะคำว่า zero-copy ฟังดูเร็วกว่าเสมอ
