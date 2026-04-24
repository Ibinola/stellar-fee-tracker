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

/// Configuration for simulated network load (#120).
#[derive(Debug, Clone)]
pub struct NetworkLoadConfig {
    /// Maximum transactions the network can process per ledger.
    pub ledger_capacity: u32,
    /// Number of transactions submitted per ledger.
    pub tx_per_ledger: u32,
    /// Time between ledger closes in milliseconds.
    pub ledger_interval_ms: u64,
}

/// A single simulated ledger produced by the throughput simulator (#121).
#[derive(Debug, Clone)]
pub struct SimulatedLedger {
    pub ledger_seq: u64,
    pub tx_count: u32,
    /// Capacity pressure in [0.0, 1.0]: tx_per_ledger / ledger_capacity (#122).
    pub pressure: f64,
}

impl NetworkLoadConfig {
    /// Returns the capacity pressure ratio (#122).
    pub fn pressure(&self) -> f64 {
        self.tx_per_ledger as f64 / self.ledger_capacity as f64
    }
}

impl NetworkLoad {
    /// Simulate `ledger_count` ledger closes and return the resulting ledgers (#121).
    pub fn simulate(config: &NetworkLoadConfig, ledger_count: u64) -> Vec<SimulatedLedger> {
        (0..ledger_count)
            .map(|seq| SimulatedLedger {
                ledger_seq: seq + 1,
                tx_count: config.tx_per_ledger,
                pressure: config.pressure(),
            })
            .collect()
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
