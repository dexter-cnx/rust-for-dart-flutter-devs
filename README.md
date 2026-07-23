# Rust for Dart & Flutter Developers — The Complete Guide

**Language:** English (default) · [ภาษาไทย](README_TH.md)

> A practical tour of Rust for developers who already know Dart and Flutter. Each section starts from a Dart/Flutter mental model and maps it to the Rust equivalent — including the places where the comparison breaks down.

This guide is designed for experienced Flutter developers. It assumes you already understand Dart syntax, null safety, `Future`, `Stream`, classes, mixins, isolates, packages, and common Flutter architecture patterns.

The goal is not merely to teach Rust syntax. The real goal is to help you build the **Rust mental model** you need before putting Rust behind a Flutter application.

## Runnable companion

Every core language concept has a runnable Rust example in [`examples/`](examples):

```bash
cargo run --example 01_variables
cargo run --example 07_ownership
cargo run --example 08_borrowing
cargo run --example 13_async
cargo run --example 14_concurrency
```

Run all checks with:

```bash
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
```

---

## The 30-second summary

Rust will initially feel familiar to a Dart developer: type inference, immutable-by-default bindings, enums, generics, closures, async/await, iterators, packages, and expressive type systems.

Then ownership appears.

Dart uses garbage collection. Rust does not. Rust instead tracks who owns every value, when that value is moved, when it is borrowed, and how long references remain valid. These rules are checked at compile time.

For Flutter developers, the biggest shift is therefore not syntax. It is moving from:

> “I can pass this object reference around and the GC will eventually clean it up.”

To:

> “Who owns this value right now, and am I moving it, borrowing it, cloning it, or sharing it?”

| Dart / Flutter habit | Rust reality |
|---|---|
| `final` / `var` | `let` / `let mut` |
| GC-managed objects | Ownership and deterministic `Drop` |
| Nullable `T?` | `Option<T>` |
| `throw` / `try` / `catch` | `Result<T, E>` for recoverable failures |
| `class` | Usually `struct` + `impl` |
| `abstract class` / interface-like contracts | `trait` |
| Enhanced enums / sealed classes | Data-carrying `enum` + `match` |
| `Future<T>` | `Future<Output = T>` + `async`/`.await` |
| `Stream<T>` | Often async streams/channels/bridge-specific streams |
| Isolates | OS threads, async tasks, channels, `Send`/`Sync` |
| `pubspec.yaml` | `Cargo.toml` |
| `dart pub` | `cargo` |
| `dart format` | `cargo fmt` |
| `dart analyze` | compiler + `cargo clippy` |
| Native plugin / FFI | C ABI via `dart:ffi`, or generated bridge tooling |

---

# Part I — Rust through a Dart lens

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

Rust bindings are immutable by default. Mutation is explicit with `mut`.

Rust also uses **shadowing** idiomatically:

```rust
let text = "42";
let text = text.parse::<i32>().unwrap();
```

This creates a new binding rather than mutating the old one.

Run:

```bash
cargo run --example 01_variables
```

---

## 2. Built-in types

| Dart | Rust |
|---|---|
| `int` | `i8/i16/i32/i64/i128/isize` or unsigned variants |
| `double` | `f32` / `f64` |
| `bool` | `bool` |
| `String` | `String` and `&str` |
| `List<T>` | `Vec<T>`, arrays, slices |
| `Map<K,V>` | `HashMap<K,V>` |
| `(a, b)` records | tuples `(A, B)` |
| `Never` | `!` |
| `void` | usually `()` |

Unlike Dart's arbitrary-precision VM integer behavior in some runtimes, Rust integer types have explicit widths and overflow behavior matters.

---

## 3. Strings: `String` is not just one thing

A Dart developer typically uses one main string abstraction:

```dart
String name = 'Ada';
```

In Rust you constantly meet two forms:

```rust
let borrowed: &str = "Ada";
let owned: String = String::from("Ada");
```

- `String` owns growable UTF-8 data.
- `&str` is a borrowed view into UTF-8 string data.

A useful API rule is:

> Accept `&str` when you only need to read text. Return `String` when the caller must own newly-created text.

You cannot index a Rust string with `s[0]` because UTF-8 makes “the first character” ambiguous at the byte/code-point/grapheme level.

Run:

