# Android Integration

Recommended path: keep the HEARTLIGHT PWA for immediate deployment, or embed the C++/Rust native library in an Android app when a native shell is required.

## C++ via Android NDK

1. Add `native/cpp` as a CMake subdirectory in the Android project.
2. Link the generated `heartlight` shared library.
3. Expose the small C ABI in `heartlight_c.h` through JNI.
4. Keep student-facing UI state in the Android process; do not add network transmission by default.

## Rust

Compile `heartlight-ffi` for Android targets, package the resulting `.so` files by ABI, and call the C exports through JNI.

Typical targets include `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`, and `i686-linux-android` when required by your deployment.

## Camera / sensors

Any Android camera or sensor access belongs in the host app. The HEARTLIGHT reference engine accepts environmental observations but does not need camera frames and should never receive student face imagery.

## Store release

A Play Store build requires Android signing, target-SDK review, privacy declarations, and school/district approval where applicable. Those deployment credentials are intentionally not stored in this repository.
