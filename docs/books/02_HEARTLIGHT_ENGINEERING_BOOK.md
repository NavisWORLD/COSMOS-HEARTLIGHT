# HEARTLIGHT — Engineering Book
## Building a transparent, local-first educational support platform

### Chapter 1 — Product thesis

HEARTLIGHT is a support interface, not a classifier. Its technical challenge is to make state legible without turning a child's temporary context into a permanent identity.

### Chapter 2 — Platform layers

The project now contains four engineering layers:

1. PWA user interface and offline cache.
2. Shared state contract.
3. C++17 native engine and C ABI.
4. Rust native engine and C ABI.

This makes the repository useful as both a school tool and a systems-engineering portfolio.

### Chapter 3 — The 12D state contract

Twelve bounded values encode the immediate support context. Bounded numeric state makes the engine testable while preserving an important rule: the numbers are not clinical measurements.

### Chapter 4 — Deterministic recommendations

The reference engines are intentionally inspectable. A threshold or explicit learner-selected need can emit one or more support options. There is no opaque model deciding what a child "is."

### Chapter 5 — Interoperability

The C++ and Rust layers expose C-compatible functions. This allows Swift, Kotlin/JNI, C#, Python, game engines, desktop shells, and assistive hardware to use the same contract.

### Chapter 6 — Sensor architecture

Environmental sensor data should be reduced before it reaches the core. A room camera can become RGB/brightness values in the host application; frames do not need to enter the engine. Biometrics are treated as optional observations, not emotional truth.

### Chapter 7 — Privacy as architecture

Local-first is a technical requirement, not just a privacy-policy sentence. No account is required. The default system works without a server. Student check-ins remain ephemeral. Explicitly saved educator observations are bounded and erasable.

### Chapter 8 — Verification

The repo includes static privacy/function audits, C++ unit tests, and Rust unit tests. CI should compile both native implementations on every change.

### Chapter 9 — Commercialization without losing the mission

A commercial deployment can sell packaging, district deployment support, training, integrations, accessibility hardware adapters, hosted administration approved by a district, or enterprise support. The child-support engine should remain explainable, privacy-preserving, and auditable.

### Chapter 10 — Portfolio walkthrough

In a technical interview, demonstrate:

- the PWA on a phone;
- the same support scenario in C++;
- the Rust crate and FFI boundary;
- the state contract;
- automated CI;
- privacy/safety threat modeling;
- the teacher documentation as evidence that engineering followed a real user workflow.

That tells a stronger story than "I made an app": it demonstrates product design, systems programming, accessibility thinking, privacy engineering, testing, documentation, and cross-platform architecture.
