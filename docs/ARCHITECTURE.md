# HEARTLIGHT Architecture

## 1. Design goals

- works on low-cost devices;
- offline-capable;
- local-first;
- transparent deterministic logic;
- multiple communication modes;
- no mandatory biometrics;
- safe when sensors are unavailable;
- child preferences outrank sensor interpretation.

## 2. Runtime

The shipped implementation is a static PWA:

- `app/index.html` — interface + deterministic support engine
- `app/manifest.webmanifest` — install metadata
- `app/sw.js` — offline cache
- `app/icon.svg` — app icon
- `run_local.py` — Python 3.10+ local server

No build step and no third-party JavaScript dependencies are required.

## 3. CST classroom projection

The 12-dimensional support-state vector is:

1. `sensory_load`
2. `visual_load`
3. `movement_need`
4. `focus_access`
5. `transition_need`
6. `communication_load`
7. `social_space_need`
8. `body_comfort`
9. `predictability_need`
10. `recovery_need`
11. `engagement_access`
12. `regulation_confidence`

Values are 0–10 and are produced only from explicit check-in controls and selected needs. They are not latent diagnoses and are not trained on children.

## 4. Lineage mapping

### Perceive
Child selection, teacher observation, optional room light/color sample.

### Compress
Convert inputs to a small support-state vector.

### Expand
Generate multiple low-risk environmental/communication options.

### Validate
Require human/learner judgment. Sensor signals never override learner preference.

### Express
Display plain-language choices.

### Store
Keep only explicit teacher notes locally. Child state remains ephemeral.

## 5. Ambient color sensor

`getUserMedia()` receives a rear-camera stream when permission is granted. A 32x32 canvas samples downscaled frames. Approximately every 700 ms, the app averages sparse RGB samples and computes perceptual brightness:

`Y = 0.2126 R + 0.7152 G + 0.0722 B`

The current implementation retains only the latest numeric sample in memory. It does not call `toDataURL`, `MediaRecorder`, upload APIs, or persistent frame storage.

The sensor supports only simple environmental suggestions such as glare reduction. It must not be aimed at students for face/emotion analysis.

## 6. Persistence

`localStorage` keys:

- `heartlight.logs`
- `heartlight.settings`

No selected child need is saved. No sensor frame/sample history is saved.

## 7. Extension API philosophy

Future adapters may add AAC integration, school SSO, encrypted district storage, external heart-rate monitors, or clinician-approved sensors, but they should preserve:

- explicit opt-in;
- least data necessary;
- inspectable mappings;
- raw-media avoidance;
- no automated diagnosis/discipline;
- deletion and export controls;
- provenance/versioning for algorithms.
