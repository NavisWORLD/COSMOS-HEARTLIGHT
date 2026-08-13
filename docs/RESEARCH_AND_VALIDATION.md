# HEARTLIGHT Research & Validation Manual

## Claim discipline

HEARTLIGHT currently demonstrates a software architecture and human-support workflow. It does **not** establish clinical efficacy, therapeutic benefit, diagnostic validity, or improved academic outcomes.

## Validation ladder

### Level 0 — software correctness
- app loads offline;
- controls work with keyboard/touch;
- logs export correctly;
- erase actually erases;
- camera off means camera tracks stop;
- no network calls except initial static-file fetches.

### Level 1 — accessibility/usability
Test with diverse students and adults using appropriate consent and ethics oversight. Include AAC users, nonspeaking users, low-vision users, motor-access users, and users with cognitive/learning disabilities.

### Level 2 — implementation fidelity
Measure whether adults use neutral observation language, offer choices, and avoid turning support data into compliance scores.

### Level 3 — educational outcomes
Pre-register outcomes such as task access, student-reported usefulness, successful transitions, participation, and reduction in preventable removals from instruction.

### Level 4 — comparative research
Compare HEARTLIGHT-supported practice against an appropriate control/usual-practice condition. Do not withhold required accommodations or services.

## Reproducibility

For future studies:

- freeze app version/hash;
- publish the algorithm mapping;
- pre-register primary outcomes;
- preserve null results;
- report attrition and missing data;
- use multiple sites when possible;
- include learner/family input;
- conduct subgroup fairness analyses;
- separate exploratory from confirmatory findings.

## Falsification questions

HEARTLIGHT should be considered unsuccessful or in need of redesign if, for example:

- students report that it increases pressure or surveillance;
- adults use it to justify discipline rather than supports;
- sensor suggestions routinely disagree with learner preference;
- accessibility barriers exclude intended users;
- data retention grows beyond the defined purpose;
- outcomes do not improve in adequately powered studies.
