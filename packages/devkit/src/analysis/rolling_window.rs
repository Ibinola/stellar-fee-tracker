/// Maintains a rolling window of fee observations with stateful incremental updates.
pub struct RollingWindow {
    window: usize,
    buf: std::collections::VecDeque<f64>,
}

impl RollingWindow {
    /// Create a new rolling window of the given size. Panics if `window == 0`.
    pub fn new(window: usize) -> Self {
        assert!(window > 0, "window size must be > 0");
        Self {
            window,
            buf: std::collections::VecDeque::with_capacity(window),
        }
    }

    /// Push a new fee value and return the current SMA if the window is full.
    pub fn push(&mut self, fee: f64) -> Option<f64> {
        if self.buf.len() == self.window {
            self.buf.pop_front();
        }
        self.buf.push_back(fee);
        if self.buf.len() == self.window {
            Some(self.buf.iter().sum::<f64>() / self.window as f64)
        } else {
            None
        }
    }

    /// Simple moving average over a complete slice with the given window size.
    /// Returns one value per position once the window is full. O(n) implementation.
    pub fn sma(fees: &[f64], window: usize) -> Vec<f64> {
        if window == 0 || fees.len() < window {
            return vec![];
        }
        let mut result = Vec::with_capacity(fees.len() - window + 1);
        let mut sum: f64 = fees[..window].iter().sum();
        result.push(sum / window as f64);
        for i in window..fees.len() {
            sum += fees[i] - fees[i - window];
            result.push(sum / window as f64);
        }
        result
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
