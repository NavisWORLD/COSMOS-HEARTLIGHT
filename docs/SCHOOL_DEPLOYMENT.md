# 🏫 HEARTLIGHT School & District Deployment Guide

This document is a technical/privacy deployment guide, not legal advice.

## Recommended deployment posture

Use HEARTLIGHT as a **local-first support utility**, not as a centralized student-profile system.

The reference implementation is intentionally designed to avoid accounts, cloud analytics, advertising, facial recognition, facial emotion recognition, microphone surveillance, and automatic behavioral scoring.

## Approval before classroom use

For U.S. schools, teachers should check district/school approval processes before using an online application that may handle student education information. District administration and IT should determine whether the service is appropriate and consistent with FERPA and local policy.

The FTC's current COPPA guidance also places responsibility on covered online-service operators and explains the limited circumstances in which schools may authorize collection on behalf of parents for school-authorized educational purposes.

## Suggested deployment tiers

### Tier 0 — communication-only

- Student uses Child screen.
- Nothing is saved.
- Camera sensor disabled.
- No identifiers entered.

This is the lowest-data configuration.

### Tier 1 — local teacher observation

- Child screen plus teacher observations.
- Observations saved only to a school-managed device.
- Use a school-approved code rather than student name where feasible.
- Establish local deletion/retention routine.

### Tier 2 — institutionally integrated

If a district modifies HEARTLIGHT to send data to servers, identity providers, analytics systems, SIS platforms, EHRs, or external APIs, the privacy properties of the reference build no longer describe the deployed system.

Before deployment, complete security/privacy/legal review, access-control design, retention/destruction rules, incident response, vendor review, and required notices/consents.

## Do not add by default

Avoid introducing:

- ad-tech SDKs
- cross-site tracking
- third-party analytics tied to student identifiers
- facial recognition
- facial expression/emotion classification
- continuous audio capture
- hidden behavior-risk scores
- automated discipline recommendations
- generative AI that sends identifiable student records to an unapproved external service

## Data inventory for the reference build

### Session-only child data

- selected support needs
- four slider values describing current self-report
- computed 12D support state

These are held in browser memory and are not intentionally persisted by the Child screen.

### Optional local adult records

The Teacher screen can save:

- timestamp
- optional student code
- context
- observable behavior
- support given
- result
- support-state snapshot

These records use browser local storage until erased/exported.

### Camera

The Color Sensor obtains a live rear-camera stream when the user explicitly grants permission. Sampled frames are used to calculate approximate RGB/brightness and are not intentionally written to storage by the reference application.

## District checklist

- [ ] Confirm educational purpose.
- [ ] Decide whether learner participation is optional and how refusal is respected.
- [ ] Confirm approved-device/browser policy.
- [ ] Review FERPA/PPRA requirements and state student-privacy law.
- [ ] Review COPPA obligations if applicable.
- [ ] Review accessibility requirements and test with actual assistive technology used in the district.
- [ ] Decide whether camera functionality is permitted.
- [ ] Document what data is retained and why.
- [ ] Define deletion schedule.
- [ ] Define who can access saved observation data.
- [ ] Define export/storage location if observations become education records.
- [ ] Train staff on neutral observation language.
- [ ] Train staff that software output cannot automate diagnosis, discipline, restraint, seclusion, IEP eligibility, or placement.
- [ ] Provide families with plain-language information about the deployed configuration.
- [ ] Test offline behavior and device wiping procedures.
- [ ] Re-review any fork that adds cloud services or external AI.

## FERPA-oriented questions for any modified deployment

- What student PII is collected?
- Is each field necessary for the stated educational purpose?
- Who controls the data?
- Who can access it?
- Is it redisclosed?
- Is it used for any non-educational/commercial purpose?
- How can records be reviewed/corrected/deleted where required?
- What happens when a student leaves the school?
- What happens when a device is lost?

## COPPA-oriented questions for child-directed online use

The COPPA Rule was amended in 2025. For covered operators, review the current FTC Rule and FAQ rather than relying on an old checklist.

Questions include:

- Is the service covered by COPPA?
- What personal information is collected from children under 13?
- Is collection reasonably necessary?
- Who provides authorization/consent where applicable?
- Is data used only for the authorized educational purpose?
- Can the school/parent review or request deletion where applicable?
- How is confidentiality/security maintained?
- How long is information retained?

## Accessibility

HEARTLIGHT should be tested against current WCAG 2.2 expectations and with real users/assistive technology. The presence of large controls, keyboard-friendly elements, contrast options, and reduced motion does not itself constitute a conformance claim.

## Current primary references

- U.S. Department of Education IDEA Behavioral Support Resources: https://sites.ed.gov/idea/behavioral-support-resources/
- U.S. Department of Education Student Privacy / FERPA: https://studentprivacy.ed.gov/
- Student Privacy FAQ for classroom online tools: https://studentprivacy.ed.gov/faq/i-want-use-online-tool-or-application-part-my-course-however-i-am-worried-it-violation-ferpa
- FTC COPPA FAQ: https://www.ftc.gov/business-guidance/resources/complying-coppa-frequently-asked-questions
- FTC COPPA Rule: https://www.ftc.gov/legal-library/browse/rules/childrens-online-privacy-protection-rule-coppa
- W3C WCAG 2.2: https://www.w3.org/TR/WCAG22/
