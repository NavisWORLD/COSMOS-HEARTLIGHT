//! HEARTLIGHT Synaptic Kernel v1.
//! Deterministic, bounded, dependency-free, network-free and persistence-free.

pub const DIMENSIONS: usize = 12;
pub const WEIGHT_COUNT: usize = DIMENSIONS * DIMENSIONS;

pub type State = [f64; DIMENSIONS];
pub type Weights = [f64; WEIGHT_COUNT];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub retention: f64,
    pub learning_rate: f64,
    pub weight_decay: f64,
    pub association_gain: f64,
    pub max_weight: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            retention: 0.85,
            learning_rate: 0.08,
            weight_decay: 0.002,
            association_gain: 0.25,
            max_weight: 1.0,
        }
    }
}

impl Config {
    pub fn normalized(self) -> Self {
        let mut max_weight = finite_or(self.max_weight, 1.0);
        if max_weight <= 0.0 { max_weight = 1.0; }
        max_weight = max_weight.min(100.0);
        Self {
            retention: clamp(self.retention, 0.0, 1.0, 0.5),
            learning_rate: clamp(self.learning_rate, 0.0, 1.0, 0.5),
            weight_decay: clamp(self.weight_decay, 0.0, 1.0, 0.5),
            association_gain: clamp(self.association_gain, 0.0, 1.0, 0.5),
            max_weight,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Result {
    pub state: State,
    pub weights: Weights,
}

fn finite_or(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn clamp(value: f64, lo: f64, hi: f64, fallback: f64) -> f64 {
    finite_or(value, fallback).clamp(lo, hi)
}

pub fn default_state() -> State { [5.0; DIMENSIONS] }
pub fn zero_weights() -> Weights { [0.0; WEIGHT_COUNT] }

pub fn step(raw_state: &State, raw_stimulus: &State, raw_weights: &Weights, raw_config: Config) -> Result {
    let cfg = raw_config.normalized();
    let mut state = [0.0; DIMENSIONS];
    let mut stimulus = [0.0; DIMENSIONS];
    let mut weights = [0.0; WEIGHT_COUNT];
    for i in 0..DIMENSIONS {
        state[i] = clamp(raw_state[i], 0.0, 10.0, 5.0);
        stimulus[i] = clamp(raw_stimulus[i], 0.0, 10.0, 5.0);
    }
    for i in 0..WEIGHT_COUNT {
        weights[i] = clamp(raw_weights[i], -cfg.max_weight, cfg.max_weight, 0.0);
    }

    let mut x = [0.0; DIMENSIONS];
    for j in 0..DIMENSIONS { x[j] = (stimulus[j] - 5.0) / 5.0; }

    let mut next = [0.0; DIMENSIONS];
    for i in 0..DIMENSIONS {
        let row = i * DIMENSIONS;
        let mut association = 0.0;
        for j in 0..DIMENSIONS { association += weights[row + j] * x[j]; }
        association /= DIMENSIONS as f64;
        next[i] = clamp(
            cfg.retention * state[i]
                + (1.0 - cfg.retention) * stimulus[i]
                + cfg.association_gain * 5.0 * association,
            0.0,
            10.0,
            5.0,
        );
    }

    let mut next_weights = [0.0; WEIGHT_COUNT];
    for i in 0..DIMENSIONS {
        let post = (next[i] - 5.0) / 5.0;
        let row = i * DIMENSIONS;
        for j in 0..DIMENSIONS {
            let idx = row + j;
            let delta = cfg.learning_rate * post * x[j] - cfg.weight_decay * weights[idx];
            next_weights[idx] = (weights[idx] + delta).clamp(-cfg.max_weight, cfg.max_weight);
        }
    }
    Result { state: next, weights: next_weights }
}

pub fn hebbian_delta(pre: f64, post: f64, weight: f64, learning_rate: f64, decay: f64) -> f64 {
    let p = clamp(pre, -1.0, 1.0, 0.0);
    let q = clamp(post, -1.0, 1.0, 0.0);
    let w = finite_or(weight, 0.0);
    let lr = clamp(learning_rate, 0.0, 1.0, 0.5);
    let d = clamp(decay, 0.0, 1.0, 0.5);
    lr * p * q - d * w
}

pub fn version() -> &'static str { "1.0.0" }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_weight_blend_matches_spec() {
        let state = default_state();
        let stimulus = [10.0,0.0,5.0,5.0,8.0,2.0,5.0,5.0,7.0,3.0,5.0,5.0];
        let out = step(&state, &stimulus, &zero_weights(), Config::default());
        let expected = [5.75,4.25,5.0,5.0,5.45,4.55,5.0,5.0,5.3,4.7,5.0,5.0];
        for i in 0..DIMENSIONS { assert!((out.state[i] - expected[i]).abs() < 1e-12); }
    }

    #[test]
    fn deterministic_and_bounded() {
        let mut state = default_state();
        state[0] = -100.0;
        state[11] = 100.0;
        let stimulus = [10.0; DIMENSIONS];
        let weights = [1.0; WEIGHT_COUNT];
        let cfg = Config { retention: 0.5, association_gain: 1.0, ..Config::default() };
        let a = step(&state, &stimulus, &weights, cfg);
        let b = step(&state, &stimulus, &weights, cfg);
        assert_eq!(a, b);
        assert!(a.state.iter().all(|v| *v >= 0.0 && *v <= 10.0));
        assert!(a.weights.iter().all(|v| *v >= -1.0 && *v <= 1.0));
    }
    #[test]
    fn weighted_conformance_vector() {
        let state: State = [1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,0.0,5.0];
        let stimulus: State = [9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0,0.0,10.0,5.0];
        let mut weights = [0.0; WEIGHT_COUNT];
        for i in 0..12_i32 { for j in 0..12_i32 { let m = ((i-j)%5+5)%5; weights[(i*12+j) as usize] = (m-2) as f64 / 10.0; } }
        let cfg=Config { retention:.8, learning_rate:.05, weight_decay:.01, association_gain:.3, max_weight:.75 };
        let out=step(&state,&stimulus,&weights,cfg);
        let expected: State=[2.575,3.1625,3.775,4.4125,5.075,5.575,6.1625,6.775,7.4125,8.075,1.975,4.9625];
        for i in 0..12 { assert!((out.state[i]-expected[i]).abs()<1e-12); }
        let sum: f64=out.weights.iter().sum(); assert!((sum-(-0.297)).abs()<1e-12);
    }

}
