# HEARTLIGHT Data Dictionary

## Ephemeral child inputs

These remain in page memory only and are cleared on reload.

- selected support needs
- energy/movement need 0–10
- sensory load 0–10
- focus access 0–10
- regulation confidence 0–10

## Teacher observation record

Saved only after explicit action:

- `time` — ISO timestamp
- `studentCode` — optional pseudonymous code
- `context` — neutral context/antecedent note
- `behavior` — observable action/communication
- `support` — support offered
- `result` — observable/communicated outcome
- `supportState` — snapshot of current transparent 12D values

## Sensor data

Current numeric RGB and brightness sample exists in memory while the sensor is on. It is not persisted by the reference app.

## Sensitive data warning

Do not enter full names, diagnoses, medications, addresses, contact information, detailed health records, or other unnecessary identifiers into the teacher log.
