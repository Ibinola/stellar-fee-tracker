/// Mock implementation of the Horizon API server for use in tests.
pub struct HorizonMock {
    /// Name of the currently active scenario.
    pub scenario: String,
    /// Optional simulated response delay in milliseconds.
    pub delay_ms: Option<u64>,
    /// Probability [0.0, 1.0] of returning a 500/503 error response.
    pub error_rate: f64,
}

impl HorizonMock {
    pub fn new(scenario: impl Into<String>) -> Self {
        Self { scenario: scenario.into(), delay_ms: None, error_rate: 0.0 }
    }

    /// Sets the simulated network latency delay.
    pub fn with_delay_ms(mut self, ms: u64) -> Self {
        self.delay_ms = Some(ms);
        self
    }

    /// Applies the configured delay, if any. Call before serving a response.
    pub fn apply_delay(&self) {
        if let Some(ms) = self.delay_ms {
            std::thread::sleep(std::time::Duration::from_millis(ms));
        }
    }

    /// Sets the error injection rate (0.0 = never, 1.0 = always).
    pub fn with_error_rate(mut self, rate: f64) -> Self {
        self.error_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Returns true if this request should be failed based on the configured error rate.
    pub fn should_inject_error(&self) -> bool {
        self.error_rate > 0.0 && rand_f64() < self.error_rate
    }

    /// Switches to the next scenario from the rotator and updates the active scenario.
    pub fn rotate(&mut self, rotator: &mut crate::harness::scenarios::ScenarioRotator) {
        if let Some(next) = rotator.next() {
            self.scenario = next.to_string();
        }
    }

    /// Logs a request to stdout with timestamp, method, path, and active scenario name.
    pub fn log_request(&self, method: &str, path: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        println!("[{}] {} {} scenario={}", now, method, path, self.scenario);
    }

    /// Returns the JSON body for `GET /health`.
    pub fn health_payload(&self) -> String {
        format!(r#"{{"status":"ok","scenario":"{}"}}"#, self.scenario)
    }

    /// Loads and returns the scenario JSON to be served at `GET /fee_stats`.
    pub fn fee_stats_payload(&self) -> std::io::Result<String> {
        crate::harness::scenarios::load_from_file(
            std::path::Path::new(&format!("src/harness/scenarios/{}.json", self.scenario)),
        )
    }
}

/// Minimal pseudo-random float in [0.0, 1.0) using system time as entropy.
fn rand_f64() -> f64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 1_000_000) as f64 / 1_000_000.0
}
