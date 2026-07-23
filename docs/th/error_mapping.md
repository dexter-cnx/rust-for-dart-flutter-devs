# Error Mapping ระหว่าง Rust และ Flutter

[English](../error_mapping.md)

UI ไม่ควร parse string error จาก Rust ให้สร้าง error category ที่เสถียร เช่น InvalidInput, Cancelled, UnsupportedFormat, ResourceExhausted, Io, Internal แล้ว map เป็น Dart failure

Rust เก็บรายละเอียด diagnostic สำหรับ log ส่วนข้อความที่ผู้ใช้เห็นให้ presentation/localization layer ของ Flutter เป็นผู้เลือก

`panic!` ควรถูกมองเป็น bug ไม่ใช่ flow ของ recoverable error
