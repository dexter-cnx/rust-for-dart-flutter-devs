# Security Checklist สำหรับ Flutter + Rust

[English](../security.md)

Safe Rust ช่วยป้องกัน memory-safety bug หลายประเภท แต่ native boundary ยังต้อง review อย่างจริงจัง

## จุดที่ต้อง Review

- ทุก `unsafe` block มี documented invariant
- pointer + length ถูก validate ก่อนสร้าง slice
- integer conversion ไม่ truncate length/offset เงียบ ๆ
- untrusted image/archive/binary input มี bounds/resource limits
- allocation size มี cap ป้องกัน resource exhaustion
- FFI string มี encoding/termination rule
- native callback ไม่ outlive Dart object ที่อ้างถึง
- secret ไม่อยู่ใน log/panic message
- dependency/toolchain update แบบ deliberate

## Unsafe Policy

ให้ `unsafe` เล็กและรวมอยู่ใน adapter/FFI module

ด้านในควร expose safe Rust API เพื่อให้ core domain ไม่ต้องรู้เรื่อง raw pointer

```text
raw FFI unsafe layer
  ↓
safe wrapper
  ↓
Rust domain/services
```

## Input Validation

อย่าเชื่อ input จาก Flutter โดยอัตโนมัติ โดยเฉพาะข้อมูลที่มาจากไฟล์/network

ตรวจ:

- length
- dimensions
- offset
- integer overflow
- maximum allocation

## Cryptography

ใช้ library ที่ผ่านการ review และ maintain ดี แทนเขียน primitive เอง

อย่า log key/secret และอย่าส่ง secret ผ่าน error string

## Dependency Supply Chain

Rust crate และ Dart package เป็นส่วนหนึ่งของ attack surface ควร pin/review update และใช้ vulnerability scanning ตามความเหมาะสมของ project
