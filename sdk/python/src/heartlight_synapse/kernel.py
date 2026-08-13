"""HEARTLIGHT Synaptic Kernel v1: deterministic 12-channel numerical association."""
from __future__ import annotations
from dataclasses import dataclass
from math import isfinite
from typing import Sequence

DIMENSIONS = 12
WEIGHT_COUNT = 144

def _finite(value, fallback):
    try: value = float(value)
    except (TypeError, ValueError): return fallback
    return value if isfinite(value) else fallback

def _clamp(value, lo, hi, fallback):
    value = _finite(value, fallback)
    return lo if value < lo else hi if value > hi else value

@dataclass(frozen=True)
class SynapticConfig:
    retention: float = 0.85
    learning_rate: float = 0.08
    weight_decay: float = 0.002
    association_gain: float = 0.25
    max_weight: float = 1.0
    def normalized(self):
        mw = _finite(self.max_weight, 1.0)
        if mw <= 0: mw = 1.0
        return SynapticConfig(_clamp(self.retention,0,1,.5), _clamp(self.learning_rate,0,1,.5), _clamp(self.weight_decay,0,1,.5), _clamp(self.association_gain,0,1,.5), min(mw,100.0))

@dataclass(frozen=True)
class SynapticResult:
    state: tuple[float, ...]
    weights: tuple[float, ...]

def default_state(): return (5.0,) * DIMENSIONS
def zero_weights(): return (0.0,) * WEIGHT_COUNT

def step(state: Sequence[float], stimulus: Sequence[float], weights: Sequence[float] | None = None, config: SynapticConfig = SynapticConfig()) -> SynapticResult:
    if len(state) != DIMENSIONS or len(stimulus) != DIMENSIONS: raise ValueError("state and stimulus must each contain 12 values")
    if weights is None: weights = zero_weights()
    if len(weights) != WEIGHT_COUNT: raise ValueError("weights must contain 144 values")
    cfg = config.normalized()
    s = [_clamp(v,0,10,5) for v in state]
    u = [_clamp(v,0,10,5) for v in stimulus]
    w = [_clamp(v,-cfg.max_weight,cfg.max_weight,0) for v in weights]
    x = [(v-5.0)/5.0 for v in u]
    nxt = [0.0] * DIMENSIONS
    for i in range(DIMENSIONS):
        row = i * DIMENSIONS
        association = sum(w[row+j] * x[j] for j in range(DIMENSIONS)) / DIMENSIONS
        nxt[i] = _clamp(cfg.retention*s[i] + (1-cfg.retention)*u[i] + cfg.association_gain*5.0*association, 0, 10, 5)
    next_weights = [0.0] * WEIGHT_COUNT
    for i in range(DIMENSIONS):
        post = (nxt[i]-5.0)/5.0
        row = i * DIMENSIONS
        for j in range(DIMENSIONS):
            idx = row+j
            delta = cfg.learning_rate*post*x[j] - cfg.weight_decay*w[idx]
            next_weights[idx] = _clamp(w[idx]+delta, -cfg.max_weight, cfg.max_weight, 0)
    return SynapticResult(tuple(nxt), tuple(next_weights))

def association_delta(pre: float, post: float, weight: float, learning_rate: float = .08, decay: float = .002) -> float:
    return _clamp(learning_rate,0,1,.5)*_clamp(pre,-1,1,0)*_clamp(post,-1,1,0) - _clamp(decay,0,1,.5)*_finite(weight,0)
