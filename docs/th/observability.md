# Observability

[English](../observability.md)

เก็บ operation name, request/correlation ID, payload-size bucket, duration, outcome, cancellation และ queue saturation เมื่อเกี่ยวข้อง

แยก compute time กับ transfer/serialization time เมื่อทำได้ เพื่อเห็น bottleneck จริง

หลีกเลี่ยง log ข้อมูลผู้ใช้, key, token, full path หรือ raw binary
