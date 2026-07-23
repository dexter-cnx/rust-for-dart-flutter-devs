# Rust for Dart & Flutter Developers — คู่มือฉบับภาษาไทย

> **Default language: English** — [Read the English README](README.md)

คู่มือนี้เขียนสำหรับ Flutter Developer ที่มีประสบการณ์และรู้จัก Dart อยู่แล้ว เป้าหมายไม่ใช่สอน programming ใหม่ตั้งแต่ศูนย์ แต่ใช้ mental model ที่คุณคุ้นจาก Dart/Flutter แล้วเชื่อมไปยัง Rust พร้อมอธิบายจุดที่ “เหมือน”, “ดูเหมือนแต่จริง ๆ ไม่เหมือน” และ “เป็นแนวคิดใหม่ทั้งหมด”

เนื้อหาครอบคลุมตั้งแต่ syntax พื้นฐานของ Rust ไปจนถึง production integration ระหว่าง Flutter ↔ Rust ผ่าน `dart:ffi` และ `flutter_rust_bridge` รวมถึง ownership, borrowing, lifetimes, async, concurrency, cancellation, error mapping, large buffers, testing, CI และ release architecture

---

## Runnable companion

ตัวอย่าง Rust อยู่ใน `examples/` และออกแบบให้รันแยกได้:

```bash
cargo run --example 01_variables
cargo run --example 07_ownership
cargo run --example 14_concurrency
```

ลำดับตัวอย่าง:

| Example | เนื้อหา |
|---|---|
| `01_variables` | `let`, `mut`, shadowing |
| `02_strings` | `String` และ `&str` |
| `03_functions_closures` | functions, expressions, closures |
| `04_option` | `Option<T>` |
| `05_structs` | `struct`, `impl` |
| `06_enums_match` | enum และ pattern matching |
| `07_ownership` | move, copy, clone |
| `08_borrowing` | `&T`, `&mut T` |
| `09_lifetimes` | lifetime annotation |
| `10_result` | `Result<T, E>` และ `?` |
| `11_traits` | traits |
| `12_iterators` | iterators |
| `13_async` | async/await |
| `14_concurrency` | threads, `Arc`, `Mutex` |

---

## สรุปใน 30 วินาที

สำหรับ Dart/Flutter Developer, Rust จะมีสองส่วน

ส่วนแรกคุ้นเคย:

- type inference
- immutability by default
- null safety ผ่าน type system
- enum + pattern matching
- generics
- closures
- async/await
- collection transformations

ส่วนที่สองคือ mental model ใหม่:

- ไม่มี Garbage Collector
- value มี owner
- การ assign อาจเป็น move
- reference ต้องผ่าน borrowing rules
- lifetime ของ reference ถูก compiler ตรวจ
- mutable shared state ต้อง explicit

ตารางเทียบ mental model:

| Dart / Flutter | Rust |
|---|---|
| `final` | `let` |
| `var` | `let mut` |
| `String?` | `Option<String>` |
| `throw` / `try-catch` | `Result<T, E>` |
| class | `struct` + `impl` |
| abstract class / interface | `trait` |
| sealed class / enhanced enum | `enum` |
| `switch` pattern matching | `match` |
| `Future<T>` | Rust `Future` |
| Isolate | thread/task — mental model ต่างกัน |
| GC | ownership + borrowing |

จุดที่ควรใช้เวลามากที่สุดคือ **Ownership → Borrowing → Lifetimes** ไม่ใช่ syntax

---

# Part I — Rust ผ่านมุมมองของ Dart

## 1. Variables: `final` / `var` → `let` / `let mut`

Dart:

```dart
final x = 5;
var y = 10;
y = 20;
```

Rust:

```rust
let x = 5;
let mut y = 10;
y = 20;
```

Rust immutable by default เหมือนการเลือกใช้ `final` เป็น default ใน Dart แต่ Rust บังคับที่ binding level ชัดกว่า

### Shadowing

Rust นิยม shadowing:

