#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define HEARTLIGHT_STATE_DIMENSIONS 12

typedef struct HeartlightState12 {
    double values[HEARTLIGHT_STATE_DIMENSIONS];
} HeartlightState12;

typedef struct HeartlightEnvironment {
    int has_brightness;
    double brightness_percent;
    int has_rgb;
    int red;
    int green;
    int blue;
} HeartlightEnvironment;

// Returns a bitmask of recommended support categories.
// This small C ABI is intentionally stable for Swift, Kotlin/JNI, C#,
// Python ctypes, game engines, and other language bridges.
unsigned int heartlight_evaluate_mask(const HeartlightState12* state,
                                      unsigned int need_mask,
                                      const HeartlightEnvironment* environment);

const char* heartlight_version(void);

#ifdef __cplusplus
}
#endif
