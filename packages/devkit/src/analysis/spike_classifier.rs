use crate::analysis::percentile::Percentile;

/// Severity level of a fee spike relative to baseline.
#[derive(Debug, Clone, PartialEq)]
pub enum SpikeSeverity {
    /// 2–5× baseline
    Low,
    /// 5–10× baseline
    Medium,
    /// 10–50× baseline
    High,
    /// >50× baseline
    Critical,
}

/// A detected spike with its severity and duration.
#[derive(Debug, Clone, PartialEq)]
pub struct SpikeEvent {
    pub severity: SpikeSeverity,
    /// Number of consecutive ledgers the spike persisted.
    pub duration_ledgers: usize,
}

/// Classifies fee spikes in a time series.
pub struct SpikeClassifier;

impl SpikeClassifier {
    /// Returns indices of fees that fall outside 1.5 × IQR from Q1/Q3.
    /// `fees` need not be sorted; indices refer to the original slice.
    pub fn iqr_outliers(fees: &[u64]) -> Vec<usize> {
        if fees.len() < 4 {
            return vec![];
        }
        let mut sorted = fees.to_vec();
        sorted.sort_unstable();
        let q1 = Percentile::nearest_rank(&sorted, 25) as f64;
        let q3 = Percentile::nearest_rank(&sorted, 75) as f64;
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        fees.iter()
            .enumerate()
            .filter(|(_, &v)| (v as f64) < lower || (v as f64) > upper)
            .map(|(i, _)| i)
            .collect()
    }

    /// Classify a single fee against a baseline.
    /// Returns `None` if the fee is below the 2× spike threshold.
    pub fn classify(fee: u64, baseline: u64) -> Option<SpikeSeverity> {
        if baseline == 0 {
            return None;
        }
        let ratio = fee as f64 / baseline as f64;
        match ratio {
            r if r > 50.0 => Some(SpikeSeverity::Critical),
            r if r >= 10.0 => Some(SpikeSeverity::High),
            r if r >= 5.0 => Some(SpikeSeverity::Medium),
            r if r >= 2.0 => Some(SpikeSeverity::Low),
            _ => None,
        }
    }

    /// Detect all spike events in a fee sequence given a fixed baseline.
    /// Consecutive ledgers above the 2× threshold are grouped into one event.
    pub fn detect(fees: &[u64], baseline: u64) -> Vec<SpikeEvent> {
        let mut events = Vec::new();
        let mut i = 0;
        while i < fees.len() {
            if let Some(severity) = Self::classify(fees[i], baseline) {
                let start = i;
                // Advance while still a spike (any severity)
                while i < fees.len() && Self::classify(fees[i], baseline).is_some() {
                    i += 1;
                }
                // Severity of the event = max severity seen in the run
                let severity = fees[start..i]
                    .iter()
                    .filter_map(|&f| Self::classify(f, baseline))
                    .max_by_key(|s| match s {
                        SpikeSeverity::Low => 1,
                        SpikeSeverity::Medium => 2,
                        SpikeSeverity::High => 3,
                        SpikeSeverity::Critical => 4,
                    })
                    .unwrap_or(severity);
                events.push(SpikeEvent {
                    severity,
                    duration_ledgers: i - start,
                });
            } else {
                i += 1;
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iqr_outliers_detects_spike() {
        let fees = [100u64, 102, 98, 101, 99, 500, 103, 97];
        let outliers = SpikeClassifier::iqr_outliers(&fees);
        assert!(outliers.contains(&5), "500 should be flagged");
    }

    #[test]
    fn iqr_outliers_no_outliers() {
        let fees = [10u64, 11, 12, 13, 14, 15];
        assert!(SpikeClassifier::iqr_outliers(&fees).is_empty());
    }

    #[test]
    fn iqr_outliers_too_small() {
        assert!(SpikeClassifier::iqr_outliers(&[1, 2, 3]).is_empty());
    }
}
