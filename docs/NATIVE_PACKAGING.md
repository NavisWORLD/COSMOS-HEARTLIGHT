# Native Packaging Guide

HEARTLIGHT is already an installable PWA. Native app-store packages can be added without changing the core.

## iOS / Android

Recommended wrapper: Capacitor or another minimal WebView shell.

Production steps:

1. Serve/build the `app/` directory as the web asset root.
2. Configure camera permission text to explain that the optional sensor samples the room and does not intentionally save frames.
3. Do not request microphone, contacts, precise location, advertising ID, or background camera permission.
4. Ensure the app works fully when camera permission is denied.
5. Run platform accessibility checks and privacy-manifest reviews.
6. Have the deploying school/entity complete legal/privacy review before collecting student records.

## Windows

The PWA can be installed directly through Edge/Chrome. A Windows Store package can later wrap the same PWA.

## Offline-only school deployment

For devices on a managed school network, host the static `app/` folder on a district HTTPS server or managed device web server. No external CDN is required.
