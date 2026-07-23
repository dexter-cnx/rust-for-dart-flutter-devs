# Buffers, Typed Data, and Zero-Copy Design

[ภาษาไทย](th/buffers_zero_copy.md)

## Start with a copy budget

For each large-buffer operation, count logical copies and allocations. Measure before optimizing.

```text
Dart bytes
  → bridge representation
  → Rust input
  → Rust output
  → bridge representation
  → Dart bytes
```

The exact number of physical copies depends on the binding mechanism, codec, platform, and versions in use.

## Prefer coarse-grained pipelines

Good:

```text
processImage(encodedBytes, options) -> processedBytes
```

Bad:

```text
for every pixel: Dart -> native call -> Dart
```

## Ownership contract

For raw FFI, document allocation/deallocation pairs and lengths. Never free memory on a different allocator unless the ABI contract explicitly supports it. A pointer without a length and ownership rule is an incomplete API.

## Typed data

Dart FFI can interoperate with native memory, and modern Dart also provides mechanisms that can reduce copying for some typed-data paths. Treat this as version- and API-specific: benchmark the exact supported SDK and platform matrix.

## Zero-copy is not free

A zero-copy buffer often trades copying for stricter lifetime rules, pinning, ownership transfer, or synchronization. The optimization is only valuable when transfer cost is material in the measured profile.

## Recommended workflow

1. Implement the simplest owned-value API.
2. Benchmark end-to-end.
3. Identify transfer cost separately from compute cost.
4. Optimize only the hot path.
5. Add tests for ownership/lifecycle behavior.
