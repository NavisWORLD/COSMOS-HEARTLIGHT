# COSMOS HEARTLIGHT Release Notes

## v0.2.0 — Native Engines & Books

HEARTLIGHT is now a multi-language platform rather than a web-only reference implementation.

### Added

- portable C++17 core library
- C++ static/shared builds and stable C ABI
- C++ CLI demonstration and unit tests
- Rust `heartlight-core` crate
- Rust `heartlight-ffi` static/dynamic C ABI crate
- shared machine-readable 12D state contract
- Android, Apple, and Windows native integration guides
- multi-language architecture manual
- any-device deployment matrix
- educator book
- engineering book
- study workbook
- portfolio/product guide
- GitHub Actions native C++ + Rust build workflow

### Design boundary retained

The optional bio observation channel is deliberately non-diagnostic. The reference engine does not infer medical condition, emotion, danger, compliance, or misconduct risk from pulse or other biometrics.

### Verification

The C++ build and unit test were compiled locally with CMake/G++ before publication. Rust verification is run in the repository's GitHub Actions workflow.

## v0.1.0 — Public School-Support Reference Build

Initial PWA, standalone app, teacher manuals, student/family resources, privacy architecture, school deployment guidance, static audit, and contribution infrastructure.
