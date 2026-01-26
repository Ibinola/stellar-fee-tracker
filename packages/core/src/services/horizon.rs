use serde::Deserialize;
use crate::error::AppError;

#[derive(Clone)]
pub struct HorizonClient {
    base_url: String,
}

impl HorizonClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[derive(Debug, Deserialize)]
pub struct HorizonTransaction {
    pub hash: String,
    pub successful: bool,
    pub fee_charged: String,
}

#[derive(Debug, Deserialize)]
pub struct HorizonOperation {
    #[serde(rename = "type")]
    pub op_type: String,

    pub from: Option<String>,
    pub to: Option<String>,

    pub asset_type: Option<String>,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,

    pub amount: Option<String>,
}

impl HorizonClient {
    /// Fetch the latest transaction from Horizon.
    ///
    /// NOTE:
    /// - Networking is added in a later issue
    /// - This stub prevents accidental Horizon usage early
    pub fn fetch_latest_transaction(
        &self,
    ) -> Result<HorizonTransaction, AppError> {
        Err(AppError::Network(
            "Horizon client not implemented yet".into(),
        ))
    }

    /// Fetch operations for a given transaction hash.
    pub fn fetch_operations(
        &self,
        _tx_hash: &str,
    ) -> Result<Vec<HorizonOperation>, AppError> {
        Err(AppError::Network(
            "Horizon client not implemented yet".into(),
        ))
    }
}
