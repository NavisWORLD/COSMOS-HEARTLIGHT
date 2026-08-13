# HEARTLIGHT Multi-Language Architecture

## One product, several implementations

HEARTLIGHT is not "an HTML project with ports." It is a shared support-state specification with multiple front ends and native engines.

```text
Student / Teacher / Aide / Family UI
                 |
                 v
      HEARTLIGHT State Contract
                 |
      +----------+----------+
      |          |          |
      v          v          v
    Web/JS     C++17       Rust
      |          |          |
      +----------+----------+
                 |
                 v
     Human-reviewed support choices
```

## Shared 12D state

All implementations use the same 0–10 dimensions:

1. sensory load
2. visual load
3. movement need
4. focus access
5. transition need
6. communication load
7. social-space need
8. body comfort
9. predictability need
10. recovery need
11. engagement access
12. regulation confidence

The dimensions describe a present support context. They are not personality traits, diagnoses, disability labels, or risk scores.

## Why C++

C++ gives HEARTLIGHT a conventional systems integration path for Android NDK, Windows, embedded/assistive hardware, desktop software, game/visualization engines, and research tooling.

## Why Rust

Rust gives HEARTLIGHT a memory-safe systems implementation suitable for native apps, services, WebAssembly experiments, device adapters, and modern engineering portfolios.

## C ABI as interoperability spine

Both native implementations expose a small C-compatible boundary so that mobile and desktop host languages do not need to understand internal C++ or Rust types.

## Biometric boundary

The contract permits an optional observation channel so approved sensors can be integrated later. The reference engine intentionally refuses to infer emotion, diagnosis, danger, or behavior risk from pulse or other biometrics. Any clinical research extension must be separately validated and governed.

## Versioning

- Contract: `contracts/heartlight_state_v1.json`
- C++ library: semantic version in `heartlight::version()`
- Rust crates: semantic versions in Cargo manifests
- PWA: release notes at repository root

Breaking contract changes should introduce a new contract version instead of silently changing the meaning of a dimension.
