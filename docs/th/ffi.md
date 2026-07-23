# Raw `dart:ffi` — Mental Model สำหรับ Flutter Developer

[English](../ffi.md)

`dart:ffi` ทำให้ Dart Native เรียก native API ที่ expose C-compatible ABI ได้ เป็นทางเลือกที่ให้ control สูงสุด แต่ต้องรับผิดชอบเรื่อง ABI, pointer, memory ownership และ threading เองมากกว่าการใช้ generated bridge ระดับสูง

## Boundary

แนวคิดพื้นฐาน:

```text
Dart code
  ↓ FFI call
C ABI-compatible function
  ↓
Rust implementation
```

Rust สามารถ expose symbol ผ่าน `extern "C"` function ส่วน export attribute ที่ใช้จริงอาจขึ้นกับ Rust edition/toolchain และ build configuration ควรตรวจเอกสารปัจจุบันเมื่อตั้ง production binding

## ทำไมต้อง C ABI

Dart ไม่เข้าใจ Rust native ABI โดยตรง รวมถึง:

- Rust enum layout
- ownership model
- trait objects
- generics
- references/lifetimes

C-compatible ABI จึงเป็น common boundary ที่ทั้งสองฝั่งเข้าใจ

## เริ่มจาก Primitive ก่อน

FFI API ที่ง่ายที่สุดใช้ fixed-width integer และ float เช่น `i32`, `u64`, `f64`

เมื่อเริ่มส่ง:

- String
- Array/Buffer
- Callback
- Struct ซับซ้อน
- Long-lived object

คุณต้องมี explicit ownership contract

## คำถาม Ownership 4 ข้อ

ทุก pointer/buffer ต้องตอบให้ได้:

1. ใคร allocate
2. หลัง call ใครเป็น owner
3. ใคร free
4. valid นานเท่าใด

ถ้าข้อใดตอบไม่ชัด API ยังไม่พร้อม production

## String

Rust `String` ไม่ใช่ C string และ Dart `String` ก็ไม่ใช่ native C string

Raw FFI มักใช้:

- UTF-8 bytes + length
- C string ในกรณีที่เหมาะสม

ห้าม return pointer ไปยัง temporary Rust value ที่ถูก drop เมื่อ function จบ

## Buffer

งาน image/audio/binary ต้องวัดแยก:

- Dart → native copy
- Rust processing
- native → Dart copy

Algorithm 5 ms ที่ถูกล้อมด้วย copy 30 ms ไม่ใช่ feature 5 ms

## Blocking และ UI Isolate

Native call ไม่ได้ async อัตโนมัติเพียงเพราะ implementation เขียนด้วย Rust

ถ้า synchronous FFI call ถูกเรียกจาก UI isolate และใช้เวลานาน Flutter ยังเกิด jank ได้

ต้องออกแบบว่า CPU-heavy work ถูก schedule ที่ใด และ Dart API ที่คืน `Future` เชื่อมกับ native execution จริงอย่างไร

## Binding Generation ด้วย `ffigen`

สำหรับ C header-based API สามารถใช้ `ffigen` เพื่อ generate Dart FFI bindings ลด boilerplate ของ signature

แต่ `ffigen` ไม่ได้แก้ปัญหา:

- ownership
- lifetime
- cancellation
- threading
- error model

สิ่งเหล่านี้ยังเป็นหน้าที่ของ API design

## เมื่อใด Raw FFI เหมาะสม

ใช้ raw FFI เมื่อคุณต้องการ:

- ควบคุม ABI อย่างละเอียด
- integrate C API ที่มีอยู่แล้ว
- native surface เล็กและ stable
- runtime abstraction น้อย

ถ้าต้องเขียน type conversion, async plumbing, error mapping และ memory management จำนวนมาก generated bridge ระดับสูงอาจคุ้มกว่า

## แนวทาง Production

แยก FFI code ไว้ใน adapter module ขนาดเล็กและ review ง่าย

```text
Dart domain
  ↓
NativeEngine interface
  ↓
RawFfiNativeEngine
  ↓
dart:ffi bindings
  ↓
C ABI
  ↓
Rust safe wrapper
  ↓
Rust core
```

ฝั่ง Rust ให้ `unsafe` อยู่ใกล้ boundary ที่สุด แล้ว wrap ด้วย safe Rust API ภายใน

## Official Reference

https://dart.dev/interop/c-interop
