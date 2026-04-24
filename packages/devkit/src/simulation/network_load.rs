/// Generates synthetic network load profiles for simulation.
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