```bash
cargo run --example 02_strings
```

---

## 4. Functions and expressions

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

The final expression without a semicolon becomes the return value.

```rust
let value = {
    let x = 10;
    x * 2
};
```

The block itself evaluates to `20`.

Closures look familiar:

```rust
let double = |x| x * 2;
```

Run:

```bash
cargo run --example 03_functions_closures
```

---

## 5. Null safety: `T?` → `Option<T>`

Dart encodes absence with nullable types:

```dart
String? name;
```

Rust encodes absence with an enum:

```rust
let name: Option<String> = None;
let name = Some(String::from("Ada"));
```

Conceptual mapping:

| Dart | Rust |
|---|---|
| `T?` | `Option<T>` |
| `null` | `None` |
| non-null value | `Some(value)` |
| `x?.foo()` | `x.as_ref().map(...)` |
| `x ?? fallback` | `x.unwrap_or(fallback)` |
| `x!` | `x.unwrap()` / `expect(...)` |

Prefer not to treat `unwrap()` as the Rust equivalent of normal Dart `!` usage. In production Rust, explicit propagation or pattern matching is usually better.

Run:

```bash
cargo run --example 04_option
```

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

Rust has no class inheritance hierarchy. Data is typically modeled with structs and enums. Behavior is implemented with `impl` blocks and shared contracts are modeled with traits.

Three receiver forms matter:

- `&self`: read-only borrow
- `&mut self`: exclusive mutable borrow
- `self`: consume the value

This distinction becomes fundamental once ownership clicks.

Run:

```bash
cargo run --example 05_structs
```

---

## 7. Enhanced enums / sealed modeling → Rust enums

Rust enums are one of the language's strongest modeling tools.

```rust
enum LoadState {
    Idle,
    Loading,
    Success { items: Vec<String> },
    Failure(String),
}
```

For a Flutter developer this should feel close to modeling UI state with sealed classes:

```dart
sealed class LoadState {}
final class Idle extends LoadState {}
final class Loading extends LoadState {}
final class Success extends LoadState {
  Success(this.items);
  final List<String> items;
}
```

Rust keeps the variants in a single declaration and combines naturally with exhaustive `match`.

Run:

```bash
cargo run --example 06_enums_match
```

---

# Part II — The actual new mental model

## 8. Ownership: the concept Dart developers do not have

In Dart, this is ordinary:

```dart
final a = User();
final b = a;
```

Both variables refer to the same GC-managed object.

In Rust, many heap-backed values move by default:

```rust
let a = String::from("hello");
let b = a;
// println!("{a}"); // compile error: a was moved
```

There is exactly one owner of that `String` after the assignment: `b`.

Why? Because Rust wants to know exactly who is responsible for cleaning up the value without a garbage collector.

When the owner leaves scope, Rust calls `Drop` and releases the resource.

### Move vs Copy vs Clone

- **Move** transfers ownership.
- **Copy** duplicates small stack-like values automatically when the type implements `Copy`.
- **Clone** explicitly duplicates a value.

```rust
let x = 5;
let y = x; // Copy

let a = String::from("hello");
let b = a.clone(); // explicit duplicate
```

Do not be afraid of `.clone()` while learning. First make ownership correct; optimize unnecessary cloning later.

Run:

```bash
cargo run --example 07_ownership
```

---

## 9. Borrowing: use a value without taking it

Instead of transferring ownership, you can borrow:

```rust
fn length(text: &str) -> usize {
    text.len()
}
```

Mutable borrow:

```rust
fn append_bang(text: &mut String) {
    text.push('!');
}
```

The central rule is often summarized as:

> You may have many shared references, or one mutable reference, but not both at the same time.

This is the “aliasing XOR mutability” rule.

For Flutter developers, think about all the runtime bugs that can occur when several pieces of code share and mutate the same object. Rust pushes a large class of those problems into compile-time errors.

Run:

```bash
cargo run --example 08_borrowing
```

---

## 10. Lifetimes

A lifetime describes the relationship between references. It does not manually control how many milliseconds an object exists.

Most lifetimes are inferred.

You write explicit lifetime parameters when Rust needs help understanding how an output reference relates to input references:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

For Flutter integration, lifetimes matter because crossing an FFI boundary usually requires **owned or bridge-managed values**. You cannot casually return a temporary borrowed Rust reference into Dart and expect the GC and borrow checker to coordinate magically.

