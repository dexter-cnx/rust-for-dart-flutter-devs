# Raw `dart:ffi` — Mental Model for Flutter Developers

[ภาษาไทย](th/ffi.md)
`dart:ffi` lets Dart Native applications call native APIs that expose a C-compatible ABI.

## The boundary

Conceptually:

```text
Dart code
  ↓ FFI call
C ABI-compatible function
  ↓
Rust implementation
```

Rust can expose a symbol using an `extern "C"` function. The exact export attributes depend on the Rust edition/toolchain and your build configuration, so verify current Rust guidance when creating production bindings.

## Why C ABI?

Dart does not directly understand Rust's native ABI, Rust enums, ownership model, trait objects, or generic types. A stable C-compatible interface provides a common boundary.

## Start with primitives

The easiest FFI APIs use values such as fixed-width integers and floats.

As soon as strings, arrays, buffers, callbacks, or long-lived objects cross the boundary, you need an explicit ownership contract.

## The four ownership questions

For every pointer or buffer, answer:

1. Who allocates it?
2. Who owns it after the call?
3. Who frees it?
4. How long is it valid?

If any answer is vague, the API is not ready.

## Strings

Rust `String` is not a C string and Dart `String` is not a native C string. A raw FFI design normally uses an agreed encoding and buffer representation, often UTF-8 bytes plus a length, or C strings where appropriate.

Avoid returning pointers to temporary Rust values.

## Buffers

For large image/audio/binary workloads, copying can dominate the actual algorithm cost. Measure:

- Dart → native copy
- Rust processing
- native → Dart copy

A 5 ms algorithm surrounded by 30 ms of copying is not a 5 ms feature.

## Blocking

A native call is not automatically asynchronous just because the implementation is written in Rust.

If Dart calls a synchronous FFI function from the UI isolate and the function blocks, Flutter can still jank.

## Binding generation

For C header-based APIs, Dart provides `ffigen` to generate Dart FFI bindings. This reduces handwritten signature boilerplate, but it does not eliminate the need to design correct ownership and threading contracts.

## When raw FFI is appropriate

Use raw FFI when you need:

- very explicit ABI control
- integration with an existing C API
- minimal runtime abstraction
- a small, stable native surface

Consider a higher-level generated bridge when you would otherwise spend significant time hand-writing type conversion, error mapping, async plumbing, and memory management.

## Official reference

https://dart.dev/interop/c-interop
