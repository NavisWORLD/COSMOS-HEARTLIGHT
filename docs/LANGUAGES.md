# HEARTLIGHT Languages

HEARTLIGHT is designed to be multilingual without sending learner, teacher, sensor, or observation text to a cloud translation service.

## Shipped languages

- **English (`en`)** — canonical fallback language.
- **Spanish / Español (`es`)** — complete supported interface locale for the child support screen, teacher/aide tools, sensor guidance, study content, safety language, accessibility settings, alerts, and support suggestions.

## How language selection works

### Web, Windows, Android, and PWA

The core application loads `app/i18n.js`. A language selector is added to HEARTLIGHT itself. The selected locale is saved only in local browser/application storage under `heartlight.language`.

If no preference has been saved, HEARTLIGHT reads the device/browser language. Devices configured for Spanish start in Spanish; other languages fall back to English.

Changing language does **not** alter saved teacher observation data, the CSV field schema, student codes, or the 12D numerical support state. This keeps records portable and avoids splitting data by interface language.

### iPhone, iPad, and macOS native app

The SwiftUI application ships Apple localization resources in `native/apple_app/Resources/`. Apple builds follow the operating-system/app language selection. Spanish camera-permission wording is localized as well.

## Privacy rule

Localization is deterministic and local. HEARTLIGHT does not require Google Translate, Microsoft Translator, an LLM API, analytics, advertising SDKs, or a remote translation backend.

## Adding another language

For the web-family application, extend the locale map in `app/i18n.js`. For Apple platforms, add another `<locale>.lproj/Localizable.strings` directory under `native/apple_app/Resources/` and, when needed, an `InfoPlist.strings` file for permission text.

A new locale should translate the entire learner-facing and adult-facing safety surface, not only navigation labels. At minimum review:

1. learner need/communication buttons;
2. support suggestions and timer language;
3. teacher observation labels and warnings;
4. camera/sensor permission and safety guidance;
5. study/stimming/accessibility text;
6. privacy and human-decision boundaries;
7. dynamic alerts, confirmations, and status messages.

Translations should be reviewed by fluent speakers familiar with the educational context before being described as production-ready for a school deployment.
