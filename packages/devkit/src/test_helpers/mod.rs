use crate::simulation::fee_model::{FeeModel, FeeModelConfig};
use crate::types::FeeRecord;

/// Returns a deterministic fee sequence of `count` records seeded by `seed`.
pub fn make_fee_sequence(count: usize, seed: u64) -> Vec<FeeRecord> {
    let config = FeeModelConfig {
        seed: Some(seed),
        ..Default::default()
    };
    FeeModel::new(config).generate(count, 0)
}

/// Returns a fee sequence where every record is flagged as a spike.
pub fn make_spike_sequence(count: usize) -> Vec<FeeRecord> {
    let config = FeeModelConfig {
        spike_probability: 1.0,
        seed: Some(0),
        ..Default::default()
    };
    FeeModel::new(config).generate(count, 0)
}

/// Returns a fee sequence with no spikes (baseline load only).
pub fn make_baseline_sequence(count: usize) -> Vec<FeeRecord> {
    let config = FeeModelConfig {
        spike_probability: 0.0,
        seed: Some(1),
        ..Default::default()
    };
    FeeModel::new(config).generate(count, 0)
}