Run:

```bash
cargo run --example 09_lifetimes
```

---

## 11. Error handling: exceptions → `Result<T, E>`

Dart:

```dart
try {
  final text = await file.readAsString();
} catch (e) {
  // ...
}
```

Rust makes recoverable failure part of the return type:

```rust
fn parse_age(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse::<u32>()
}
```

The `?` operator propagates an error immediately:

```rust
fn load_age(text: &str) -> Result<u32, std::num::ParseIntError> {
    let age = text.parse::<u32>()?;
    Ok(age)
}
```

Use `panic!` for broken invariants and programmer bugs, not ordinary recoverable application failures.

For Flutter bridges, a typed Rust `Result` can often be mapped into a Dart-side failure/exception representation by the bridge layer.

Run:

```bash
cargo run --example 10_result
```

---

## 12. Interfaces and mixins → traits

A Rust trait describes shared behavior:

```rust
trait Describe {
    fn describe(&self) -> String;
}
```

Then implement it for a type:

```rust
struct Movie {
    title: String,
}

impl Describe for Movie {
    fn describe(&self) -> String {
        self.title.clone()
    }
}
```

Traits fill several roles Dart developers associate with:

- interface contracts
- abstract behavior
- generic constraints
- extension-like behavior
- operator traits

Rust prefers composition over inheritance.

Run:

```bash
cargo run --example 11_traits
```

---

## 13. Generics

Dart:

```dart
T first<T>(List<T> items) => items.first;
```

Rust:

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

Trait bounds constrain generic types:

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{value}");
}
```

Rust generics are commonly monomorphized at compile time, which is one of the mechanisms behind Rust's zero-cost abstraction story.

---

## 14. Collections and iterators

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

The ownership-sensitive iterator methods matter:

- `.iter()` borrows items
- `.iter_mut()` mutably borrows items
- `.into_iter()` consumes the collection

That difference is more important in Rust than the superficial syntax difference.

Run:

```bash
cargo run --example 12_iterators
```

---

# Part III — Async, isolates, and concurrency

## 15. `Future` → Rust `Future`

Dart:

```dart
Future<String> fetch() async {
  return 'hello';
}
```

Rust:

```rust
async fn fetch() -> String {
    "hello".to_string()
}
```

And:

```rust
let value = fetch().await;
```

The syntax looks similar, but there is an architectural difference: Rust's standard library defines the future abstraction but does not provide a full async runtime. Applications commonly use a runtime such as Tokio.

Run:

```bash
cargo run --example 13_async
```

---

## 16. Dart Isolates vs Rust threads/tasks

Do not map Dart isolates directly to Rust threads.

A Dart isolate:

- has its own memory heap
- communicates primarily through messages
- avoids ordinary shared mutable memory

Rust can use:

- OS threads
- async tasks
- channels
- shared state with `Arc<T>`
- synchronization with `Mutex<T>` / `RwLock<T>`

Rust then uses the marker traits `Send` and `Sync` to express whether types may safely move or be shared across threads.

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
```

For a Flutter application, the practical rule is:

> Never block Flutter's UI isolate with heavy native work. Decide deliberately whether the Rust operation is synchronous, executed on a worker pool, or asynchronous.

Run:

```bash
cargo run --example 14_concurrency
```

---

# Part IV — Project structure and tooling

## 17. Packages: `pubspec.yaml` → `Cargo.toml`

| Dart / Flutter | Rust |
|---|---|
| package | crate |
| `pubspec.yaml` | `Cargo.toml` |
| `dart pub get` | `cargo fetch` / `cargo build` |
| `dart pub add foo` | `cargo add foo` |
| package import | module/crate paths |
| `lib/` | typically `src/lib.rs` |
| executable entry | `src/main.rs` |

Modules are private by default unless exposed with `pub`.

---

## 18. Tooling crash course

| Task | Dart / Flutter | Rust |
|---|---|---|
| Create project | `dart create`, `flutter create` | `cargo new` |
| Build | `flutter build` | `cargo build` |
| Optimized build | profile-specific Flutter build | `cargo build --release` |
| Run | `dart run`, `flutter run` | `cargo run` |
| Test | `dart test`, `flutter test` | `cargo test` |
| Format | `dart format` | `cargo fmt` |
| Analyze/lint | `dart analyze` | compiler + `cargo clippy` |
| Dependency metadata | `pubspec.yaml` | `Cargo.toml` |
| Docs | `dart doc` | `cargo doc --open` |

