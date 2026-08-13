# 💚 COSMOS HEARTLIGHT

> **Open-source, local-first, multi-language support technology for inclusive classrooms.**  
> Communication • sensory access • teacher observation • accessibility • student autonomy • C++ • Rust • PWA

```text
╔══════════════════════════════════════════════════════════════════════╗
║                    C O S M O S   H E A R T L I G H T             ║
║                                                                      ║
║  CHILD SIGNAL → HUMAN UNDERSTANDING → GENTLE SUPPORT → LEARNING     ║
║                                                                      ║
║      PWA + C++17 + Rust + C ABI + Books + Teacher Toolkit           ║
║                                                                      ║
║        no ads • no accounts • no child scoring • local-first         ║
╚══════════════════════════════════════════════════════════════════════╝
```

HEARTLIGHT is for the learner who may be quiet, overwhelmed, moving, masking, non-speaking, dysregulated, hard to read, or simply being misunderstood.

It turns the COSMOS/CST engineering lineage into a bounded, inspectable support loop:

```text
perceive → compress → expand → validate → express → bounded storage
```

In HEARTLIGHT: **notice the situation, summarize understandable signals, generate several support possibilities, check the learner's response, communicate clearly, and store as little as necessary.**

---

## 🚀 Choose your path

| I am… | Start here | What you get |
|---|---|---|
| 👩‍🏫 Teacher | [`docs/00_TEACHER_START_HERE.md`](docs/00_TEACHER_START_HERE.md) | classroom setup + practical workflow |
| 🧩 Behavioral aide / paraeducator | [`docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md`](docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md) | neutral observation + support loop |
| 🌱 Student / learner | [`docs/STUDENT_GUIDE.md`](docs/STUDENT_GUIDE.md) | plain-language app guide |
| 👨‍👩‍👧 Family member | [`docs/FAMILY_GUIDE.md`](docs/FAMILY_GUIDE.md) | what HEARTLIGHT does and does not do |
| 🧠 OT / SLP / therapist / clinician | [`docs/CLINICIAN_ADJUNCT_GUIDE.md`](docs/CLINICIAN_ADJUNCT_GUIDE.md) | adjunct use without automated diagnosis |
| 🏫 School / district | [`docs/SCHOOL_DEPLOYMENT.md`](docs/SCHOOL_DEPLOYMENT.md) | privacy + deployment checklist |
| 💻 Developer | [`native/README.md`](native/README.md) | PWA, C++, Rust, C ABI, platform integration |
| 🧪 Researcher | [`docs/RESEARCH_AND_VALIDATION.md`](docs/RESEARCH_AND_VALIDATION.md) | validation boundaries + study direction |
| 💼 Recruiter / hiring manager | [`docs/PORTFOLIO_AND_PRODUCT_GUIDE.md`](docs/PORTFOLIO_AND_PRODUCT_GUIDE.md) | engineering/product walkthrough |

---

# 📲 Use HEARTLIGHT now

### One-file app

Download [`HEARTLIGHT_STANDALONE.html`](HEARTLIGHT_STANDALONE.html) and open it in a modern browser.

### Full installable PWA

```bash
python run_local.py
```

Then open `http://127.0.0.1:8877`.

The `app/` build is installable from HTTPS on iPhone/iPad, Android, Windows, macOS, Linux/ChromeOS browsers, and tablets. The PWA remains the fastest universal deployment path.

---

# 🧬 This is now a multi-language platform

```text
                         HEARTLIGHT
                             │
               ┌─────────────┼─────────────┐
               │             │             │
               ▼             ▼             ▼
            Web/PWA        C++17          Rust
               │             │             │
               │       static/shared   core + FFI
               │             │             │
               └─────────────┼─────────────┘
                             ▼
                  shared 12D state contract
                             │
                             ▼
                   human-reviewed supports
```

## 🌐 Web/PWA

The student/teacher application lives in [`app/`](app/) with a self-contained version at [`HEARTLIGHT_STANDALONE.html`](HEARTLIGHT_STANDALONE.html).

## ⚙️ C++17

[`native/cpp/`](native/cpp/) contains:

- portable C++17 core library
- static and shared-library builds
- stable C ABI in `heartlight_c.h`
- CLI demonstration
- unit tests
- CMake build

