/// Maintains a rolling window of fee observations.
pub struct RollingWindow;

impl RollingWindow {
    /// Simple moving average over `window` elements.
    pub fn sma(fees: &[f64], window: usize) -> Vec<f64> {
        if window == 0 || fees.len() < window {
            return vec![];
        }
        fees.windows(window)
            .map(|w| w.iter().sum::<f64>() / window as f64)
            .collect()
    }

    /// Exponential moving average with configurable smoothing factor `alpha` (0 < alpha <= 1).
    pub fn ema(fees: &[f64], alpha: f64) -> Vec<f64> {
        if fees.is_empty() {
            return vec![];
        }
        let mut result = Vec::with_capacity(fees.len());
        let mut prev = fees[0];
        result.push(prev);
        for &fee in &fees[1..] {
            prev = alpha * fee + (1.0 - alpha) * prev;
            result.push(prev);
        }
        result
    }

    /// Weighted moving average — most recent values weighted highest (linear weights).
    pub fn wma(fees: &[f64], window: usize) -> Vec<f64> {
        if window == 0 || fees.len() < window {
            return vec![];
        }
        let denom = (window * (window + 1) / 2) as f64;
        fees.windows(window)
            .map(|w| {
                w.iter()
                    .enumerate()
                    .map(|(i, &v)| v * (i + 1) as f64)
                    .sum::<f64>()
                    / denom
            })
            .collect()
    }
}
