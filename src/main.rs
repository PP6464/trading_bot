use trading_bot::client::client::Client;
use trading_bot::config::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config =
        Config::load().expect("Config could not loaded - cannot proceed. Shutting down.");
    let client = Client::from_config(&config)
        .expect("HTTP client could not be initialised - cannot proceed. Shutting down.");

    let resp = client.get("equity/account/info").send().await?;

    println!("Status: {}", resp.status());
    println!("Headers: {:?}", resp.headers());
    println!("{}", resp.text().await?);
    Ok(())
}