Run Clippy constantly:

```bash
cargo clippy --all-targets -- -D warnings
```

It is one of the best ways to learn idiomatic Rust.

---

# Part V — Flutter ↔ Rust integration

## 19. When should a Flutter developer use Rust?

Good candidates include:

- CPU-heavy image processing
- compression and codecs
- cryptographic primitives through audited libraries
- parsing large binary formats
- signal/audio processing
- reusable domain engines shared across platforms
- high-performance algorithms
- existing Rust libraries you want to reuse
- deterministic native components with tight memory control

Poor candidates include:

- ordinary screen composition
- standard REST API calls already handled comfortably in Dart
- business logic that changes frequently and has no performance/native reuse benefit
- code that forces large amounts of data to cross the bridge repeatedly

The bridge has a cost. A fast Rust function can still produce a slow application if you continuously serialize and copy large objects between Dart and Rust.

---

## 20. Integration option A: `dart:ffi`

Dart's FFI layer calls native functions through a C-compatible ABI.

A common Rust pattern is to expose C ABI functions:

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Conceptually, Dart then loads the native library and looks up the symbol.

Raw FFI gives maximum control but you must think carefully about:

- C-compatible types
- symbol export
- allocation and deallocation ownership
- strings and buffers
- pointers
- dynamic/static library packaging
- Android/iOS/macOS/Windows/Linux build differences
- asynchronous work
- callback threading

Read [`docs/ffi.md`](docs/ffi.md) for the detailed mental model.

For larger C-style APIs, Dart tooling can generate bindings from C headers using `ffigen`.

---

## 21. Integration option B: `flutter_rust_bridge`

For many Flutter applications, a generated bridge can remove much of the manual glue.

The current `flutter_rust_bridge` documentation describes a workflow where you write Rust APIs, generate glue code, compile/bundle the Rust crate with the Flutter application, then call generated Dart APIs.

Typical setup flow:

```bash
cargo install flutter_rust_bridge_codegen
flutter_rust_bridge_codegen create my_app
cd my_app
flutter run
```

For an existing Flutter project:

```bash
flutter_rust_bridge_codegen integrate
```

When Rust APIs change:

```bash
flutter_rust_bridge_codegen generate
```

Or watch continuously:

```bash
flutter_rust_bridge_codegen generate --watch
```

A Rust function such as:

```rust
pub fn hello(name: String) -> String {
    format!("Hello, {name} from Rust")
}
```

can be exposed through generated Dart bindings, making the Dart call site feel much closer to a normal Dart API than hand-written raw FFI.

See [`docs/flutter_rust_bridge.md`](docs/flutter_rust_bridge.md).

---

## 22. Boundary design: the most important integration skill

The key architecture question is not “How do I call Rust?”

It is:

> What should cross the Dart ↔ Rust boundary?

Prefer coarse-grained APIs:

```text
Dart UI
  ↓
processImage(bytes, options)
  ↓
Rust performs entire pipeline
  ↓
Result bytes / metadata
```

Avoid extremely chatty APIs:

```text
Dart → Rust: setPixel
Dart → Rust: setPixel
Dart → Rust: setPixel
... millions of calls
```

A good bridge API:

- transfers data in meaningful batches
- minimizes copies of large buffers
- keeps long-running domain state on one side when possible
- exposes explicit error types
- defines cancellation behavior
- defines threading behavior
- avoids leaking Rust lifetime complexity into Dart APIs

---

## 23. Memory ownership across the boundary

Inside pure Rust, the compiler tracks ownership.

Across a raw C FFI boundary, the compiler cannot automatically protect both languages from incorrect ownership conventions.

You must define contracts such as:

- Who allocates this buffer?
- Who frees it?
- Is Dart borrowing it or taking ownership?
- Can Rust retain this pointer after the call returns?
- Is this callback still valid later?

Generated bridge frameworks exist partly to automate and constrain this complexity.

The safest high-level rule is:

> Prefer owned values at API boundaries unless the bridge explicitly provides a safe managed or zero-copy abstraction.

