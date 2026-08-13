# HEARTLIGHT Privacy, Safety, and Child-Data Rules

## Default architecture

- no account;
- no analytics;
- no ad technology;
- no external API calls in the supplied app;
- no cloud database;
- no facial recognition;
- no emotion recognition from a face;
- no microphone capture;
- no camera frame storage;
- student check-ins are session-only;
- teacher logs save only after an explicit button press;
- all locally stored records can be erased from Settings.

## Why “forever memory” is disabled for children

COSMOS research includes durable memory concepts. HEARTLIGHT deliberately changes that design. A child's temporary stress, movement, communication, or sensory state should not become an irreversible profile.

For school use, memory must be bounded, reviewable, purpose-limited, and deletable.

## FERPA

FERPA protects the privacy of student education records in covered U.S. educational institutions. Whether a specific HEARTLIGHT record becomes an education record depends on how a school uses and maintains it. District review is required before operational deployment.

Primary source: https://studentprivacy.ed.gov/ferpa

## COPPA

COPPA applies to certain online services collecting personal information from children under 13. The Rule was amended in 2025; hosted or modified HEARTLIGHT deployments should be reviewed against the current FTC Rule and their actual data flows.

Primary sources:
- https://www.ftc.gov/legal-library/browse/rules/childrens-online-privacy-protection-rule-coppa
- https://www.ftc.gov/business-guidance/resources/complying-coppa-frequently-asked-questions

## Data minimization

A school deployment should define:

- what is collected;
- why it is needed;
- who can access it;
- how long it is retained;
- how it is deleted;
- whether families/students can inspect/correct it;
- how exports are secured;
- whether any vendor receives it.

## Accessibility

The app targets accessible interaction patterns and should be audited against WCAG 2.2 before production deployment.

Primary source: https://www.w3.org/TR/WCAG22/

## Prohibited product directions

Do not extend HEARTLIGHT into:

- biometric identity recognition;
- automated emotion detection;
- behavioral “risk scores”;
- compliance scoring;
- covert monitoring;
- continuous student audio/video recording;
- advertising/profiling;
- selling or brokering child data;
- automated disciplinary recommendations.
