//! HEARTLIGHT Synaptic Kernel v1: deterministic 12-channel numerical association.

pub const DIMENSIONS: usize = 12;
pub const WEIGHT_COUNT: usize = 144;
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
    fn default() -> Self { Self { retention:0.85, learning_rate:0.08, weight_decay:0.002, association_gain:0.25, max_weight:1.0 } }
}
impl Config {
    fn normalized(self) -> Self {
        let mut mw = if self.max_weight.is_finite() { self.max_weight } else { 1.0 };
        if mw <= 0.0 { mw = 1.0; }
        Self {
            retention: bounded(self.retention,0.0,1.0,0.5),
            learning_rate: bounded(self.learning_rate,0.0,1.0,0.5),
            weight_decay: bounded(self.weight_decay,0.0,1.0,0.5),
            association_gain: bounded(self.association_gain,0.0,1.0,0.5),
            max_weight: mw.min(100.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Result { pub state: State, pub weights: Weights }

fn bounded(v:f64, lo:f64, hi:f64, fallback:f64)->f64 { if v.is_finite(){v.clamp(lo,hi)} else {fallback} }
pub fn default_state()->State { [5.0; DIMENSIONS] }
pub fn zero_weights()->Weights { [0.0; WEIGHT_COUNT] }

pub fn step(raw_state:&State, raw_stimulus:&State, raw_weights:&Weights, raw_config:Config)->Result {
    let cfg=raw_config.normalized();
    let mut state=[0.0;DIMENSIONS]; let mut stimulus=[0.0;DIMENSIONS]; let mut weights=[0.0;WEIGHT_COUNT];
    for i in 0..DIMENSIONS { state[i]=bounded(raw_state[i],0.0,10.0,5.0); stimulus[i]=bounded(raw_stimulus[i],0.0,10.0,5.0); }
    for i in 0..WEIGHT_COUNT { weights[i]=bounded(raw_weights[i],-cfg.max_weight,cfg.max_weight,0.0); }
    let mut x=[0.0;DIMENSIONS]; for j in 0..DIMENSIONS { x[j]=(stimulus[j]-5.0)/5.0; }
    let mut next=[0.0;DIMENSIONS];
    for i in 0..DIMENSIONS {
        let row=i*DIMENSIONS; let mut association=0.0;
        for j in 0..DIMENSIONS { association += weights[row+j]*x[j]; }
        association /= DIMENSIONS as f64;
        next[i]=bounded(cfg.retention*state[i]+(1.0-cfg.retention)*stimulus[i]+cfg.association_gain*5.0*association,0.0,10.0,5.0);
    }
    let mut nw=[0.0;WEIGHT_COUNT];
    for i in 0..DIMENSIONS { let post=(next[i]-5.0)/5.0; let row=i*DIMENSIONS; for j in 0..DIMENSIONS { let k=row+j; let d=cfg.learning_rate*post*x[j]-cfg.weight_decay*weights[k]; nw[k]=(weights[k]+d).clamp(-cfg.max_weight,cfg.max_weight); } }
    Result { state:next, weights:nw }
}
pub fn version()->&'static str { "1.0.0" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blend_vector() {
        let out=step(&default_state(), &[10.0,0.0,5.0,5.0,8.0,2.0,5.0,5.0,7.0,3.0,5.0,5.0], &zero_weights(), Config::default());
        let expected=[5.75,4.25,5.0,5.0,5.45,4.55,5.0,5.0,5.3,4.7,5.0,5.0];
        for i in 0..12 { assert!((out.state[i]-expected[i]).abs()<1e-12); }
    }
    #[test]
    fn weighted_vector() {
        let state=[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,0.0,5.0];
        let stimulus=[9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0,0.0,10.0,5.0];
        let mut weights=[0.0;WEIGHT_COUNT];
        for i in 0..12_i32 { for j in 0..12_i32 { let m=((i-j)%5+5)%5; weights[(i*12+j) as usize]=(m-2) as f64/10.0; } }
        let cfg=Config{retention:0.8,learning_rate:0.05,weight_decay:0.01,association_gain:0.3,max_weight:0.75};
        let out=step(&state,&stimulus,&weights,cfg);
        let expected=[2.575,3.1625,3.775,4.4125,5.075,5.575,6.1625,6.775,7.4125,8.075,1.975,4.9625];
        for i in 0..12 { assert!((out.state[i]-expected[i]).abs()<1e-12); }
        assert!((out.weights.iter().sum::<f64>()+0.297).abs()<1e-12);
    }
}
