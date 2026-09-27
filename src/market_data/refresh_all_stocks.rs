use std::sync::Arc;
use std::time::Duration;
use sqlx::SqlitePool;
use crate::client::client::Client;
use crate::market_data::models::Instrument;

pub async fn refresh_all_stocks(pool: &SqlitePool, client: &Client) -> anyhow::Result<()> {
    let instruments: Vec<Instrument> = client
        .get("equity/metadata/instruments")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mut tx = pool.begin().await?;

    for instrument in instruments.iter().filter(|i| i.instrument_type == "STOCK") {
        sqlx::query!(
            "INSERT INTO stock (ticker) VALUES (?1) ON CONFLICT (ticker) DO NOTHING",
            instrument.ticker
        ).execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn run_refresh_all_stocks_task(pool: Arc<SqlitePool>, client: Arc<Client>) {
    let mut interval = tokio::time::interval(Duration::from_mins(7 * 24 * 60));
    loop {
        interval.tick().await;
        if let Err(e) = refresh_all_stocks(&*pool, &*client).await {
            tracing::error!("[REFRESH ALL STOCKS]: Task failed with error: {e:#}")
        }
    }
}
