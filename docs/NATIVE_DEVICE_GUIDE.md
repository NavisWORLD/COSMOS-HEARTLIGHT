# HEARTLIGHT Any-Device Guide

"Any device" is implemented as a layered deployment strategy rather than a claim that one binary runs everywhere.

## Works immediately

The offline-first PWA is the universal UI path for modern iPhone/iPad, Android, Windows, macOS, Linux, ChromeOS, and tablets with a modern browser.

## Native engine targets

| Platform | Immediate path | Native core path |
|---|---|---|
| iPhone / iPad | PWA | Swift shell + C++/Rust C ABI |
| Android | PWA | Kotlin/Java shell + JNI + C++/Rust |
| Windows | PWA/local server | C++ DLL or Rust DLL/static lib |
| macOS | PWA/local server | Swift/AppKit shell + native core |
| Linux | PWA/local server | C++ `.so` or Rust library |
| ChromeOS | PWA | Android/Linux integration where supported |
| Embedded / assistive hardware | device-specific UI | C++17 or Rust core where toolchain permits |

## Why the PWA remains first-class

A school needs something deployable without app-store credentials. The PWA is therefore not a placeholder; it is the main cross-device application. Native libraries add integration depth without making deployment harder for teachers.

## What is not bundled

Store-signed `.ipa`, `.aab`, `.msix`, or notarized macOS binaries are not committed because they require platform-specific signing identities and release accounts. Source, native cores, tests, and integration guides are included so those builds can be produced by an authorized maintainer.
