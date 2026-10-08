# Contributing to sentinel-contract

## Setup
```
cargo build --target wasm32-unknown-unknown --release
cargo test
cargo clippy --all-targets
```

## Before opening a PR
- Run the commands above and make sure they pass.
- Keep the PR scoped to one issue; reference it with `Closes #N`.
- Update README.md if you changed public behavior.

## Related repos
- sentinel-backend — scores addresses and calls this contract
- sentinel-frontend — dashboard reading data derived from this contract
