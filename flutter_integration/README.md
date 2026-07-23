# Flutter Integration Exercises

This directory intentionally contains integration exercises rather than generated bridge output, because generated files and exact platform scaffolding change with tool versions.

## Exercise 1 — Pure function

Move this conceptual Dart function into Rust:

```dart
List<int> grayscale(List<int> rgbaBytes) {
  // CPU-heavy pixel transformation
}
```

Design one coarse-grained Rust API that accepts a full buffer and returns a full buffer.

Questions:

- Is the input copied?
- Is the output copied?
- Where does the work execute?
- Can the operation be cancelled?
- How are errors represented?

## Exercise 2 — Progress stream

Create a Rust job that processes 100 logical steps and reports progress to Flutter.

Model:

```text
startJob(request)
→ progress 0..100
→ final result or failure
```

Avoid one bridge call per inner-loop operation.

## Exercise 3 — Clean Architecture adapter

Wrap generated bridge calls behind a Dart interface:

```dart
abstract interface class RustEngine {
  Future<String> transform(String input);
}
```

Use a fake in presentation tests and the real bridge only in integration tests.

## Exercise 4 — Benchmark

Compare:

1. pure Dart implementation
2. Dart isolate implementation
3. Rust implementation through the bridge

Measure end-to-end latency rather than Rust execution time alone.
