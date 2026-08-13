#pragma once

#include <array>
#include <cstddef>

namespace heartlight::synapse {

constexpr std::size_t kDimensions = 12;
constexpr std::size_t kWeightCount = kDimensions * kDimensions;

using State = std::array<double, kDimensions>;
using Weights = std::array<double, kWeightCount>;

struct Config {
    double retention{0.85};
    double learning_rate{0.08};
    double weight_decay{0.002};
    double association_gain{0.25};
    double max_weight{1.0};
};

struct Result {
    State state{};
    Weights weights{};
};

State default_state();
Weights zero_weights();
Config normalize_config(const Config& config);
Result step(const State& state, const State& stimulus, const Weights& weights, const Config& config = {});
double hebbian_delta(double pre, double post, double weight,
                     double learning_rate = 0.08, double decay = 0.002);
const char* version();

}  // namespace heartlight::synapse
