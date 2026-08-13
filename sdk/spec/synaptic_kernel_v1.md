# HEARTLIGHT Synaptic Kernel v1

HEARTLIGHT Synapse is a deterministic 12-channel numerical association kernel shared across language SDKs.

It is an engineering abstraction. It is **not** a model of a human nervous system, a consciousness detector, a diagnostic system, an emotion recognizer, or a behavior-risk predictor.

## State

A state is 12 finite numbers bounded to `[0, 10]`.

A stimulus is another 12-value vector bounded to `[0, 10]`.

Weights are a row-major `12 x 12` matrix bounded to `[-max_weight, +max_weight]`.

Non-finite state/stimulus values are replaced with `5.0`. Non-finite weights are replaced with `0.0`.

## Configuration

Defaults:

- `retention = 0.85`
- `learning_rate = 0.08`
- `weight_decay = 0.002`
- `association_gain = 0.25`
- `max_weight = 1.0`

Configuration values are sanitized:

- retention, learning rate, weight decay, association gain -> `[0,1]`
- max weight -> `(0, 100]`, defaulting to `1.0` when invalid

## Step

For channel `j`:

`x_j = (stimulus_j - 5) / 5`

For output channel `i`:

`association_i = (1/12) * sum_j(weights[i,j] * x_j)`

`next_i = clamp10(retention * state_i + (1-retention) * stimulus_i + association_gain * 5 * association_i)`

Then:

`post_i = (next_i - 5) / 5`

and each weight updates as:

`delta_ij = learning_rate * post_i * x_j - weight_decay * weight_ij`

`next_weight_ij = clamp(weight_ij + delta_ij, -max_weight, +max_weight)`

## Properties

- deterministic
- no random number generator
- no network calls
- no hidden model weights
- no file/database persistence
- bounded numeric state
- same row-major matrix layout in every implementation
- reference conformance vectors live in `synaptic_test_vectors_v1.json`

## Persistence boundary

The kernel itself never writes state or weights to disk. A host application may explicitly persist values for an adult/research use case, but HEARTLIGHT's child-facing reference application does not create permanent learner synaptic profiles.
