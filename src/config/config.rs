#[derive(Debug, Clone)]
pub struct Config {
    pub api_url_base: String,
    pub api_key: String,
    pub api_secret: String,
}

impl Config {
    /// Loads the configuration from the env.
    /// This ensures the API url ends in a trailing backslash.
    pub fn load() -> Option<Config> {
        dotenvy::dotenv().ok()?;
        let env = std::env::var("ENV").ok()?;
        match env.as_str() {
            "demo" => {
                let api_url = std::env::var("DEMO_API_URL").ok()?;
                if !api_url.ends_with('/') {
                    return None;
                }
                let api_key = std::env::var("DEMO_API_KEY").ok()?;
                let api_secret = std::env::var("DEMO_API_SECRET").ok()?;
                Some(Config {
                    api_url_base: api_url,
                    api_key,
                    api_secret,
                })
            }
            "live" => {
                let api_url = std::env::var("LIVE_API_URL").ok()?;
                if !api_url.ends_with('/') {
                    return None;
                }
                let api_key = std::env::var("LIVE_API_KEY").ok()?;
                let api_secret = std::env::var("LIVE_API_SECRET").ok()?;
                Some(Config {
                    api_url_base: api_url,
                    api_key,
                    api_secret,
                })
            }
            _ => None,
        }
    }
}