```bash
cmake -S native/cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp --parallel 2
ctest --test-dir build/cpp --output-on-failure
```

The C ABI makes the engine usable from Swift, Kotlin/JNI, C#, Python `ctypes`, native desktop shells, game engines, research applications, and assistive hardware.

## 🦀 Rust

[`native/rust/`](native/rust/) contains a Cargo workspace with:

- `heartlight-core` — dependency-free support-state engine
- `heartlight-ffi` — `cdylib` + `staticlib` C ABI bridge
- examples and unit tests

```bash
cargo test --workspace --manifest-path native/rust/Cargo.toml
cargo build --release --workspace --manifest-path native/rust/Cargo.toml
```

## 📱 Native device integration

- Android: [`native/android/README.md`](native/android/README.md)
- iPhone / iPad / macOS: [`native/apple/README.md`](native/apple/README.md)
- Windows: [`native/windows/README.md`](native/windows/README.md)
- full matrix: [`docs/NATIVE_DEVICE_GUIDE.md`](docs/NATIVE_DEVICE_GUIDE.md)

A single signed binary cannot literally run on every operating system. HEARTLIGHT therefore uses a **layered any-device strategy**: PWA for immediate universal UI, plus portable native cores for platform-specific apps and integrations.

---

# 🧭 Shared 12D support-state contract

All implementations use the same bounded present-context dimensions:

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

Machine-readable specification: [`contracts/heartlight_state_v1.json`](contracts/heartlight_state_v1.json).

These are **support variables**, not diagnoses, personality scores, disability labels, or misconduct-risk scores.

---

# ✨ What the student-facing system can communicate

Learners can choose needs such as quiet, movement, less light, space, help, a break, familiar proprioceptive options, a different sound environment, predictability, another communication channel, **“I don't know,”** or **“stay with me.”**

Speech is not treated as the only valid communication method.

The optional ambient sensor can reduce a room/light sample to RGB and brightness without needing to send camera frames into the state engine. The learner's preference outranks the sensor.

---

# 👩‍🏫 Teacher mode

```text
1. ASK       Give the learner access to communication choices.
2. LISTEN    Treat their selection as information, not a verdict.
3. OFFER     Choose one or two low-risk supports.
4. WAIT      Allow processing time.
5. OBSERVE   Record what can actually be observed.
6. CHECK     Ask whether the support helped.
7. ADAPT     Change the environment before blaming the learner.
8. ERASE     Retain only records the school actually needs.
```

Teacher starting point: [`docs/00_TEACHER_START_HERE.md`](docs/00_TEACHER_START_HERE.md).

---

# 📚 HEARTLIGHT Books & Learning Library

### Books

- 📗 [`HEARTLIGHT — Educator Book`](docs/books/01_HEARTLIGHT_EDUCATOR_BOOK.md)
- 📘 [`HEARTLIGHT — Engineering Book`](docs/books/02_HEARTLIGHT_ENGINEERING_BOOK.md)
- 📙 [`HEARTLIGHT — Study Workbook`](docs/books/03_HEARTLIGHT_STUDY_WORKBOOK.md)

### Classroom & support

- [`docs/TEACHER_MANUAL.md`](docs/TEACHER_MANUAL.md)
- [`docs/STUDY_ARC.md`](docs/STUDY_ARC.md)
- [`docs/STIMMING_GUIDE.md`](docs/STIMMING_GUIDE.md)
- [`docs/CLASSROOM_SCENARIOS.md`](docs/CLASSROOM_SCENARIOS.md)
- [`docs/PRINTABLE_SUPPORT_MENU.md`](docs/PRINTABLE_SUPPORT_MENU.md)
- [`docs/FAMILY_GUIDE.md`](docs/FAMILY_GUIDE.md)

### Therapy / related services

- [`docs/THERAPY_USAGE_GUIDE.md`](docs/THERAPY_USAGE_GUIDE.md)
- [`docs/CLINICIAN_ADJUNCT_GUIDE.md`](docs/CLINICIAN_ADJUNCT_GUIDE.md)

### Engineering / research / product

