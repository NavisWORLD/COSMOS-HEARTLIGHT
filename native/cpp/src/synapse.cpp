#include "heartlight/synapse.hpp"

#include <algorithm>
#include <cmath>

namespace heartlight::synapse {
namespace {

double finite_or(double value, double fallback) {
    return std::isfinite(value) ? value : fallback;
}

double clamp(double value, double lo, double hi, double fallback) {
    value = finite_or(value, fallback);
    return std::clamp(value, lo, hi);
}

double clamp10(double value) {
    return clamp(value, 0.0, 10.0, 5.0);
}

}  // namespace

State default_state() {
    State out{};
    out.fill(5.0);
    return out;
}

Weights zero_weights() {
    Weights out{};
    out.fill(0.0);
    return out;
}

Config normalize_config(const Config& c) {
    Config out;
    out.retention = clamp(c.retention, 0.0, 1.0, 0.5);
    out.learning_rate = clamp(c.learning_rate, 0.0, 1.0, 0.5);
    out.weight_decay = clamp(c.weight_decay, 0.0, 1.0, 0.5);
    out.association_gain = clamp(c.association_gain, 0.0, 1.0, 0.5);
    double mw = finite_or(c.max_weight, 1.0);
    if (mw <= 0.0) mw = 1.0;
    out.max_weight = std::min(mw, 100.0);
    return out;
}

Result step(const State& raw_state, const State& raw_stimulus, const Weights& raw_weights,
            const Config& raw_config) {
    const Config cfg = normalize_config(raw_config);
    State state{};
    State stimulus{};
    Weights weights{};
    for (std::size_t i = 0; i < kDimensions; ++i) {
        state[i] = clamp10(raw_state[i]);
        stimulus[i] = clamp10(raw_stimulus[i]);
    }
    for (std::size_t i = 0; i < kWeightCount; ++i) {
        weights[i] = clamp(raw_weights[i], -cfg.max_weight, cfg.max_weight, 0.0);
    }

    std::array<double, kDimensions> x{};
    for (std::size_t j = 0; j < kDimensions; ++j) x[j] = (stimulus[j] - 5.0) / 5.0;

    Result out{};
    for (std::size_t i = 0; i < kDimensions; ++i) {
        const std::size_t row = i * kDimensions;
        double association = 0.0;
        for (std::size_t j = 0; j < kDimensions; ++j) association += weights[row + j] * x[j];
        association /= static_cast<double>(kDimensions);
        out.state[i] = clamp10(cfg.retention * state[i]
                               + (1.0 - cfg.retention) * stimulus[i]
                               + cfg.association_gain * 5.0 * association);
    }

    for (std::size_t i = 0; i < kDimensions; ++i) {
        const double post = (out.state[i] - 5.0) / 5.0;
        const std::size_t row = i * kDimensions;
        for (std::size_t j = 0; j < kDimensions; ++j) {
            const std::size_t idx = row + j;
            const double delta = cfg.learning_rate * post * x[j] - cfg.weight_decay * weights[idx];
            out.weights[idx] = std::clamp(weights[idx] + delta, -cfg.max_weight, cfg.max_weight);
        }
    }
    return out;
}

double hebbian_delta(double pre, double post, double weight, double learning_rate, double decay) {
    const double p = clamp(pre, -1.0, 1.0, 0.0);
    const double q = clamp(post, -1.0, 1.0, 0.0);
    const double w = finite_or(weight, 0.0);
    const double lr = clamp(learning_rate, 0.0, 1.0, 0.5);
    const double d = clamp(decay, 0.0, 1.0, 0.5);
    return lr * p * q - d * w;
}

const char* version() { return "1.0.0"; }

}  // namespace heartlight::synapse
