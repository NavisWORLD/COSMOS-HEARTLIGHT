#include "heartlight/synapse.hpp"

#include <cassert>
#include <cmath>

int main() {
    using namespace heartlight::synapse;
    {
        State state = default_state();
        State stimulus{10,0,5,5,8,2,5,5,7,3,5,5};
        auto result = step(state, stimulus, zero_weights());
        const State expected{5.75,4.25,5,5,5.45,4.55,5,5,5.3,4.7,5,5};
        for (std::size_t i = 0; i < kDimensions; ++i) assert(std::abs(result.state[i] - expected[i]) < 1e-12);
    }
    {
        State state{1,2,3,4,5,6,7,8,9,10,0,5};
        State stimulus{9,8,7,6,5,4,3,2,1,0,10,5};
        Weights weights{};
        for (int i=0;i<12;++i) for (int j=0;j<12;++j) {
            const int mod = ((i-j)%5 + 5) % 5;
            weights[static_cast<std::size_t>(i*12+j)] = (mod - 2) / 10.0;
        }
        Config cfg{.8,.05,.01,.3,.75};
        auto result = step(state, stimulus, weights, cfg);
        const State expected{2.575,3.1625,3.775,4.4125,5.075,5.575,6.1625,6.775,7.4125,8.075,1.975,4.9625};
        for (std::size_t i=0;i<12;++i) assert(std::abs(result.state[i]-expected[i])<1e-12);
        double sum=0; for(double w:result.weights) sum+=w;
        assert(std::abs(sum - (-0.297)) < 1e-12);
    }
    return 0;
}