- [`docs/MULTI_LANGUAGE_ARCHITECTURE.md`](docs/MULTI_LANGUAGE_ARCHITECTURE.md)
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/DATA_DICTIONARY.md`](docs/DATA_DICTIONARY.md)
- [`docs/NATIVE_DEVICE_GUIDE.md`](docs/NATIVE_DEVICE_GUIDE.md)
- [`docs/PRIVACY_AND_SAFETY.md`](docs/PRIVACY_AND_SAFETY.md)
- [`docs/RESEARCH_AND_VALIDATION.md`](docs/RESEARCH_AND_VALIDATION.md)
- [`docs/PORTFOLIO_AND_PRODUCT_GUIDE.md`](docs/PORTFOLIO_AND_PRODUCT_GUIDE.md)

---

# 🛡️ Hard safety boundary

HEARTLIGHT is not a medical device, diagnosis engine, psychotherapy replacement, crisis service, lie detector, emotion-recognition system, behavior-risk predictor, restraint/seclusion recommender, automated IEP authority, or automated discipline tool.

The software must never decide punishment, restraint, seclusion, medication, diagnosis, disability classification, eligibility, or denial of access.

### Bio/sensor rule

The native API may carry an explicitly entered or approved-sensor observation such as pulse, but the **reference engine intentionally performs no medical, emotional, danger, compliance, or misconduct inference from that value.** This is an architectural boundary, not a missing feature.

---

# 🔐 Privacy by architecture

The public reference build intentionally contains no account system, advertising, analytics SDK, cloud database, facial recognition, facial emotion recognition, microphone surveillance, hidden child score, or permanent child profile.

Student check-ins are session state. Teacher observations persist only after explicit human action and can be exported or erased locally.

Schools must still perform their own privacy, legal, accessibility, security, and device-approval review before deployment.

---

# 🧪 Verification

### Static privacy/function audit

```bash
python tests/static_audit.py
```

### Native C++

```bash
cmake -S native/cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp --parallel 2
ctest --test-dir build/cpp --output-on-failure
```

### Native Rust

```bash
cargo test --workspace --manifest-path native/rust/Cargo.toml
```

GitHub Actions runs both native language builds through [`.github/workflows/native-build.yml`](.github/workflows/native-build.yml).

Tests verify engineering behavior; they are not FERPA/COPPA/WCAG certification or evidence of clinical efficacy.

---

# 🧰 Repository map

```text
COSMOS-HEARTLIGHT/
├── HEARTLIGHT_STANDALONE.html
├── app/                         # installable PWA
├── contracts/                   # shared state specification
├── native/
│   ├── cpp/                     # C++17 + C ABI + tests
│   ├── rust/                    # Rust core + FFI + tests
│   ├── android/                 # Android integration
│   ├── apple/                   # iOS/iPadOS/macOS integration
│   └── windows/                 # Windows integration
├── docs/
│   ├── books/                   # educator, engineering, workbook
│   ├── teacher/family/therapy guides
│   ├── privacy/research architecture
│   └── portfolio/product guide
├── examples/
├── tests/
├── .github/workflows/
└── LICENSE
```

---

# 💼 Why this repo matters as a portfolio

HEARTLIGHT demonstrates more than a front-end prototype: product design for a real user population, accessibility thinking, offline-first web engineering, C++ systems programming, Rust/FFI design, cross-platform architecture, deterministic algorithms, privacy engineering, CI/testing, documentation, educator workflows, and open-source governance.

See [`docs/PORTFOLIO_AND_PRODUCT_GUIDE.md`](docs/PORTFOLIO_AND_PRODUCT_GUIDE.md) for a technical-interview walkthrough and commercialization paths that do not require selling child data or making unsupported clinical claims.

---

# 🤝 Contributing

The best contribution makes HEARTLIGHT easier to communicate with, more accessible, more private, more transparent, more testable, and less likely to stigmatize or surveil a learner.

**Never submit real student records, faces, recordings, IEP documents, private health information, or identifying classroom data to this public repository.**

Read [`CONTRIBUTING.md`](CONTRIBUTING.md).

---

# 📜 License

Apache License 2.0. See [`LICENSE`](LICENSE).

---

## 💚 The HEARTLIGHT rule

> **Do not ask only “How do we stop this behavior?”**  
> Ask **“What is the learner communicating, what is making access harder, and what safe support can we try together?”**

### Be ambitious with support. Be conservative with surveillance.
