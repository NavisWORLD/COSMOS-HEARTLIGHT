#include "heartlight/heartlight.hpp"
#include <iostream>

int main() {
    heartlight::Input input;
    input.state.sensory_load = 8.0;
    input.state.visual_load = 8.0;
    input.state.regulation_confidence = 2.0;
    input.needs = {heartlight::Need::Quiet, heartlight::Need::DimLight, heartlight::Need::Unknown};
    input.environment.ambient_brightness_percent = 91.0;

    const auto result = heartlight::evaluate(input);
    std::cout << "HEARTLIGHT C++ " << heartlight::version() << "\n";
    for (const auto& s : result.suggestions) {
        std::cout << "[" << s.priority << "] " << s.title << " — " << s.rationale << "\n";
    }
    return 0;
}
