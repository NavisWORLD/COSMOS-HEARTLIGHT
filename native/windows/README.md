# Windows Integration

HEARTLIGHT supports three practical Windows routes:

1. Run the PWA from the repository's local server launcher.
2. Build `native/cpp` with Visual Studio/CMake and call the C++ or C ABI directly.
3. Build `native/rust/heartlight-ffi` and call it from C#, C++, WinUI, WPF, Tauri-like shells, or another approved desktop host.

Example CMake flow:

```powershell
cmake -S native/cpp -B build/cpp -G "Visual Studio 17 2022"
cmake --build build/cpp --config Release
ctest --test-dir build/cpp -C Release --output-on-failure
```

The core contains no required cloud service and no account system.
