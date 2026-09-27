use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;
use trading_bot::client::client::Client;
use trading_bot::config::config::Config;
use trading_bot::market_data::refresh_all_stocks::run_refresh_all_stocks_task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config from env
    let config =
        Config::load().expect("Config could not loaded - cannot proceed. Shutting down.");
    // Set up T212 API client
    let client = Client::from_config(&config)
        .expect("HTTP client could not be initialised - cannot proceed. Shutting down.");
    let client_arc = Arc::new(client);

    // Set up logger
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Set up SQLite connection
    let options = SqliteConnectOptions::from_str(&config.db_url)?
        .foreign_keys(true)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let pool_arc = Arc::new(pool);

    // Start the task to refresh all stocks
    let refresh_all_stocks_task = tokio::spawn(run_refresh_all_stocks_task(pool_arc, client_arc));

    tokio::join!(refresh_all_stocks_task).0?;

    Ok(())
}
