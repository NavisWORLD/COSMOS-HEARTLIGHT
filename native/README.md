# HEARTLIGHT Native Platform Layer

HEARTLIGHT is intentionally multi-language:

- `app/` — installable offline-first PWA and primary user interface.
- `native/cpp/` — C++17 static/shared library plus C ABI.
- `native/rust/` — Rust core plus C ABI crate.
- `native/android/` — Android integration path.
- `native/apple/` — iOS/iPadOS/macOS integration path.
- `native/windows/` — Windows integration path.

The web app remains the fastest cross-device deployment. Native cores exist for embedded use, offline desktop/mobile shells, research integrations, accessibility hardware, and employers who want to inspect a conventional systems-language implementation.

All language implementations share the contract in `contracts/heartlight_state_v1.json`.

## Start here as a developer

- C++ build/API: [`cpp/README.md`](cpp/README.md)
- Rust workspace: [`rust/README.md`](rust/README.md)
- Full embedding/API examples: [`../docs/LIBRARY_INTEGRATION.md`](../docs/LIBRARY_INTEGRATION.md)
- Multi-language architecture: [`../docs/MULTI_LANGUAGE_ARCHITECTURE.md`](../docs/MULTI_LANGUAGE_ARCHITECTURE.md)
- Device matrix: [`../docs/NATIVE_DEVICE_GUIDE.md`](../docs/NATIVE_DEVICE_GUIDE.md)

The CI workflow also produces downloadable Linux build artifacts for both the C++ install tree and Rust FFI libraries after successful native builds.
