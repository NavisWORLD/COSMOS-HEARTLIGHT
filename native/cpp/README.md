# HEARTLIGHT C++ Core

Portable C++17 implementation of the transparent HEARTLIGHT support-state engine.

## Build

```bash
cmake -S native/cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp --config Release
ctest --test-dir build/cpp --output-on-failure
```

Outputs include a static library, a shared library, a C ABI, a CLI demo, and tests.

## Why a C ABI?

`heartlight_c.h` gives mobile and desktop wrappers a small stable interface. Swift/Objective-C, Kotlin/JNI, C#, Python `ctypes`, Unity/Unreal plugins, and other host environments can call it without depending on C++ name mangling.

## Safety contract

The native core returns support options. It does not diagnose, score compliance, predict misconduct, recommend restraint/seclusion, or interpret pulse as emotion or medical status.
