//! Generates synthetic network load profiles for simulation.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct NetworkLoadConfig {
    /// Minimum transactions per ledger.
    pub min_tx: u64,
    /// Maximum transactions per ledger.
    pub max_tx: u64,
    /// Optional RNG seed for reproducibility.
    pub seed: Option<u64>,
}

impl Default for NetworkLoadConfig {
    fn default() -> Self {
        Self {
            min_tx: 10,
            max_tx: 1000,
            seed: None,
        }
    }
}

/// Generates synthetic network load profiles for simulation.
pub struct NetworkLoad {
    config: NetworkLoadConfig,
    rng: SmallRng,
}

impl NetworkLoad {
    pub fn new(config: NetworkLoadConfig) -> Self {
        let rng = match config.seed {
            Some(s) => SmallRng::seed_from_u64(s),
            None => SmallRng::from_entropy(),
        };
        Self { config, rng }
    }

    /// Generate `count` transaction-count samples.
    pub fn generate(&mut self, count: usize) -> Vec<u64> {
        (0..count)
            .map(|_| self.rng.gen_range(self.config.min_tx..=self.config.max_tx))
            .collect()
pub struct NetworkLoad;

impl NetworkLoad {
    /// Returns a fee multiplier (1.0–3.0) based on hour of day (0–23).
    /// Peak hours (8–20) have higher fees simulating daytime congestion.
    pub fn diurnal_multiplier(hour: u8) -> f64 {
        // Simple sinusoidal: peak at hour 14 (2pm UTC), trough at hour 2 (2am UTC)
        let angle = std::f64::consts::PI * (hour as f64 - 2.0) / 12.0;
        1.0 + angle.sin().max(0.0) * 2.0
    }

    /// Apply diurnal multiplier to a base fee given the hour of day.
    pub fn diurnal_fee(base_fee: u64, hour: u8) -> u64 {
        (base_fee as f64 * Self::diurnal_multiplier(hour)).round() as u64
    }
}
