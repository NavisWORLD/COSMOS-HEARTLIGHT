#pragma once

#include <optional>
#include <string>
#include <vector>

namespace heartlight {

enum class Need {
    Quiet,
    Movement,
    DimLight,
    Space,
    Help,
    Break,
    Pressure,
    SoundChoice,
    Predictability,
    AlternateCommunication,
    Unknown,
    Company
};

struct SupportState {
    double sensory_load{5.0};
    double visual_load{5.0};
    double movement_need{5.0};
    double focus_access{5.0};
    double transition_need{5.0};
    double communication_load{5.0};
    double social_space_need{5.0};
    double body_comfort{5.0};
    double predictability_need{5.0};
    double recovery_need{5.0};
    double engagement_access{5.0};
    double regulation_confidence{5.0};
};

struct EnvironmentObservation {
    std::optional<double> ambient_brightness_percent{};
    std::optional<int> ambient_red{};
    std::optional<int> ambient_green{};
    std::optional<int> ambient_blue{};
};

struct BioObservation {
    // Optional manually-entered or approved-sensor observation only.
    // The reference engine intentionally does not infer diagnosis, emotion,
    // danger, compliance, or behavior risk from this value.
    std::optional<int> pulse_bpm{};
};

struct Input {
    SupportState state{};
    std::vector<Need> needs{};
    EnvironmentObservation environment{};
    BioObservation bio{};
};

struct Suggestion {
    std::string id;
    std::string title;
    std::string rationale;
    int priority{0};
    bool requires_consent{false};
};

struct Result {
    SupportState normalized_state{};
    std::vector<Suggestion> suggestions{};
    std::vector<std::string> notices{};
};

SupportState normalize(const SupportState& state);
Result evaluate(const Input& input);
std::string need_name(Need need);
std::string version();

}  // namespace heartlight