```rust
let value = "42";
let value = value.parse::<i32>().unwrap();
```

นี่ไม่ใช่ mutation แต่เป็น binding ใหม่ และเปลี่ยน type ได้

`const` ใช้กับ compile-time constant และต้องระบุ type:

```rust
const MAX_ITEMS: usize = 100;
```

---

## 2. Built-in types

| Dart | Rust |
|---|---|
| `int` | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` |
| ไม่มี unsigned int แบบ core usage ทั่วไป | `u8` ... `u128`, `usize` |
| `double` | `f32`, `f64` |
| `bool` | `bool` |
| `String` | `String` และ `&str` |
| `List<T>` | `Vec<T>`, array, slice |
| `Map<K,V>` | `HashMap<K,V>` |
| record | tuple |
| `Never` | `!` |

Rust ให้ความสำคัญกับขนาด integer และ signedness เพราะเป็นภาษา systems programming และ ABI/memory layout มีผลจริง

`usize` ใช้บ่อยกับ index และ length เพราะขนาดสัมพันธ์กับ architecture

---

## 3. Strings: `String` ไม่ได้มีแค่รูปแบบเดียว

Dart Developer คุ้นกับ `String` เป็น object เดียวที่ GC ดูแล

Rust ใช้สองรูปแบบหลัก:

- `String` — owned, heap allocated, growable
- `&str` — borrowed string slice

```rust
let borrowed: &str = "hello";
let owned: String = String::from("hello");
```

Guideline ทั่วไป:

- รับ parameter เป็น `&str` เมื่อแค่อ่าน
- return `String` เมื่อ function สร้างและส่ง ownership ออกไป

Rust ไม่อนุญาต index string ด้วย `s[0]` เพราะ UTF-8 ทำให้ byte index, Unicode scalar และ grapheme ไม่ใช่สิ่งเดียวกัน

---

## 4. Functions และ Expressions

Dart:

```dart
int add(int a, int b) {
  return a + b;
}
```

Rust:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

บรรทัดสุดท้ายที่ไม่มี `;` คือค่าของ expression

```rust
let value = {
    let x = 10;
    x * 2
};
```

`value` เท่ากับ `20`

Closure:

```rust
let double = |x| x * 2;
```

Syntax คล้าย lambda ของ Dart แต่ ownership ของ captured values เป็นเรื่องสำคัญกว่ามาก

---

## 5. Null safety: `T?` → `Option<T>`

Dart:

```dart
String? name;
```

Rust:

```rust
let name: Option<String> = None;
```

`Option<T>` มีสอง variant:

```rust
Some(value)
None
```

ตัวอย่าง:

```rust
let length = name.as_ref().map(|value| value.len());
```

เทียบแนวคิด:

| Dart | Rust |
|---|---|
| `value?.foo()` | `value.map(...)` / `as_ref().map(...)` |
| `value ?? fallback` | `unwrap_or(fallback)` |
| `value!` | `unwrap()` — panic ได้ |
| null check | `if let Some(x)` / `match` |

อย่าใช้ `unwrap()` เป็น replacement ของ `!` แบบไม่คิด ควรใช้เมื่อ invariant รับประกันจริงหรือใน prototype/test

---

## 6. Classes → `struct` + `impl`

Dart:

```dart
class User {
  User(this.name, this.age);

  final String name;
  int age;

  void birthday() => age++;
}
```

Rust:

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}
```

`&self` หมายถึง borrow แบบอ่าน

`&mut self` หมายถึง mutable borrow

`self` หมายถึง method consume ownership ของ object

ความแตกต่างนี้เป็นส่วนหนึ่งของ API contract ไม่ใช่ syntax decoration

---

## 7. Enhanced enums / sealed modeling → Rust enums

Rust enum เป็นเครื่องมือหลักในการ model state ที่มีหลายรูปแบบ

```rust
enum LoadState {
    Idle,
    Loading,
    Success(String),
    Failure { code: u32, message: String },
}
```

