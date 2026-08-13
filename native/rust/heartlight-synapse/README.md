# heartlight-synapse

Dependency-free Rust implementation of HEARTLIGHT Synaptic Kernel v1.

```rust
use heartlight_synapse::{default_state, step, zero_weights, Config};
let result = step(&default_state(), &stimulus, &zero_weights(), Config::default());
```

The crate performs no network I/O and no persistence.
