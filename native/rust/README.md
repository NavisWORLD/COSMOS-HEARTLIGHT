# HEARTLIGHT Rust Workspace

Rust implementation of the same transparent 12D support-state contract used by the C++ core.

## Crates

- `heartlight-core`: pure Rust library with no third-party dependencies.
- `heartlight-ffi`: `cdylib` + `staticlib` C ABI bridge for Swift, Kotlin/JNI, C#, Python, game engines, and native shells.

## Build and test

```bash
cargo test --workspace --manifest-path native/rust/Cargo.toml
cargo build --release --workspace --manifest-path native/rust/Cargo.toml
```

## Cross compilation

Rust can target many desktop/mobile architectures through `rustup target add ...`. Platform signing and app-store packaging still require the appropriate Apple/Google/Microsoft toolchains and developer credentials.

## Safety contract

The core suggests educational supports only. It does not perform diagnosis, emotion recognition, misconduct prediction, or biometric interpretation.
