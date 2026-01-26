mod config;
mod services;
mod error;
mod logging;

use crate::services::horizon::HorizonClient;
use dotenvy::dotenv;
use crate::config::Config;
use crate::error::AppError;
use crate::logging::init_logging;

fn main() {
    dotenv().ok();
    init_logging();

    let config = Config::from_env()
        .map_err(AppError::Config)
        .unwrap_or_else(|err| {
            tracing::error!("{}", err);
            std::process::exit(1);
        });

    println!("🚀 Stellar Fee Tracker starting up");
    println!("🔧 Loaded config: {:#?}", config);
   

    let horizon_client = HorizonClient::new(config.horizon_url.clone());
    tracing::info!(
        "Horizon client initialized with base URL: {}",
        horizon_client.base_url()
    );
}


    tracing::info!("Service started with config: {:?}", config);
}
