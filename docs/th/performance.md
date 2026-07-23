# คู่มือ Performance

[English](../performance.md)

อย่าย้ายโค้ดไป Rust เพียงเพราะ Rust เร็วกว่าใน benchmark บางชนิด ให้ profile pipeline จริงก่อน

งานที่มักเหมาะ: image/audio processing, codecs, binary parsing, cryptography ผ่าน library ที่เชื่อถือได้, algorithm CPU-heavy และ reusable native engine

งานที่มักไม่คุ้ม: UI composition, REST CRUD ทั่วไป และ business logic ที่เปลี่ยนบ่อยโดยไม่มี performance/native reuse benefit

วัดทั้ง compute time, serialization/copy, allocation, peak memory และผลต่อ frame responsiveness ใน release mode บนอุปกรณ์จริง
