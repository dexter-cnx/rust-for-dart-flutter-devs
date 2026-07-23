# Security Checklist for Flutter + Rust

[ภาษาไทย](th/security.md)

Rust's safe subset prevents many memory-safety bugs, but the native boundary still deserves explicit review.

## Review areas

- Every `unsafe` block has a documented invariant.
- Pointer + length pairs are validated before constructing slices.
- Integer conversions cannot silently truncate lengths or offsets.
- Untrusted binary/image/archive input has bounds and resource limits.
- Allocation sizes are capped to prevent resource exhaustion.
- FFI strings have explicit encoding and termination rules.
- Native callbacks cannot outlive Dart objects they reference.
- Secrets are not included in logs or panic messages.
- Dependencies and toolchains are updated deliberately.

## Unsafe policy

Keep `unsafe` small and concentrated in adapter/FFI modules. Wrap it in safe Rust APIs and test the invariants at the safe boundary.

## Cryptography

Prefer well-reviewed maintained libraries rather than implementing cryptographic primitives yourself. Keep keys and secrets out of debug logs and bridge exceptions.