---

## 24. CPU-heavy work and the Flutter UI isolate

A common mistake is replacing slow Dart code with fast Rust code but calling it synchronously on the UI isolate.

If a Rust call takes 200 ms and blocks the caller, Flutter can still drop frames.

Think in terms of two independent questions:

1. Is the implementation fast?
2. Does the call block the Flutter UI isolate?

For CPU-heavy Rust work, use bridge/runtime mechanisms that execute away from the UI thread and return results asynchronously.

For tiny calculations needed synchronously during non-critical code paths, synchronous calls may be appropriate. Do not put expensive native work inside `build()`.

---

## 25. Mapping common Flutter architecture to Rust

A pragmatic split often looks like:

```text
Flutter / Dart
├── Widgets and rendering
├── Navigation
├── Presentation state (Cubit/Riverpod/etc.)
├── Platform UX
└── Rust adapter/repository boundary
        ↓
Rust
├── Computation-heavy domain engine
├── Parsers/codecs
├── Shared algorithms
├── Native libraries
└── Performance-sensitive state machine
```

Do not move code to Rust merely because Rust exists.

A useful test is:

> If this module were written in excellent Dart, would Rust still provide a clear advantage in performance, safety, portability, or library reuse?

If the answer is no, keep it in Dart.

---

# Part VI — Production Flutter ↔ Rust

Learning syntax is only the beginning. A production integration needs an explicit contract for threading, cancellation, memory ownership, error mapping, observability, testing, packaging, and upgrades.

## 26. Production boundary checklist

Before exposing a Rust API to Flutter, write down:

- **Ownership:** which side owns input and output buffers, and when memory can be released.
- **Threading:** whether the call is synchronous, asynchronous, CPU-bound, or I/O-bound.
- **Cancellation:** whether cancellation is best-effort, cooperative, or unsupported after native work starts.
- **Errors:** the stable error codes/types Dart presentation code can depend on.
- **Progress:** whether long work reports progress and whether updates are throttled.
- **Backpressure:** what happens when Dart requests work faster than Rust can complete it.
- **Versioning:** how generated bindings and Rust APIs are upgraded together.
- **Observability:** where timing, failures, and correlation IDs are recorded.

The bridge is an architectural boundary, not just a function-call mechanism.

## 27. Recommended production layering

```text
Flutter UI / Widgets
        ↓
Presentation state (Riverpod / Cubit / Bloc)
        ↓
Dart domain-facing interface
        ↓
RustEngineAdapter  ← maps DTOs, errors, cancellation, progress
        ↓
Generated bridge API
        ↓
Rust public API layer  ← stable boundary types only
        ↓
Rust domain / services / algorithms
```

Keep generated bindings out of Widgets and preferably out of domain code. This lets you mock the engine in Dart tests and prevents bridge-specific types from spreading across the app.

See [Production Architecture](docs/production_architecture.md).

## 28. Cancellation and latest-request-wins

Flutter screens often launch work that becomes stale: search queries, image previews, filters, or route-scoped processing. Treat cancellation as a protocol.

A practical design is:

1. Dart creates a request ID or cancellation token.
2. Rust periodically checks cooperative cancellation at safe checkpoints.
3. Dart discards any late result whose request ID is no longer current.
4. Cancellation is represented separately from failure.

This combination matters because cancelling a Dart `Future` does not automatically stop arbitrary native computation that has already started.

See [Async, Cancellation, and Backpressure](docs/async_cancellation.md).

## 29. Large buffers and copy budgets

For images, audio, ML tensors, archives, and binary parsers, benchmark the entire pipeline:

```text
read/decode → Dart allocation → bridge transfer → Rust allocation
→ compute → bridge transfer → Dart allocation → render/write
```

A 3× faster algorithm can lose its advantage if the architecture performs several full-size copies per frame. Prefer coarse-grained operations and reuse native-side state when it materially reduces transfers.

Do not call an API “zero-copy” without validating the exact bridge/runtime path on every supported platform. Treat zero-copy as an optimization with lifetime constraints, not a default assumption.

See [Buffers, Typed Data, and Zero-Copy Design](docs/buffers_zero_copy.md).

## 30. Stable error mapping

Do not make UI code parse Rust error strings. Model stable error categories:

