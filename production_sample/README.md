# Production Sample Architecture

This folder is a **bridge-agnostic reference skeleton**, not a generated FRB project. Generate/integrate the bridge with the version selected by your application, then connect the generated API inside `RustNativeEngine`.

```text
production_sample/
├── dart/
│   ├── native_engine.dart
│   ├── rust_native_engine.dart
│   ├── request_coordinator.dart
│   ├── riverpod_example.dart
│   └── cubit_example.dart
└── rust/
    └── src/
        ├── api.rs
        ├── domain.rs
        └── lib.rs
```

The key rule is dependency direction: presentation depends on `NativeEngine`; only the adapter depends on generated bridge code.
