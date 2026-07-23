# Production Readiness Checklist

[English](../PRODUCTION_CHECKLIST.md)

- [ ] Rust API เป็น coarse-grained และมีแนวทาง versioning ชัดเจน
- [ ] generated bridge types ถูกซ่อนหลัง Dart adapter
- [ ] ระบุ ownership และขนาดสูงสุดของ buffer
- [ ] งาน native หนักไม่ block เส้นทาง UI แบบ synchronous
- [ ] ทดสอบ cancellation และ stale-result behavior
- [ ] error ถูก map เป็น Dart failure ที่เสถียร
- [ ] progress stream มี throttling/backpressure
- [ ] CI ผ่าน Rust fmt, Clippy และ tests
- [ ] consuming Flutter app ผ่าน analyze/tests
- [ ] มี policy ตรวจ generated-code drift
- [ ] smoke test release build Android/iOS
- [ ] benchmark workload สำคัญใน release mode บนอุปกรณ์ที่เป็นตัวแทน
- [ ] review `unsafe` และ raw pointer โดยเฉพาะ
- [ ] document compatibility ของ toolchain และ bridge
- [ ] log ไม่เก็บ secret หรือ raw user payload
