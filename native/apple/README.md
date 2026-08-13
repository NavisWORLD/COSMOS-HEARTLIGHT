# Apple Integration — iPhone, iPad, macOS

The fastest Apple deployment is the installable PWA. For a native Swift/SwiftUI shell, link either the C++ C ABI or the Rust static library.

## C++ path

- Build `native/cpp` for the required Apple architecture(s).
- Expose `heartlight_c.h` in an Objective-C bridging header or module map.
- Call `heartlight_evaluate_mask` from Swift through the C interface.

## Rust path

- Build `heartlight-ffi` as a static library for each required Apple target.
- Combine architectures as appropriate for your Xcode project.
- Expose the exported C symbols to Swift using a bridging header/module map.

## Privacy

Camera/light sampling belongs in the host application and should target the room/environment, not the learner. The native engine never needs image frames.

## App Store release

App Store distribution requires an Apple Developer account, signing identities, entitlements, privacy manifests/declarations, and Apple review. This repository provides source and integration architecture, not somebody else's signing credentials.
