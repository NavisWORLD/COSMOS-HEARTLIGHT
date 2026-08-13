#include "heartlight/heartlight.hpp"

#include <algorithm>
#include <cmath>
#include <set>
#include <sstream>

namespace heartlight {
namespace {

double clamp10(double value) {
    if (std::isnan(value) || std::isinf(value)) return 5.0;
    return std::clamp(value, 0.0, 10.0);
}

bool has_need(const std::vector<Need>& needs, Need target) {
    return std::find(needs.begin(), needs.end(), target) != needs.end();
}

void add_unique(std::vector<Suggestion>& out, std::set<std::string>& seen,
                std::string id, std::string title, std::string rationale,
                int priority, bool requires_consent = false) {
    if (!seen.insert(id).second) return;
    out.push_back(Suggestion{std::move(id), std::move(title), std::move(rationale), priority, requires_consent});
}

}  // namespace

SupportState normalize(const SupportState& s) {
    SupportState n = s;
    n.sensory_load = clamp10(s.sensory_load);
    n.visual_load = clamp10(s.visual_load);
    n.movement_need = clamp10(s.movement_need);
    n.focus_access = clamp10(s.focus_access);
    n.transition_need = clamp10(s.transition_need);
    n.communication_load = clamp10(s.communication_load);
    n.social_space_need = clamp10(s.social_space_need);
    n.body_comfort = clamp10(s.body_comfort);
    n.predictability_need = clamp10(s.predictability_need);
    n.recovery_need = clamp10(s.recovery_need);
    n.engagement_access = clamp10(s.engagement_access);
    n.regulation_confidence = clamp10(s.regulation_confidence);
    return n;
}

Result evaluate(const Input& input) {
    Result result;
    result.normalized_state = normalize(input.state);
    const auto& s = result.normalized_state;
    std::set<std::string> seen;

    if (has_need(input.needs, Need::Quiet) || s.sensory_load >= 7.0) {
        add_unique(result.suggestions, seen, "quiet-option", "Offer a quieter option",
                   "High sensory load or an explicit quiet request can justify reducing competing sound.", 90);
    }
    if (has_need(input.needs, Need::Movement) || s.movement_need >= 7.0) {
        add_unique(result.suggestions, seen, "movement-option", "Offer safe movement",
                   "Movement can support access and regulation. Offer a safe, non-punitive movement choice.", 85);
    }
    if (has_need(input.needs, Need::DimLight) || s.visual_load >= 7.0 ||
        (input.environment.ambient_brightness_percent && *input.environment.ambient_brightness_percent >= 80.0)) {
        add_unique(result.suggestions, seen, "light-option", "Reduce glare or brightness",
                   "The learner requested less light, visual load is high, or the room reading is very bright.", 85);
    }
    if (has_need(input.needs, Need::Space) || s.social_space_need >= 7.0) {
        add_unique(result.suggestions, seen, "space-option", "Offer more space",
                   "Provide physical space without using isolation as punishment.", 80);
    }
    if (has_need(input.needs, Need::Help) || s.focus_access <= 3.0 || s.engagement_access <= 3.0) {
        add_unique(result.suggestions, seen, "next-step", "Show one visible next step",
                   "Lowering task complexity can improve access when focus or engagement feels difficult.", 80);
    }
    if (has_need(input.needs, Need::Break) || s.recovery_need >= 7.0) {
        add_unique(result.suggestions, seen, "break-option", "Offer a flexible break",
                   "A predictable, non-punitive break may support recovery and return to learning.", 90);
    }
    if (has_need(input.needs, Need::Pressure)) {
        add_unique(result.suggestions, seen, "pressure-option", "Offer familiar proprioceptive choices",
                   "Use only learner-preferred options such as wall pushes or carrying a light classroom item. Never impose touch.", 70, true);
    }
    if (has_need(input.needs, Need::SoundChoice)) {
        add_unique(result.suggestions, seen, "sound-choice", "Offer a sound-level choice",
                   "The learner explicitly requested a different sound environment.", 75);
    }
    if (has_need(input.needs, Need::Predictability) || s.predictability_need >= 7.0 || s.transition_need >= 7.0) {
        add_unique(result.suggestions, seen, "predictability", "Make the next transition visible",
                   "A first-then card, visual schedule, or warning can reduce uncertainty.", 85);
    }
    if (has_need(input.needs, Need::AlternateCommunication) || s.communication_load >= 7.0) {
        add_unique(result.suggestions, seen, "communication-choice", "Offer another way to communicate",
                   "AAC, typing, pointing, drawing, gesture, and yes/no choices are valid communication paths.", 95);
    }
    if (has_need(input.needs, Need::Unknown) || s.regulation_confidence <= 3.0) {
        add_unique(result.suggestions, seen, "unknown-valid", "Reduce demands and offer two simple choices",
                   "Not knowing what would help is valid information; simplify the next decision.", 95);
    }
    if (has_need(input.needs, Need::Company)) {
        add_unique(result.suggestions, seen, "stay-near", "Stay nearby without crowding",
                   "The learner requested supportive presence. Follow their preferred distance and communication style.", 90, true);
    }

    if (result.suggestions.empty()) {
        add_unique(result.suggestions, seen, "ask-first", "Ask what would make learning easier",
                   "No strong rule fired. The learner's preference is more important than forcing a recommendation.", 50);
    }

    std::sort(result.suggestions.begin(), result.suggestions.end(), [](const Suggestion& a, const Suggestion& b) {
        if (a.priority != b.priority) return a.priority > b.priority;
        return a.id < b.id;
    });

    result.notices.push_back("HEARTLIGHT suggestions are educational support options, not diagnoses or treatment instructions.");
    result.notices.push_back("Human review and learner preference remain authoritative.");
    if (input.bio.pulse_bpm) {
        std::ostringstream oss;
        oss << "Pulse observation present (" << *input.bio.pulse_bpm
            << " bpm). Reference engine stores no medical or emotional interpretation.";
        result.notices.push_back(oss.str());
    }
    return result;
}

std::string need_name(Need need) {
    switch (need) {
        case Need::Quiet: return "quiet";
        case Need::Movement: return "movement";
        case Need::DimLight: return "dim-light";
        case Need::Space: return "space";
        case Need::Help: return "help";
        case Need::Break: return "break";
        case Need::Pressure: return "pressure";
        case Need::SoundChoice: return "sound-choice";
        case Need::Predictability: return "predictability";
        case Need::AlternateCommunication: return "alternate-communication";
        case Need::Unknown: return "unknown";
        case Need::Company: return "company";
    }
    return "unknown";
}

std::string version() { return "0.2.0"; }

}  // namespace heartlight