```text
Rust internal error
  → public Rust EngineError
  → bridge representation
  → Dart EngineFailure
  → localized presentation message
```

Keep diagnostic detail for logs while exposing a stable machine-readable category to Dart.

See [Error Mapping](docs/error_mapping.md).

## 31. Riverpod / Cubit integration

The state-management library should depend on a Dart interface, not directly on generated bridge functions. The adapter owns:

- generated API calls
- request IDs / cancellation
- error conversion
- progress stream conversion
- DTO mapping
- performance tracing

See [State Management Integration](docs/state_management.md).

## 32. Testing pyramid

A production setup should separate:

- Rust unit and property tests for algorithms and invariants
- Rust integration tests for public API behavior
- Dart unit tests for the adapter and error mapping
- Riverpod/Cubit tests with a fake engine
- Flutter widget tests without native binaries where possible
- Flutter integration tests that exercise generated bridge code
- physical-device smoke tests for Android/iOS packaging and performance

See [Testing and CI](docs/testing_ci.md).

## 33. CI quality gates

A sensible baseline:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
flutter analyze
flutter test
```

Then add code-generation drift checks and at least one platform integration build. Keep platform matrices proportional to what you actually ship.

## 34. Packaging and release

Applications and reusable packages have different constraints. Applications can build Rust during the app build; reusable packages may need a deliberate strategy for source builds versus prebuilt binaries. Pin or document compatible versions of the Rust toolchain, bridge generator, Flutter/Dart SDK, Android NDK, and Apple deployment targets.

See [Packaging and Release](docs/packaging_release.md).

## 35. Security and native-code review

Rust reduces broad classes of memory-safety bugs in safe Rust, but integration code can still be unsafe or incorrect. Audit `unsafe`, raw pointers, C ABI ownership, integer/length conversions, untrusted input parsers, dependency updates, and native library loading.

See [Security Checklist](docs/security.md).

## 36. Observability and performance tracing

Measure at the boundary. Record operation name, duration, payload size, outcome category, and an optional correlation ID. Avoid logging secrets or raw user data. For expensive pipelines, capture separate timings for transfer/serialization and computation where practical.

See [Observability](docs/observability.md).

## 37. Production sample architecture

The repository includes [`production_sample/`](production_sample/) with a framework-neutral Dart adapter, Riverpod/Cubit integration sketches, Rust API shapes, cancellation conventions, and a test matrix. Generated files are intentionally not committed because they depend on the selected bridge version and integration backend.

---

# Part VII — Dart → Rust idiom cheat sheet

| Dart | Rust |
|---|---|
| `final x = 5` | `let x = 5;` |
| `var x = 5` | `let mut x = 5;` |
| `String?` | `Option<String>` |
| `null` | `None` |
| `x ?? fallback` | `x.unwrap_or(fallback)` |
| `x!` | `x.unwrap()` / `expect(...)` |
| `if (x != null)` | `if let Some(x) = ...` |
| `switch` / pattern matching | `match` |
| sealed state hierarchy | data-carrying `enum` |
| `class` | `struct` + `impl` |
| interface contract | `trait` |
| extension methods | extension traits / inherent `impl` |
| `List<T>` | `Vec<T>` / `&[T]` |
| `Map<K,V>` | `HashMap<K,V>` |
| `Future<T>` | Rust future returned by `async fn` |
| `await future` | `future.await` |
| `throw` | usually `Err(...)` for recoverable failures |
| `try/catch` | `match Result` / `?` propagation |
| `Isolate` | no direct equivalent; threads/tasks/channels differ |
| `pubspec.yaml` | `Cargo.toml` |
| `dart analyze` | compiler + Clippy |
| `dart format` | rustfmt |

---

# Part VIII — Things Rust has that Dart does not

- Ownership and borrowing without a garbage collector
- Lifetimes checked by the compiler
- Deterministic resource cleanup with `Drop`
- `Send`/`Sync` compile-time concurrency guarantees
- Algebraic data types as a central language idiom
- Exhaustive pattern matching
- Trait-based abstraction without class inheritance
- Native binaries without a Dart VM runtime requirement
- Strong control over memory layout and allocation
- Mature systems-programming ecosystem

---

# Part IX — Things Flutter developers may miss

- Garbage collection that makes arbitrary object graphs easy
- Hot reload across nearly all application logic
- Extremely fast UI iteration
- Simple shared object references
- High-level platform abstractions
- One language from UI through most application logic
- `pub.dev` packages designed specifically around Flutter widgets
- Debugging a single runtime instead of a Dart/native boundary

Rust is not a replacement for Flutter. In most successful Flutter + Rust designs, Rust is a specialized engine underneath a Flutter application.

---

# Part X — A 10-day learning plan for experienced Flutter developers

## Day 1 — Syntax and tooling

Learn `let`, `mut`, scalar types, functions, expressions, Cargo, rustfmt, and Clippy.

Run examples 01–03.

## Day 2 — Structs, enums, and `match`

Map Dart data classes/sealed UI states into Rust structs and enums.

Run examples 05–06.

## Day 3 — Ownership

Spend the entire day on move, copy, clone, and `Drop`.

Run example 07 repeatedly and intentionally create compiler errors.

## Day 4 — Borrowing

Learn `&T`, `&mut T`, slices, and aliasing XOR mutability.

Run example 08.

## Day 5 — Lifetimes and API design

Understand what lifetimes prove. Do not memorize lifetime syntax blindly.

Run example 09.

## Day 6 — `Option`, `Result`, traits, generics

Rebuild familiar Dart domain models with explicit absence and typed errors.

Run examples 04, 10, and 11.

## Day 7 — Iterators and collections

Learn ownership-aware iteration and avoid cloning everything by default.

Run example 12.

## Day 8 — Async and concurrency

Compare Dart's event loop/isolate model with Rust runtimes, threads, channels, `Send`, and `Sync`.

Run examples 13–14.

## Day 9 — Raw FFI mental model

Read [`docs/ffi.md`](docs/ffi.md). Build a tiny C ABI function. Focus more on ownership contracts than syntax.

## Day 10 — Flutter integration

Read [`docs/flutter_rust_bridge.md`](docs/flutter_rust_bridge.md), create a sample project, and move exactly one compute-heavy pure function behind the bridge.

Then measure whether moving it actually helped.

---

# Recommended learning order

```text
Rust syntax
    ↓
