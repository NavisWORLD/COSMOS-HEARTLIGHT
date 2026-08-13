# HEARTLIGHT Cross-Language SDK

HEARTLIGHT Synaptic Kernel v1 uses one shared numerical contract across language ecosystems.

## First-class implementations

- C++17: `native/cpp/`
- C ABI: `native/cpp/include/heartlight/synapse_c.h`
- Rust: `native/rust/heartlight-synapse/`
- Python: `sdk/python/`

All implementations use the same 12-channel state layout, row-major 12x12 matrix, defaults, bounds, and versioned conformance vectors in `sdk/spec/`.

## Universal C ABI bridge

Any runtime that can load a C-compatible shared library can integrate HEARTLIGHT. This covers Java/Kotlin, C#, Swift/Objective-C, Go, Node runtimes, Ruby, PHP, Dart/Flutter, Julia, Lua, R, Zig, Nim, Crystal, Haskell, OCaml, Fortran, MATLAB/Octave, Unity and other FFI-capable systems.

This approach keeps one source-of-truth contract rather than letting mathematical behavior drift between handwritten ports.

## Entry points

Python:

```bash
python -m pip install -e sdk/python
```

C++:

```cpp
#include <heartlight/synapse.hpp>
```

Rust:

```rust
use heartlight_synapse::{step, Config};
```

See `sdk/spec/synaptic_kernel_v1.md` and `sdk/spec/synaptic_test_vectors_v1.json` for the exact contract.
