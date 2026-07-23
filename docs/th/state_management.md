# การเชื่อม Riverpod และ Cubit กับ Rust

[English](../state_management.md)

## กฎหลัก: Presentation พึ่ง Interface

```dart
abstract interface class NativeEngine {
  Future<ProcessResult> process(ProcessRequest request);
  Future<void> cancel(String requestId);
}
```

`RustNativeEngine` implement interface นี้ด้วย generated bridge ส่วน test ใช้ `FakeNativeEngine`

Generated bridge imports ควรอยู่ใน adapter file เท่านั้น

## Riverpod

Expose interface ผ่าน provider:

```dart
final nativeEngineProvider = Provider<NativeEngine>((ref) {
  return RustNativeEngine();
});
```

จากนั้น `AsyncNotifier`/`Notifier` เป็นเจ้าของ request lifecycle

สำหรับ latest-request-wins ให้เก็บ generation/request ID และ ignore late result หลังมี request ใหม่หรือ provider ถูก dispose

ควรใช้ `ref.onDispose` เพื่อ trigger best-effort cancellation เมื่อ lifecycle เหมาะสม

## Cubit

Inject `NativeEngine` ผ่าน constructor

```dart
class ProcessCubit extends Cubit<ProcessState> {
  ProcessCubit(this.engine) : super(const ProcessInitial());

  final NativeEngine engine;
}
```

State ควร explicit เช่น:

```text
initial
processing(progress)
success(result)
failure(typedFailure)
```

ถ้า Cubit เป็น route-scoped ให้ `close()` trigger cancellation ตาม policy

## Progress

Adapter ควรแปลง native progress เป็น domain abstraction เช่น `Stream<Progress>`

ถ้า native ส่ง event ถี่ ให้ throttle/coalesce ก่อน emit UI state

## Error Mapping

Notifier/Cubit ไม่ควรจับ arbitrary bridge exceptions แล้ว parse string

Adapter map เป็น `EngineFailure` ก่อน จากนั้น presentation ตัดสินใจ localization/UI

## Testing

ใช้ fake engine และ deterministic completer ทดสอบ:

- initial → loading → success
- typed failure mapping
- cancellation on disposal
- stale result ignored
- progress throttling
- request ใหม่แทน request เก่า

## สิ่งที่ไม่ควรทำ

- import generated bridge ใน Widget
- สร้าง Rust engine singleton แบบ global โดยไม่คิด lifecycle
- ให้ Cubit รับ raw Rust error type
- emit progress ทุก event โดยไม่มี backpressure
