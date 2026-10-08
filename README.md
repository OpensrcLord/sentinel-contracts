# sentinel-contract

Soroban smart contract for Stellar Sentinel — an on-chain risk registry that
authorized off-chain AI agents can write anomaly flags to.

## Status
Early scaffold. `initialize`, `authorize_agent`, `is_agent`, `flag_anomaly`,
and `get_threshold` are implemented. Flag events are emitted only when an
authorized agent submits a score at or above the configured threshold. See
open issues for what's missing (role separation, flag history, upgrade safety,
and broader integration coverage).

## Build
```
cargo build --target wasm32-unknown-unknown --release
```

## Test
```
cargo test
```
