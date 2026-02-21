use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::error::AppError;
use crate::services::horizon::HorizonClient;

/// Shared state type for the fees route.
pub type FeesState = Arc<HorizonClient>;

#[derive(Serialize)]
pub struct PercentileFees {
    pub p10: String,
    pub p25: String,
    pub p50: String,
    pub p75: String,
    pub p90: String,
    pub p95: String,
}

#[derive(Serialize)]
pub struct CurrentFeeResponse {
    pub base_fee: String,
    pub min_fee: String,
    pub max_fee: String,
    pub avg_fee: String,
    pub percentiles: PercentileFees,
}

pub async fn current_fees(
    State(client): State<FeesState>,
) -> Result<Json<CurrentFeeResponse>, AppError> {
    let stats = client.fetch_fee_stats().await?;

    Ok(Json(CurrentFeeResponse {
        base_fee: stats.last_ledger_base_fee,
        min_fee: stats.fee_charged.min,
        max_fee: stats.fee_charged.max,
        avg_fee: stats.fee_charged.avg,
        percentiles: PercentileFees {
            p10: stats.fee_charged.p10,
            p25: stats.fee_charged.p25,
            p50: stats.fee_charged.p50,
            p75: stats.fee_charged.p75,
            p90: stats.fee_charged.p90,
            p95: stats.fee_charged.p95,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_fee_response_serialises_with_percentiles() {
        let response = CurrentFeeResponse {
            base_fee: "100".into(),
            min_fee: "100".into(),
            max_fee: "5000".into(),
            avg_fee: "213".into(),
            percentiles: PercentileFees {
                p10: "100".into(),
                p25: "100".into(),
                p50: "150".into(),
                p75: "300".into(),
                p90: "500".into(),
                p95: "800".into(),
            },
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["base_fee"], "100");
        assert_eq!(json["percentiles"]["p10"], "100");
        assert_eq!(json["percentiles"]["p50"], "150");
        assert_eq!(json["percentiles"]["p95"], "800");
    }

    #[test]
    fn percentile_fees_has_all_six_fields() {
        let p = PercentileFees {
            p10: "100".into(),
            p25: "100".into(),
            p50: "150".into(),
            p75: "300".into(),
            p90: "500".into(),
            p95: "800".into(),
        };
        let json = serde_json::to_value(&p).unwrap();
        for field in &["p10", "p25", "p50", "p75", "p90", "p95"] {
            assert!(json.get(field).is_some(), "missing field: {}", field);
            assert!(!json[field].as_str().unwrap().is_empty());
        }
    }
}