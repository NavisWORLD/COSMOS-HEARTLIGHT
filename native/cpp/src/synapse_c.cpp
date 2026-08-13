#include "heartlight/synapse_c.h"
#include "heartlight/synapse.hpp"

#include <algorithm>

extern "C" void heartlight_synapse_default_config(HeartlightSynapseConfig* out_config) {
    if (!out_config) return;
    const heartlight::synapse::Config c{};
    out_config->retention = c.retention;
    out_config->learning_rate = c.learning_rate;
    out_config->weight_decay = c.weight_decay;
    out_config->association_gain = c.association_gain;
    out_config->max_weight = c.max_weight;
}

extern "C" int heartlight_synapse_step(const double state[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                                         const double stimulus[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                                         const double weights[HEARTLIGHT_SYNAPSE_WEIGHT_COUNT],
                                         const HeartlightSynapseConfig* config,
                                         double state_out[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                                         double weights_out[HEARTLIGHT_SYNAPSE_WEIGHT_COUNT]) {
    if (!state || !stimulus || !weights || !state_out || !weights_out) return 0;

    heartlight::synapse::State s{};
    heartlight::synapse::State u{};
    heartlight::synapse::Weights w{};
    std::copy_n(state, HEARTLIGHT_SYNAPSE_DIMENSIONS, s.begin());
    std::copy_n(stimulus, HEARTLIGHT_SYNAPSE_DIMENSIONS, u.begin());
    std::copy_n(weights, HEARTLIGHT_SYNAPSE_WEIGHT_COUNT, w.begin());

    heartlight::synapse::Config cfg{};
    if (config) {
        cfg.retention = config->retention;
        cfg.learning_rate = config->learning_rate;
        cfg.weight_decay = config->weight_decay;
        cfg.association_gain = config->association_gain;
        cfg.max_weight = config->max_weight;
    }
    const auto result = heartlight::synapse::step(s, u, w, cfg);
    std::copy(result.state.begin(), result.state.end(), state_out);
    std::copy(result.weights.begin(), result.weights.end(), weights_out);
    return 1;
}

extern "C" double heartlight_synapse_hebbian_delta(double pre, double post, double weight,
                                                      double learning_rate, double decay) {
    return heartlight::synapse::hebbian_delta(pre, post, weight, learning_rate, decay);
}

extern "C" const char* heartlight_synapse_version(void) {
    return heartlight::synapse::version();
}
