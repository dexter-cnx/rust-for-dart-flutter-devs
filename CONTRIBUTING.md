# Contributing

English is the default documentation language. When changing a production guide, update the corresponding Thai file under `docs/th/` when practical.

## Quality checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

The root crate contains runnable Rust learning examples. `production_sample/` is a reference skeleton and intentionally does not include generated bridge files.

## Documentation policy

- Keep exact tool commands version-aware.
- Prefer official Dart/Flutter/Rust/flutter_rust_bridge documentation.
- Do not promise zero-copy without platform/version evidence.
- Separate measured facts from architecture recommendations.
