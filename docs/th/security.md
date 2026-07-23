# Security Checklist

[English](../security.md)

Safe Rust ลด memory-safety bug ได้มาก แต่ FFI/native boundary ยังต้อง review

- จำกัดและ document `unsafe`
- validate pointer + length
- ป้องกัน integer truncation/overflow
- จำกัด resource สำหรับ untrusted input
- กำหนด string encoding
- ตรวจ callback lifetime
- ไม่ log secret/token/raw payload
- update dependencies/toolchain อย่างมีแผน