Struct + Enum + Match
    ↓
Ownership
    ↓
Borrowing
    ↓
Lifetimes
    ↓
Option + Result
    ↓
Traits + Generics
    ↓
Iterators
    ↓
Async + Concurrency
    ↓
FFI mental model
    ↓
flutter_rust_bridge
    ↓
Performance measurement and production architecture
```

---

# Repository map

Documentation is English-first. Thai translations live under [`docs/th/`](docs/th/) and the Thai entry README is [`README_TH.md`](README_TH.md).


```text
.
├── Cargo.toml
├── README.md
├── examples/
│   ├── 01_variables.rs
│   ├── 02_strings.rs
│   ├── 03_functions_closures.rs
│   ├── 04_option.rs
│   ├── 05_structs.rs
│   ├── 06_enums_match.rs
│   ├── 07_ownership.rs
│   ├── 08_borrowing.rs
│   ├── 09_lifetimes.rs
│   ├── 10_result.rs
│   ├── 11_traits.rs
│   ├── 12_iterators.rs
│   ├── 13_async.rs
│   └── 14_concurrency.rs
├── docs/
│   ├── ffi.md
│   ├── flutter_rust_bridge.md
│   ├── architecture.md
│   └── performance.md
└── flutter_integration/
    └── README.md
```

---

# Final advice

For an experienced Flutter developer, the Rust syntax is not the difficult part.

The curriculum is:

> ownership → borrowing → lifetimes → boundary design

Once those ideas become natural, Flutter + Rust stops feeling like “calling native code” and starts feeling like designing two collaborating runtimes with an explicit contract between them.

Keep UI and product iteration in Flutter. Put Rust where its strengths are measurable.

## References

- Rust Book: https://doc.rust-lang.org/book/
- Dart C interop (`dart:ffi`): https://dart.dev/interop/c-interop
- flutter_rust_bridge: https://cjycode.com/flutter_rust_bridge/
- flutter_rust_bridge quickstart: https://cjycode.com/flutter_rust_bridge/quickstart

> Integration tooling evolves. Verify current commands and platform requirements against the official documentation before adopting them in production.