คล้าย sealed class ของ Dart แต่ concise กว่าและ variant เก็บข้อมูลได้โดยตรง

ใช้คู่กับ `match`:

```rust
match state {
    LoadState::Idle => {}
    LoadState::Loading => {}
    LoadState::Success(data) => println!("{data}"),
    LoadState::Failure { code, message } => {
        println!("{code}: {message}");
    }
}
```

Compiler บังคับ exhaustive matching

---

# Part II — Mental Model ที่ใหม่จริง

## 8. Ownership: แนวคิดที่ Dart Developer ไม่มี

Dart ใช้ GC ดังนั้น object สามารถมี reference หลายจุดและ runtime ตัดสินใจว่าเมื่อใดควร free

Rust ไม่มี GC โดย default แต่ใช้ ownership

กฎพื้นฐาน:

1. แต่ละ value มี owner
2. เมื่อ owner ออกจาก scope value ถูก drop
3. ownership สามารถ move ได้

```rust
let a = String::from("hello");
let b = a;
```

หลังบรรทัดนี้ `a` ใช้ไม่ได้ เพราะ ownership ถูก move ไป `b`

### Move vs Copy vs Clone

Primitive บางชนิด implement `Copy`:

```rust
let a = 10;
let b = a;
println!("{a} {b}");
```

แต่ heap-owned type อย่าง `String` มัก move

ถ้าต้องการ duplicate จริง:

```rust
let a = String::from("hello");
let b = a.clone();
```

`clone()` มี cost ดังนั้นใช้ด้วยความเข้าใจ แต่ตอนเรียนไม่จำเป็นต้องกลัวการ clone จน API ซับซ้อนเกินไป

---

## 9. Borrowing: ใช้ค่าโดยไม่รับ ownership

แทนที่จะ move value เข้า function สามารถ borrow ได้

```rust
fn length(value: &String) -> usize {
    value.len()
}
```

หรือดีกว่าสำหรับ string API:

```rust
fn length(value: &str) -> usize {
    value.len()
}
```

Mutable borrow:

```rust
fn append(value: &mut String) {
    value.push('!');
}
```

กฎสำคัญ:

- มี immutable borrow (`&T`) หลายอันได้
- หรือ mutable borrow (`&mut T`) หนึ่งอัน
- แต่ไม่ใช้ทั้งสองแบบพร้อมกันในช่วงที่ overlap

Mental model คือ **aliasing XOR mutability**

นี่เป็นหนึ่งในเหตุผลที่ Rust ป้องกัน data race ได้ตั้งแต่ compile time

---

## 10. Lifetimes

Lifetime ไม่ได้หมายถึงการกำหนดอายุ object แบบ manual

มันเป็นวิธีที่ compiler ใช้พิสูจน์ว่า reference จะไม่อยู่ยาวกว่าข้อมูลที่อ้างถึง

ส่วนใหญ่ compiler infer ให้

ตัวอย่างที่ต้อง annotation:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

สำหรับ Flutter integration ควรหลีกเลี่ยงการออกแบบ public boundary ที่ expose lifetime-sensitive references ข้าม Dart ↔ Rust

Boundary DTO แบบ owned value มักง่ายและปลอดภัยกว่า

---

## 11. Error handling: Exceptions → `Result<T, E>`

Dart:

```dart
try {
  final value = await load();
} catch (e) {
  ...
}
```

Rust ใช้ `Result` สำหรับ recoverable errors:

```rust
fn load() -> Result<String, EngineError> {
    ...
}
```

มีสอง variant:

```rust
Ok(value)
Err(error)
```

`?` operator ช่วย propagate error:

```rust
fn load_config() -> Result<Config, EngineError> {
    let text = read_file()?;
    let config = parse(&text)?;
    Ok(config)
}
```

สำหรับ Flutter boundary ควร map Rust error เป็น stable category แล้วค่อย map เป็น Dart domain failure

---

## 12. Interfaces และ Mixins → Traits

Trait เป็น abstraction หลักของ Rust

