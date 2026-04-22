# stellar-devkit

Developer toolkit for the Stellar Fee Tracker. Provides utilities for testing, mocking, and simulating Stellar network behaviour without hitting live infrastructure.

## Scope

`stellar-devkit` is a standalone testing and simulation package. It must not import from `stellar-core` or any live-network crate. All functionality is self-contained and intended for use in `[dev-dependencies]` only.

## Boundary Rules

- No imports from `packages/core`
- No live Horizon API calls
- No database connections
- All external I/O must be injectable or mockable

## Modules

| Module | Description |
|---|---|
| `harness` | Mock Horizon server and pre-built fee scenario fixtures |
| `harness::scenarios` | JSON scenario files and runtime loader |
| `simulation` | Fee models, network-load generators, congestion predictors |
| `analysis` | Percentile stats, spike classification, rolling window |
| `cli` | Replay, export, and benchmark CLI stubs |
| `types` | Shared types: `FeeRecord`, `Scenario`, `SimResult` |
| `error` | `DevkitError` unified error enum |

## Running

```bash
# Run all devkit tests
cargo test -p stellar-devkit

# Run a specific test file
cargo test -p stellar-devkit --test harness_congested
```

## Adding to Your Crate

```toml
[dev-dependencies]
stellar-devkit = { path = "../devkit" }
```
