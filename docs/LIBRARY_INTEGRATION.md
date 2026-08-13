# HEARTLIGHT Library Integration Guide

This guide is for developers embedding HEARTLIGHT into another application. The reference platform offers two native implementations of the same shared support-state contract.

## C++ API

```cpp
#include "heartlight/heartlight.hpp"

heartlight::Input input;
input.state.sensory_load = 8.0;
input.needs = {heartlight::Need::Quiet};

auto result = heartlight::evaluate(input);
for (const auto& suggestion : result.suggestions) {
    // Present suggestion.title/rationale to a human reviewer.
}
```

Build and install:

```bash
cmake -S native/cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp
cmake --install build/cpp --prefix build/heartlight-install
```

The install tree contains headers and static/shared libraries.

## Stable C ABI

Use `native/cpp/include/heartlight/heartlight_c.h` when the host language should not depend on C++ ABI details. The function `heartlight_evaluate_mask(...)` accepts the 12D state, a bitmask of learner-selected needs, and optional reduced environmental values.

This is the preferred bridge for Swift/Objective-C, Kotlin/JNI, C#, Python `ctypes`, game engines, and many desktop shells.

## Rust API

```rust
use heartlight_core::{evaluate, Input, Need};

let mut input = Input::default();
input.state.sensory_load = 8.0;
input.needs = vec![Need::Quiet];
let result = evaluate(&input);
```

Build/test:

```bash
cargo test --workspace --manifest-path native/rust/Cargo.toml
cargo build --release --workspace --manifest-path native/rust/Cargo.toml
```

## Rust FFI

`heartlight-ffi` builds as both `cdylib` and `staticlib`. Its exported function `heartlight_rust_evaluate_mask(...)` provides a small C-compatible boundary for native host applications.

## Shared contract

Both engines implement `contracts/heartlight_state_v1.json`. If you write another implementation in Python, C#, Kotlin, Swift, Java, Go, or another language, preserve the dimension order and the safety contract.

## Important integration rule

A support suggestion is not a command. Host applications should display or route suggestions to a learner/human team for validation. Do not convert the native library into automated punishment, restraint/seclusion, diagnosis, eligibility, emotional inference, or misconduct prediction.

## Sensor inputs

Reduce camera/environment data in the host app before calling the core. The native engine needs brightness/RGB values, not image frames. The optional bio observation channel is informational only in the reference design and must not be interpreted as emotion, diagnosis, danger, or compliance.
