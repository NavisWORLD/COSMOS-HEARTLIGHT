# 💚 COSMOS HEARTLIGHT

> **An open-source, local-first support system for inclusive classrooms.**  
> Communication • sensory regulation • teacher observation • accessibility • student autonomy

```text
┌──────────────────────────────────────────────────────────────────────┐
│                      C O S M O S   H E A R T L I G H T             │
│                                                                      │
│   CHILD SIGNAL  →  HUMAN UNDERSTANDING  →  GENTLE SUPPORT  →  LEARN │
│                                                                      │
│       no ads • no accounts • no child scoring • local-first          │
└──────────────────────────────────────────────────────────────────────┘
```

HEARTLIGHT is for the learner who may be quiet, overwhelmed, moving, masking, non-speaking, dysregulated, hard to read, or simply being misunderstood.

The project turns ideas from the COSMOS/CST engineering lineage into a bounded school-support loop:

```text
perceive → compress → expand → validate → express → bounded storage
```

In HEARTLIGHT that means: **notice the situation, reduce it to understandable signals, offer more than one interpretation, check the learner's response, communicate clearly, and store as little as necessary.**

---

## 🚀 Choose your path

| I am… | Start here | What you get |
|---|---|---|
| 👩‍🏫 Teacher | [`docs/00_TEACHER_START_HERE.md`](docs/00_TEACHER_START_HERE.md) | 10-minute setup + classroom workflow |
| 🧩 Behavioral aide / paraeducator | [`docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md`](docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md) | neutral observation + support loop |
| 🌱 Student / learner | [`docs/STUDENT_GUIDE.md`](docs/STUDENT_GUIDE.md) | plain-language guide to the app |
| 👨‍👩‍👧 Family member | [`docs/FAMILY_GUIDE.md`](docs/FAMILY_GUIDE.md) | what HEARTLIGHT does and does not do |
| 🧠 OT / SLP / therapist / clinician | [`docs/CLINICIAN_ADJUNCT_GUIDE.md`](docs/CLINICIAN_ADJUNCT_GUIDE.md) | adjunct use without automated diagnosis |
| 🏫 School / district | [`docs/SCHOOL_DEPLOYMENT.md`](docs/SCHOOL_DEPLOYMENT.md) | privacy, approval, deployment checklist |
| 💻 Developer / researcher | [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | architecture, 12D state, extension points |

---

## ✨ What the app can do

### 💬 Give a learner another way to say what they need
Learners can choose needs such as:

- quiet
- movement
- less light
- more space
- a break
- help beginning
- pressure/proprioceptive input
- a different sound environment
- knowing what happens next
- another way to communicate
- “I don’t know”
- “stay with me”

**Speech is not treated as the only valid form of communication.**

### 🌈 Optional ambient color / brightness sensing
HEARTLIGHT can sample the **room or light source** through the rear camera and estimate RGB/brightness locally.

It does **not** need to photograph the learner. Camera frames are not intentionally retained. The sensor can suggest checking glare or readability, but **learner preference outranks the sensor**.

### 🌀 Support stimming and regulation instead of policing it
The included guide treats repetitive movement/sound as potentially communicative or regulatory. The goal is not to extinguish harmless stimming. See [`docs/STIMMING_GUIDE.md`](docs/STIMMING_GUIDE.md).

### 🧑‍🏫 Help adults observe without pretending to read minds
Teacher notes separate:

```text
context → observable behavior → support offered → observable result
```

That structure is deliberately different from labels like “defiant,” “lazy,” or “attention seeking.”

### ⏱️ Make time visible
A simple visual timer can support breaks, work periods, transitions, and predictable return points.

### 🧭 Transparent 12-dimensional support state
HEARTLIGHT exposes a **situation model**, not a diagnosis:

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

These dimensions describe **right now**, not “what kind of child this is.” See [`docs/DATA_DICTIONARY.md`](docs/DATA_DICTIONARY.md).

---

# 📲 Use HEARTLIGHT right now

## Option A — easiest: one file

Download [`HEARTLIGHT_STANDALONE.html`](HEARTLIGHT_STANDALONE.html) and open it in a modern browser.

That gives you the core application with no install command and no account.

> Camera access normally requires HTTPS or `localhost`; the rest of the standalone app works without camera permission.

## Option B — run the full PWA locally

Requires Python 3.10+ only as a tiny static web server:

```bash
python run_local.py
```

Open:

```text
http://127.0.0.1:8877
```

## Option C — install it like an app

Host the `app/` folder over HTTPS.

- **iPhone / iPad:** Safari → Share → **Add to Home Screen**
- **Android:** Chrome → **Install app** / **Add to Home screen**
- **Windows / ChromeOS / macOS:** Chrome or Edge → **Install**

The PWA is dependency-free HTML/CSS/JavaScript and works offline after its first successful load.

Full packaging notes: [`docs/NATIVE_PACKAGING.md`](docs/NATIVE_PACKAGING.md).

---

# 👩‍🏫 Teacher mode in 10 minutes

```text
1. ASK       Give the learner access to the Child screen.
2. LISTEN    Treat their selection as communication, not a verdict.
3. OFFER     Choose one or two low-risk supports.
4. WAIT      Allow processing time.
5. OBSERVE   Record only what you can actually observe.
6. CHECK     Ask whether the support helped.
7. ADAPT     Change the environment before blaming the learner.
8. ERASE     Keep only records your school actually needs.
```

Start with [`docs/00_TEACHER_START_HERE.md`](docs/00_TEACHER_START_HERE.md), then use the full [`docs/TEACHER_MANUAL.md`](docs/TEACHER_MANUAL.md).

Printable/quick references:

- [`docs/PRINTABLE_SUPPORT_MENU.md`](docs/PRINTABLE_SUPPORT_MENU.md)
- [`docs/CLASSROOM_SCENARIOS.md`](docs/CLASSROOM_SCENARIOS.md)
- [`docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md`](docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md)

---

# 📚 HEARTLIGHT learning library

## Classroom & support
- [`docs/00_TEACHER_START_HERE.md`](docs/00_TEACHER_START_HERE.md)
- [`docs/TEACHER_MANUAL.md`](docs/TEACHER_MANUAL.md)
- [`docs/STUDY_ARC.md`](docs/STUDY_ARC.md)
- [`docs/STIMMING_GUIDE.md`](docs/STIMMING_GUIDE.md)
- [`docs/STUDENT_GUIDE.md`](docs/STUDENT_GUIDE.md)
- [`docs/FAMILY_GUIDE.md`](docs/FAMILY_GUIDE.md)
- [`docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md`](docs/BEHAVIORAL_AIDE_QUICK_GUIDE.md)
- [`docs/CLASSROOM_SCENARIOS.md`](docs/CLASSROOM_SCENARIOS.md)

## Therapy / related services
- [`docs/THERAPY_USAGE_GUIDE.md`](docs/THERAPY_USAGE_GUIDE.md)
- [`docs/CLINICIAN_ADJUNCT_GUIDE.md`](docs/CLINICIAN_ADJUNCT_GUIDE.md)

## Engineering / research
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/DATA_DICTIONARY.md`](docs/DATA_DICTIONARY.md)
- [`docs/RESEARCH_AND_VALIDATION.md`](docs/RESEARCH_AND_VALIDATION.md)
- [`docs/PRIVACY_AND_SAFETY.md`](docs/PRIVACY_AND_SAFETY.md)
- [`docs/SCHOOL_DEPLOYMENT.md`](docs/SCHOOL_DEPLOYMENT.md)
- [`docs/NATIVE_PACKAGING.md`](docs/NATIVE_PACKAGING.md)

---

# 🛡️ Hard safety boundary

HEARTLIGHT is **not**:

- a medical device
- a diagnostic test
- psychotherapy
- a crisis service
- a lie detector
- an emotion-recognition system
- a behavior-risk predictor
- a restraint/seclusion recommender
- an automated IEP or eligibility engine
- an automated discipline tool
- a substitute for student/family consent or qualified professional judgment

The software must never decide punishment, restraint, seclusion, disability classification, medication, diagnosis, special-education eligibility, or denial of access.

The learner's direct communication, dignity, safety, and established school/clinical plans outrank software output.

---

# 🔐 Privacy by architecture

The public reference build intentionally contains:

- **no account system**
- **no advertising**
- **no analytics SDK**
- **no cloud database**
- **no facial recognition**
- **no facial emotion recognition**
- **no microphone surveillance**
- **no hidden child score**
- **no permanent child profile**

Student check-ins are session state. Teacher observations are saved only after an explicit human action and can be exported or erased locally.

Schools must still conduct their own privacy/legal review before deployment. U.S. schools should consult district administration/IT regarding FERPA, COPPA, PPRA, state law, record-retention rules, accessibility requirements, and approved-device policies.

See [`docs/PRIVACY_AND_SAFETY.md`](docs/PRIVACY_AND_SAFETY.md) and [`docs/SCHOOL_DEPLOYMENT.md`](docs/SCHOOL_DEPLOYMENT.md).

---

# 🧪 Verify the repository

Run the included static audit:

```bash
python tests/static_audit.py
```

Expected result:

```text
HEARTLIGHT static privacy/function audit: PASS
```

The audit checks important privacy/safety assumptions in the static reference application. It is **not** a certification of FERPA/COPPA/WCAG compliance or clinical effectiveness.

---

# 🧬 Architecture lineage

HEARTLIGHT adapts the COSMOS/CST engineering pattern into an education-safe form:

```text
INPUT
  │
  ├── learner-selected needs
  ├── learner self-ratings
  ├── adult observable context
  └── optional room-light sample
  │
  ▼
BOUNDED STATE ENGINE
  │
  ├── 12D situation state
  ├── transparent rule routing
  └── no hidden diagnosis
  │
  ▼
SUPPORT OPTIONS
  │
  ├── reduce load
  ├── add predictability
  ├── offer movement / break
  ├── offer communication options
  └── preserve learner choice
  │
  ▼
HUMAN VALIDATION
  │
  └── Did it actually help this learner?
  │
  ▼
BOUNDED MEMORY
     └── save only deliberate adult observations; allow export/erase
```

Read the detailed implementation in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

---

# 🏫 Evidence-informed direction

The project is designed around a few deliberately conservative principles:

- proactive, positive behavioral supports rather than punishment-first automation
- direct learner communication and autonomy
- data minimization and deletion
- school/district approval before classroom use of software handling education data
- accessible interfaces and multiple ways to communicate
- measurement of **support response**, not claims that software can infer a child's inner state

Primary references are maintained in the manuals, including U.S. Department of Education IDEA/student-privacy resources, FTC COPPA guidance, and W3C WCAG 2.2.

---

# 🧰 Repository map

```text
COSMOS-HEARTLIGHT/
├── README.md
├── HEARTLIGHT_STANDALONE.html     ← easiest single-file app
├── app/
│   ├── index.html                 ← installable PWA
│   ├── manifest.webmanifest
│   ├── sw.js
│   └── icon.svg
├── docs/
│   ├── 00_TEACHER_START_HERE.md
│   ├── TEACHER_MANUAL.md
│   ├── STUDY_ARC.md
│   ├── STIMMING_GUIDE.md
│   ├── THERAPY_USAGE_GUIDE.md
│   ├── CLINICIAN_ADJUNCT_GUIDE.md
│   ├── STUDENT_GUIDE.md
│   ├── FAMILY_GUIDE.md
│   ├── BEHAVIORAL_AIDE_QUICK_GUIDE.md
│   ├── CLASSROOM_SCENARIOS.md
│   ├── PRINTABLE_SUPPORT_MENU.md
│   ├── SCHOOL_DEPLOYMENT.md
│   ├── PRIVACY_AND_SAFETY.md
│   ├── ARCHITECTURE.md
│   ├── DATA_DICTIONARY.md
│   ├── RESEARCH_AND_VALIDATION.md
│   └── NATIVE_PACKAGING.md
├── examples/
│   └── classroom_plan.json
├── tests/
│   └── static_audit.py
├── run_local.py
├── CONTRIBUTING.md
├── SECURITY.md
├── CITATION.cff
├── NOTICE
└── LICENSE
```

---

# 🤝 Contributing

The best contributions make HEARTLIGHT:

- easier for a learner to communicate
- easier for a teacher to understand
- more accessible
- more private
- more transparent
- more testable
- less likely to stigmatize or surveil a child

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) before proposing changes.

**Never submit real student data, identifying classroom records, faces, recordings, IEP documents, or private health information to this public repository.**

---

# 📜 License

Apache License 2.0. See [`LICENSE`](LICENSE).

---

## 💚 The HEARTLIGHT rule

> **Do not ask only “How do we stop this behavior?”**  
> Ask **“What is the learner communicating, what is making access harder, and what safe support can we try together?”**

### Be ambitious with support. Be conservative with surveillance.
