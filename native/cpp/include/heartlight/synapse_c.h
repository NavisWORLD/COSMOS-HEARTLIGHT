#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define HEARTLIGHT_SYNAPSE_DIMENSIONS 12
#define HEARTLIGHT_SYNAPSE_WEIGHT_COUNT 144

typedef struct HeartlightSynapseConfig {
    double retention;
    double learning_rate;
    double weight_decay;
    double association_gain;
    double max_weight;
} HeartlightSynapseConfig;

// Stable C ABI for FFI-capable languages.
// All arrays use row-major layout. `state_out` may alias `state` or `stimulus`;
// `weights_out` may alias `weights`.
void heartlight_synapse_default_config(HeartlightSynapseConfig* out_config);
int heartlight_synapse_step(const double state[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                            const double stimulus[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                            const double weights[HEARTLIGHT_SYNAPSE_WEIGHT_COUNT],
                            const HeartlightSynapseConfig* config,
                            double state_out[HEARTLIGHT_SYNAPSE_DIMENSIONS],
                            double weights_out[HEARTLIGHT_SYNAPSE_WEIGHT_COUNT]);

double heartlight_synapse_hebbian_delta(double pre, double post, double weight,
                                         double learning_rate, double decay);
const char* heartlight_synapse_version(void);

#ifdef __cplusplus
}
#endif