```rust
trait Processor {
    fn process(&self, input: &[u8]) -> Vec<u8>;
}
```

```rust
struct FastProcessor;

impl Processor for FastProcessor {
    fn process(&self, input: &[u8]) -> Vec<u8> {
        input.to_vec()
    }
}
```

Trait ทำหน้าที่คล้าย:

- interface
- generic constraint
- extension behavior บางรูปแบบ

Rust ไม่มี class inheritance แบบ OOP ดั้งเดิม จึงนิยม composition + traits

---

## 13. Generics

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

Trait bounds:

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{value}");
}
```

Rust generics มัก monomorphize ตอน compile ทำให้ได้ zero-cost abstraction ในหลายกรณี แต่ binary size และ compile time ก็เป็น trade-off

---

## 14. Collections และ Iterators

Dart:

```dart
final result = values
    .where((x) => x.isEven)
    .map((x) => x * x)
    .toList();
```

Rust:

```rust
let result: Vec<i32> = values
    .iter()
    .filter(|x| **x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

สาม method สำคัญ:

- `.iter()` — borrow
- `.iter_mut()` — mutable borrow
- `.into_iter()` — consume

สิ่งนี้เชื่อมกับ ownership โดยตรง

---

# Part III — Async, Isolates และ Concurrency

## 15. `Future` → Rust `Future`

Dart:

```dart
Future<String> fetch() async {
  return 'data';
}
```

Rust:

```rust
async fn fetch() -> String {
    "data".to_string()
}
```

แต่ Rust standard library ไม่ได้ bundle async runtime แบบเดียวกับ Dart event loop มักใช้ runtime อย่าง Tokio ใน backend/native scenarios

สิ่งที่ต้องเข้าใจคือ:

- async syntax
- runtime/executor
- CPU-bound vs I/O-bound
- cancellation

เป็นคนละเรื่องกัน

---

## 16. Dart Isolates vs Rust Threads/Tasks

Dart isolate มี isolated heap และสื่อสารด้วย message passing เป็นหลัก

Rust thread สามารถ share memory ได้เมื่อ type ผ่าน safety constraints เช่น `Send`/`Sync`

ตัวอย่าง shared mutable state:

```rust
use std::sync::{Arc, Mutex};

let value = Arc::new(Mutex::new(0));
```

Flutter Developer ไม่ควร map mental model แบบ:

```text
Dart Isolate == Rust Thread
```

เพราะ semantics ต่างกัน

Rust ownership/type system อนุญาต shared memory แบบ controlled ขณะที่ Dart isolate เน้น memory isolation

---

# Part IV — Project Structure และ Tooling

## 17. Packages: `pubspec.yaml` → `Cargo.toml`

Rust package เรียกว่า crate

`Cargo.toml` ทำหน้าที่คล้าย package manifest/build configuration หลายส่วน

```toml
[package]
name = "my_engine"
version = "0.1.0"
edition = "2021"

[dependencies]
```

สำหรับ application ควร commit `Cargo.lock` เพื่อ reproducible dependency graph

`target/` เป็น build artifacts และควร ignore

---

## 18. Tooling crash course

| Dart / Flutter | Rust |
|---|---|
| `flutter create` / `dart create` | `cargo new` |
| `flutter run` | `cargo run` |
| `flutter test` | `cargo test` |
| `dart format` | `cargo fmt` |
| `flutter analyze` | `cargo clippy` + compiler |
| pub.dev | crates.io |

คำสั่งที่ควรใช้บ่อย:

```bash
cargo check
cargo fmt
cargo clippy
cargo test
cargo build --release
```

`cargo check` เร็วกว่าการ build binary เต็มและเหมาะกับ feedback loop ระหว่างพัฒนา

---

# Part V — Flutter ↔ Rust Integration

## 19. เมื่อใด Flutter Developer ควรใช้ Rust

เหมาะเมื่อมี workload เช่น:

- image/audio/video processing
- compression
- parser ซับซ้อน
- crypto ผ่าน mature library
- simulation
- reusable cross-platform native core
- CPU-heavy deterministic algorithm

ไม่ควรย้ายเพียงเพราะ “Rust เร็วกว่า Dart”

UI, navigation, form, CRUD orchestration และ state presentation ส่วนใหญ่ควรอยู่ Flutter

---

## 20. Integration option A: `dart:ffi`

Mental model:

```text
Dart
  ↓
C ABI
  ↓
Rust extern "C"
```

ข้อดี:

- control ABI สูง
- dependency abstraction ต่ำ
- เหมาะกับ C API เดิม

ข้อแลกเปลี่ยน:

- pointer ownership
- memory allocation/free
- string conversion
- callback lifetime
- async plumbing

ต้องออกแบบเองอย่างระมัดระวัง

อ่านรายละเอียด: [docs/th/ffi.md](docs/th/ffi.md)

---

## 21. Integration option B: `flutter_rust_bridge`

FRB ช่วย generate Dart ↔ Rust glue และลด handwritten FFI boilerplate

Workflow ทั่วไป:

```bash
cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen integrate
flutter_rust_bridge_codegen generate
```

Current v2 tooling ใช้ Cargokit เป็น default integration backend และมี Native Assets backend สำหรับ SDK ที่รองรับ

Generated bridge ไม่ควรถูกเรียกตรงจาก Widget

ควรมี adapter:

```text
Widget / Cubit / Riverpod
  ↓
NativeEngine interface
  ↓
RustNativeEngine
  ↓
FRB generated API
  ↓
Rust API
```

อ่านรายละเอียด: [docs/th/flutter_rust_bridge.md](docs/th/flutter_rust_bridge.md)

---

## 22. Boundary Design: ทักษะ Integration ที่สำคัญที่สุด

Bridge ที่ดีควร coarse-grained

ดี:

```text
processImage(bytes, options)
```

แย่:

```text
setPixel(x, y, value) x millions
```

Boundary type ควร stable และ owned ชัดเจน

สำหรับทุก method ให้ตอบ:

- input/output คืออะไร
- payload ใหญ่แค่ไหน
- sync หรือ async
- cancel ได้หรือไม่
- thread-safe หรือไม่
- error category อะไร
- buffer ใครเป็น owner

---

## 23. Memory Ownership ข้าม Boundary

Raw pointer ต้องมี contract:

1. ใคร allocate
2. ใคร owns หลัง call
3. ใคร free
4. valid นานเท่าใด
5. length/capacity อยู่ที่ไหน

ถ้าตอบไม่ได้ API ยังไม่ production-ready

ใน high-level bridge แม้ memory safety ง่ายขึ้น แต่ large buffer copy cost ยังต้องวัด

---

## 24. CPU-heavy Work และ Flutter UI Isolate

Rust ไม่ได้ทำให้ UI smooth อัตโนมัติ

ถ้า Dart เรียก blocking native function บน UI isolate ก็ยัง jank ได้

ต้องแยก:

- Dart API async หรือไม่
- native work รันที่ไหน
- queue/concurrency limit
- cancellation

และวัด Flutter frame timing จริง

---

## 25. Mapping Flutter Architecture ไป Rust

ตัวอย่าง:

```text
presentation/
  Cubit / Notifier

domain/
  ImageProcessor interface
  ProcessImageUseCase

data/
  RustImageProcessor
  generated bridge

rust/
  api/
  domain/
  services/
```

Domain layer ไม่ควร import generated bridge

---

# Part VI — Production Flutter ↔ Rust

## 26. Production Boundary Checklist

ก่อน release ต้องตอบได้ว่า:

- ownership ชัดหรือไม่
- error stable หรือไม่
- cancel ได้หรือไม่
- stale result ป้องกันหรือไม่
- buffer copy budget เท่าใด
- execution location อยู่ที่ไหน
- lifecycle ตอน dispose เป็นอย่างไร
- metrics มี correlation ID หรือไม่

รายละเอียด: [docs/th/PRODUCTION_CHECKLIST.md](docs/th/PRODUCTION_CHECKLIST.md)

---

## 27. Recommended Production Layering

```text
Presentation
  ↓
Domain abstraction
  ↓
Rust adapter
  ↓
Generated bridge
  ↓
Rust public API
  ↓
Rust domain/services
```

Rust ควรเป็น replaceable engine ไม่ใช่ dependency ที่กระจายอยู่ทุก Widget

รายละเอียด: [docs/th/production_architecture.md](docs/th/production_architecture.md)

---

## 28. Cancellation และ Latest-request-wins

Cancel และ ignore stale result เป็นคนละชั้น

```text
request #10 starts
request #11 starts → cancel #10
#10 returns late → ignore
#11 returns → accept
```

เพราะ cancellation สามารถ race กับ completion ได้

Rust algorithm ยาวควรมี cooperative checkpoints

รายละเอียด: [docs/th/async_cancellation.md](docs/th/async_cancellation.md)

---

## 29. Large Buffers และ Copy Budgets

วาด data path:

```text
Dart Uint8List
→ bridge
→ Rust
→ output
→ bridge
→ Dart
```

แล้ววัด physical copies จริง

Zero-copy ไม่ใช่ default optimization เสมอ เพราะเพิ่ม lifetime/ownership complexity

รายละเอียด: [docs/th/buffers_zero_copy.md](docs/th/buffers_zero_copy.md)

---

## 30. Stable Error Mapping

Rust public API ควรคืน stable category

```text
EngineError::InvalidInput
EngineError::Cancelled
EngineError::UnsupportedFormat
EngineError::Internal
```

Adapter map ไป Dart failure

Presentation map failure ไป localized message

รายละเอียด: [docs/th/error_mapping.md](docs/th/error_mapping.md)

---

## 31. Riverpod / Cubit Integration

State management ควร depend on:

```dart
abstract interface class NativeEngine {}
```

ไม่ใช่ generated bridge

จึง test ด้วย fake ได้ และเปลี่ยน native implementation ได้

รายละเอียด: [docs/th/state_management.md](docs/th/state_management.md)

---

## 32. Testing Pyramid

```text
             Real device / E2E
           Bridge integration test
       Dart adapter + state tests
         Rust boundary API tests
      Rust pure unit/property tests
```

ชั้นล่างเร็วและเยอะ ชั้นบนแพงแต่ตรวจ packaging/runtime behavior

รายละเอียด: [docs/th/testing_ci.md](docs/th/testing_ci.md)

---

## 33. CI Quality Gates

Baseline:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

และควร build ทุก shipping platform อย่างน้อยหนึ่ง configuration

---

## 34. Packaging และ Release

ต้อง pin:

- Flutter/Dart SDK
- Rust toolchain policy
- FRB runtime/codegen versions
- Android NDK/minSdk/ABI
- Apple deployment targets

`Cargo.lock` ควร commit สำหรับ application

`target/` ควร ignore

รายละเอียด: [docs/th/packaging_release.md](docs/th/packaging_release.md)

---

## 35. Security และ Native-code Review

Safe Rust ไม่ได้ทำให้ boundary ปลอดภัยโดยอัตโนมัติ

Review:

- `unsafe`
- pointer/length
- integer conversion
- allocation limits
- untrusted binary input
- callback lifetime
- secret leakage

รายละเอียด: [docs/th/security.md](docs/th/security.md)

---

## 36. Observability และ Performance Tracing

ใช้ request/correlation ID เชื่อม log ระหว่าง Dart กับ Rust

วัด:

- total latency
- bridge transfer
- queue wait
- compute
- output transfer
- cancellation

รายละเอียด: [docs/th/observability.md](docs/th/observability.md)

---

## 37. Production Sample Architecture

ดู `production_sample/`

แนวคิดหลัก:

```text
NativeEngine
  ↑
RustNativeEngine
  ↓
generated bridge
  ↓
Rust API
```

มีตัวอย่าง:

- RequestCoordinator
- latest-request-wins
- cooperative cancellation
- Riverpod integration sketch
- Cubit integration sketch

ตัวอย่างนี้ตั้งใจแสดง dependency direction มากกว่าจะเป็น complete application UI

---

# Part VII — Dart → Rust Idiom Cheat Sheet

| Dart | Rust |
|---|---|
| `final x = 5` | `let x = 5;` |
| `var x = 5` | `let mut x = 5;` |
| `String?` | `Option<String>` |
| `value ?? fallback` | `unwrap_or(fallback)` |
| `value!` | `unwrap()` / `expect()` |
| sealed class | `enum` |
| interface | `trait` |
| class data | `struct` |
| methods | `impl` |
| `try/catch` | `Result` + `match` / `?` |
| `throw` | `Err(...)` หรือ `panic!` ตาม semantics |
| `List<T>` | `Vec<T>` |
| `Map<K,V>` | `HashMap<K,V>` |
| `.where()` | `.filter()` |
| `.map()` | `.map()` |
| `.toList()` | `.collect()` |
| `Future<T>` | `Future<Output=T>` |
| Isolate | ไม่มี equivalent ตรง ๆ |
| GC-managed reference | ownership/borrowed/shared smart pointer |

---

# Part VIII — สิ่งที่ Rust มีแต่ Dart ไม่มี

## Ownership และ Borrow Checker

Compiler ตรวจ lifetime และ aliasing ของ references โดยไม่มี GC

## `Result` และ `?`

Recoverable errors เป็น typed values และ propagate แบบ explicit

## Data-carrying enums

Enum variant เก็บ payload ได้และ match แบบ exhaustive

## Traits

ใช้สำหรับ behavior composition และ generic constraints

## Zero-cost abstractions

Generics และ iterators หลายรูปแบบ compile ลง native code ที่ optimize ได้มาก

## Native binary ecosystem

เหมาะกับ CLI, embedded, WASM และ reusable native core

---

# Part IX — สิ่งที่ Flutter Developer อาจคิดถึง

## Garbage Collector

ใน Dart คุณไม่ต้องคิดว่า assignment จะ move ownership หรือไม่

## Shared mutable object ที่ง่ายกว่า

Dart object graph สร้างง่ายกว่า แต่ Rust บังคับให้ explicit เมื่อมี shared mutable state

## Hot Reload

Rust rebuild loop ไม่เหมือน Flutter hot reload

แยก Rust core ให้เล็กและ test ด้วย `cargo test/check` ช่วยให้ iteration เร็วขึ้น

## ความง่ายของ String/List API

Rust ได้ safety/control แลกกับ type และ ownership distinctions ที่มากขึ้น

---

# Part X — แผนเรียน 10 วันสำหรับ Flutter Developer ที่มีประสบการณ์

## Day 1 — Syntax และ Tooling

เรียน `let`, `mut`, functions, expressions, `cargo check`, `fmt`, `clippy`

อย่าใช้เวลามากกับ syntax

## Day 2 — Structs, Enums และ `match`

เชื่อม mental model จาก Dart class/sealed class

## Day 3 — Ownership

วันสำคัญที่สุด

เข้าใจ move, copy, clone และ scope

## Day 4 — Borrowing

ฝึก `&T`, `&mut T` และอ่าน borrow checker error

## Day 5 — Lifetimes และ API Design

เข้าใจว่าทำไม reference ต้องมี relationship กับ owner

ไม่ต้องพยายาม memorize ทุก lifetime pattern

## Day 6 — `Option`, `Result`, Traits, Generics

เรียน typed absence/error และ abstraction แบบ Rust

## Day 7 — Iterators และ Collections

เปรียบ `.iter()`, `.iter_mut()`, `.into_iter()`

## Day 8 — Async และ Concurrency

แยก async, runtime, thread และ cancellation

## Day 9 — Raw FFI Mental Model

เรียน C ABI, pointer, string/buffer ownership

แม้จะใช้ FRB ก็ควรรู้ boundary mental model

## Day 10 — Flutter Integration

สร้าง feature เล็กหนึ่งตัว เช่น:

- image resize
- checksum
- compression
- parser

แล้วต่อผ่าน `flutter_rust_bridge`

เพิ่ม adapter, fake engine และ tests

---

# ลำดับการเรียนที่แนะนำ

สำหรับ Flutter Developer:

```text
Syntax
↓
Struct / Enum / Match
↓
Ownership
↓
Borrowing
↓
Lifetimes
↓
Option / Result
↓
Traits / Generics
↓
Iterators
↓
Async / Concurrency
↓
FFI Mental Model
↓
flutter_rust_bridge
↓
Production Architecture
```

อย่าข้าม Ownership/Borrowing แล้วกระโดดไป FFI โดยตรง เพราะปัญหา integration หลายอย่างคือ ownership problem ในรูปแบบใหม่

---

# Repository Map

```text
README.md                 English default guide
README_TH.md              คู่มือภาษาไทย

examples/                 runnable Rust examples

docs/                     English production docs
  architecture.md
  ffi.md
  flutter_rust_bridge.md
  async_cancellation.md
  buffers_zero_copy.md
  error_mapping.md
  state_management.md
  testing_ci.md
  packaging_release.md
  performance.md
  security.md
  observability.md
  migration_checklist.md
  production_architecture.md
  PRODUCTION_CHECKLIST.md

  th/                     เอกสารภาษาไทยคู่ขนาน

production_sample/        architecture/integration sample
flutter_integration/      integration exercises
.github/workflows/        CI examples
```

---

# คำแนะนำสุดท้าย

สำหรับ Flutter Developer ที่เริ่ม Rust อย่าพยายามเขียน Rust ให้เหมือน Dart

ช่วงแรก borrow checker อาจดูเหมือนขัดขวาง แต่จริง ๆ มันกำลังบังคับให้ตอบคำถามที่ GC language มักซ่อนไว้:

- ใครเป็นเจ้าของข้อมูล
- ใครแก้ไขได้
- มีใครอ้างถึงพร้อมกันหรือไม่
- reference ยัง valid หรือไม่

เมื่อใช้ Rust กับ Flutter ประเด็นสำคัญกว่าความเร็วของภาษา คือ **boundary design**

Rust engine ที่เร็วมากแต่ API call ละเอียดเกินไป, copy buffer หลายรอบ, block UI isolate หรือมี lifecycle ไม่ชัด สามารถทำให้ระบบโดยรวมแย่กว่า pure Dart ได้

เริ่มจาก abstraction ง่าย ๆ วัด performance จริง แล้วค่อย optimize จุดที่ profiler ชี้

---

## เอกสารภาษาไทยเชิงลึก

- [Architecture](docs/th/architecture.md)
- [Production Architecture](docs/th/production_architecture.md)
- [Raw dart:ffi](docs/th/ffi.md)
- [flutter_rust_bridge](docs/th/flutter_rust_bridge.md)
- [Async / Cancellation](docs/th/async_cancellation.md)
- [Buffers / Zero-Copy](docs/th/buffers_zero_copy.md)
- [Error Mapping](docs/th/error_mapping.md)
- [Riverpod / Cubit](docs/th/state_management.md)
- [Testing / CI](docs/th/testing_ci.md)
- [Packaging / Release](docs/th/packaging_release.md)
- [Performance](docs/th/performance.md)
- [Security](docs/th/security.md)
- [Observability](docs/th/observability.md)
- [Migration Checklist](docs/th/migration_checklist.md)
- [Production Checklist](docs/th/PRODUCTION_CHECKLIST.md)

---

## References

สำหรับ API และ tooling ที่เปลี่ยนตามเวลา ให้ตรวจ official/upstream documentation ก่อนนำไปใช้ production โดยเฉพาะ Flutter/Dart SDK, Rust toolchain และ `flutter_rust_bridge` integration backend
