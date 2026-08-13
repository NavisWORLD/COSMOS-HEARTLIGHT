#include "heartlight/heartlight.hpp"
#include <cassert>
#include <string>

int main() {
    using namespace heartlight;

    SupportState dirty;
    dirty.sensory_load = 99.0;
    dirty.focus_access = -4.0;
    auto clean = normalize(dirty);
    assert(clean.sensory_load == 10.0);
    assert(clean.focus_access == 0.0);

    Input input;
    input.state.sensory_load = 8.0;
    input.state.regulation_confidence = 2.0;
    input.needs = {Need::Quiet, Need::Unknown};
    input.bio.pulse_bpm = 100;
    auto result = evaluate(input);
    assert(!result.suggestions.empty());
    bool saw_quiet = false;
    bool saw_unknown = false;
    for (const auto& s : result.suggestions) {
        if (s.id == "quiet-option") saw_quiet = true;
        if (s.id == "unknown-valid") saw_unknown = true;
    }
    assert(saw_quiet && saw_unknown);
    bool bio_notice = false;
    for (const auto& n : result.notices) {
        if (n.find("no medical or emotional interpretation") != std::string::npos) bio_notice = true;
    }
    assert(bio_notice);
    return 0;
}
