#include "heartlight/heartlight_c.h"
#include "heartlight/heartlight.hpp"

#include <array>

namespace {
heartlight::SupportState from_c(const HeartlightState12& in) {
    return heartlight::SupportState{
        in.values[0], in.values[1], in.values[2], in.values[3],
        in.values[4], in.values[5], in.values[6], in.values[7],
        in.values[8], in.values[9], in.values[10], in.values[11]
    };
}

const std::array<heartlight::Need, 12> kNeeds = {
    heartlight::Need::Quiet, heartlight::Need::Movement, heartlight::Need::DimLight,
    heartlight::Need::Space, heartlight::Need::Help, heartlight::Need::Break,
    heartlight::Need::Pressure, heartlight::Need::SoundChoice, heartlight::Need::Predictability,
    heartlight::Need::AlternateCommunication, heartlight::Need::Unknown, heartlight::Need::Company
};
}  // namespace

extern "C" unsigned int heartlight_evaluate_mask(const HeartlightState12* state,
                                                    unsigned int need_mask,
                                                    const HeartlightEnvironment* environment) {
    if (!state) return 0;
    heartlight::Input input;
    input.state = from_c(*state);
    for (unsigned int i = 0; i < kNeeds.size(); ++i) {
        if (need_mask & (1u << i)) input.needs.push_back(kNeeds[i]);
    }
    if (environment) {
        if (environment->has_brightness) input.environment.ambient_brightness_percent = environment->brightness_percent;
        if (environment->has_rgb) {
            input.environment.ambient_red = environment->red;
            input.environment.ambient_green = environment->green;
            input.environment.ambient_blue = environment->blue;
        }
    }

    const auto result = heartlight::evaluate(input);
    unsigned int mask = 0;
    for (const auto& s : result.suggestions) {
        if (s.id == "quiet-option") mask |= 1u << 0;
        else if (s.id == "movement-option") mask |= 1u << 1;
        else if (s.id == "light-option") mask |= 1u << 2;
        else if (s.id == "space-option") mask |= 1u << 3;
        else if (s.id == "next-step") mask |= 1u << 4;
        else if (s.id == "break-option") mask |= 1u << 5;
        else if (s.id == "pressure-option") mask |= 1u << 6;
        else if (s.id == "sound-choice") mask |= 1u << 7;
        else if (s.id == "predictability") mask |= 1u << 8;
        else if (s.id == "communication-choice") mask |= 1u << 9;
        else if (s.id == "unknown-valid") mask |= 1u << 10;
        else if (s.id == "stay-near") mask |= 1u << 11;
    }
    return mask;
}

extern "C" const char* heartlight_version(void) { return "0.2.0"; }
