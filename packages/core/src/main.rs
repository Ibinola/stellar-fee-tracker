mod cli;
mod config;
mod error;
mod insights;
mod logging;
mod services;
mod api;

use clap::Parser;
use dotenvy::dotenv;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::cli::Cli;
use crate::config::Config;
use crate::error::AppError;
use crate::logging::init_logging;
use crate::services::horizon::HorizonClient;
use crate::insights::{FeeInsightsEngine, HorizonFeeDataProvider, InsightsConfig, FeeDataProvider};
use crate::api::insights::{create_insights_router, InsightsState};

#[tokio::main]
async fn main() {
    // Load .env file (if present)
    dotenv().ok();

    // Initialize structured logging
    init_logging();

    // Parse CLI flags
    let cli = Cli::parse();

    // Build configuration (CLI overrides env)
    let config = Config::from_sources(&cli)
        .map_err(AppError::Config)
        .unwrap_or_else(|err| {
            tracing::error!("{}", err);
            std::process::exit(1);
        });

    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize Horizon client
    let horizon_client = HorizonClient::new(config.horizon_url.clone());
    tracing::info!(
        "Horizon client initialized with base URL: {}",
        horizon_client.base_url()
    );

    // Initialize fee insights engine
    let insights_config = InsightsConfig::default();
    let insights_engine = FeeInsightsEngine::new(insights_config);
    let insights_state: InsightsState = Arc::new(RwLock::new(insights_engine));
    tracing::info!("Fee insights engine initialized");

    // Initialize fee data provider
    let fee_data_provider = HorizonFeeDataProvider::new(horizon_client.clone());
    tracing::info!("Fee data provider initialized");

    // Create API router
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", create_insights_router(insights_state.clone()))
        .layer(CorsLayer::permissive());

    // Start the web server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("API server starting on http://0.0.0.0:3000");
    
    // Clone state for the polling task
    let polling_state = insights_state.clone();
    let poll_interval_seconds = config.poll_interval_seconds;
    
    // Spawn the polling task
    let polling_task = tokio::spawn(async move {
        run_polling_loop(fee_data_provider, polling_state, poll_interval_seconds).await;
    });
    
    // Spawn the web server task
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    
    // Wait for either task to complete (they should run indefinitely)
    tokio::select! {
        _ = polling_task => {
            tracing::error!("Polling task ended unexpectedly");
        }
        _ = server_task => {
            tracing::error!("Server task ended unexpectedly");
        }
    }
}

/// Run the fee insights polling loop
async fn run_polling_loop(
    fee_data_provider: HorizonFeeDataProvider,
    insights_state: InsightsState,
    poll_interval_seconds: u64,
) {
    let poll_interval = Duration::from_secs(poll_interval_seconds);
    let mut interval_timer = interval(poll_interval);
    
    tracing::info!("Starting fee insights polling loop with interval: {:?}", poll_interval);

    loop {
        interval_timer.tick().await;
        
        match run_insights_cycle(&fee_data_provider, &insights_state).await {
            Ok(()) => {
                tracing::debug!("Insights cycle completed successfully");
            }
            Err(err) => {
                tracing::error!("Insights cycle failed: {}", err);
                // Continue polling even if one cycle fails
            }
        }
    }
}

/// Run a single insights processing cycle
async fn run_insights_cycle(
    provider: &HorizonFeeDataProvider,
    insights_state: &InsightsState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Fetch latest fee data
    let fee_data = provider.fetch_latest_fees().await
        .map_err(|e| format!("Failed to fetch fee data: {}", e))?;
    
    if fee_data.is_empty() {
        tracing::warn!("No fee data received from provider");
        return Ok(());
    }
    
    tracing::debug!("Fetched {} fee data points", fee_data.len());
    
    // Process fee data through insights engine
    let mut engine = insights_state.write().await;
    let insights_update = engine.process_fee_data(&fee_data).await
        .map_err(|e| format!("Failed to process fee data: {}", e))?;
    
    // Log insights summary
    log_insights_summary(&insights_update);
    
    Ok(())
}

/// Log a summary of the insights update
fn log_insights_summary(update: &crate::insights::InsightsUpdate) {
    let insights = &update.insights;
    
    tracing::info!(
        "Insights updated: processed {} data points in {}ms",
        update.data_points_processed,
        update.processing_time.num_milliseconds()
    );
    
    // Log rolling averages
    tracing::info!(
        "Rolling averages - Short: {:.2}, Medium: {:.2}, Long: {:.2}",
        insights.rolling_averages.short_term.value,
        insights.rolling_averages.medium_term.value,
        insights.rolling_averages.long_term.value
    );
    
    // Log extremes
    tracing::info!(
        "Fee extremes - Min: {}, Max: {}",
        insights.extremes.current_min.value,
        insights.extremes.current_max.value
    );
    
    // Log congestion status
    tracing::info!(
        "Congestion status: {:?} (strength: {:?}, {} recent spikes)",
        insights.congestion_trends.current_trend,
        insights.congestion_trends.trend_strength,
        insights.congestion_trends.recent_spikes.len()
    );
    
    // Log data quality
    tracing::info!(
        "Data quality - Completeness: {:.1}%, Freshness: {}s, Has gaps: {}",
        insights.data_quality.completeness * 100.0,
        insights.data_quality.freshness.num_seconds(),
        insights.data_quality.has_gaps
    );
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}