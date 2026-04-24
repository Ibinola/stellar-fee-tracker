/// A single simulated fee data point.
#[derive(Debug, Clone)]
pub struct FeePoint {
    pub timestamp: u64,
    pub fee: u64,
    pub ledger: u64,
    pub is_spike: bool,
}

/// Configuration for a single simulation scenario.
#[derive(Debug, Clone)]
pub struct FeeModelConfig {
    pub base_fee: u64,
    pub ledger_count: u64,
    pub spike_probability: f64,
    pub spike_multiplier: u64,
}

impl Default for FeeModelConfig {
    fn default() -> Self {
        Self {
            base_fee: 100,
            ledger_count: 100,
            spike_probability: 0.05,
            spike_multiplier: 10,
        }
    }
}

/// Models for simulating Stellar transaction fee behaviour.
pub struct FeeModel;

impl FeeModel {
    /// Generate fee points for a single config.
    pub fn generate(config: &FeeModelConfig) -> Vec<FeePoint> {
        let mut points = Vec::with_capacity(config.ledger_count as usize);
        let mut pseudo = 6364136223846793005u64;
        for i in 0..config.ledger_count {
            pseudo = pseudo.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let rand_f = (pseudo >> 33) as f64 / u32::MAX as f64;
            let is_spike = rand_f < config.spike_probability;
            let fee = if is_spike {
                config.base_fee * config.spike_multiplier
            } else {
                config.base_fee
            };
            points.push(FeePoint { timestamp: i * 5, fee, ledger: i + 1, is_spike });
        }
        points
    }

    /// Run multiple scenarios sequentially and return combined output.
    pub fn run_scenarios(configs: &[FeeModelConfig]) -> Vec<FeePoint> {
        let mut all = Vec::new();
        let mut ledger_offset = 0u64;
        let mut time_offset = 0u64;
        for config in configs {
            let mut points = Self::generate(config);
            for p in &mut points {
                p.ledger += ledger_offset;
                p.timestamp += time_offset;
            }
            let last = points.last().map(|p| (p.ledger, p.timestamp)).unwrap_or((0, 0));
            ledger_offset = last.0;
            time_offset = last.1 + 5;
            all.extend(points);
        }
        all
    }
}
