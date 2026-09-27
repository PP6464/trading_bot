#[derive(Debug, Clone)]
pub struct Config {
    pub api_url_base: String,
    pub api_key: String,
    pub api_secret: String,
    pub db_url: String,
    pub history_length_days: u32,
    pub deactivated_purge_days: u32,
}

impl Config {
    /// Loads the configuration from the env.
    /// This ensures the API url ends in a trailing backslash.
    pub fn load() -> Option<Config> {
        dotenvy::dotenv().ok()?;
        Self::from_env(|key| std::env::var(key).ok())
    }

    pub fn from_env(get_var: impl Fn(&str) -> Option<String>) -> Option<Config> {
        let env = get_var("ENV")?;
        let history_length_days = get_var("HISTORY_LENGTH_DAYS")?.parse::<u32>().ok()?;
        let deactivated_purge_days = get_var("DEACTIVATION_PURGE_DAYS")?.parse::<u32>().ok()?;
        let db_url = get_var("DATABASE_URL")?;
        match env.as_str() {
            "demo" => {
                let api_url = get_var("DEMO_API_URL")?;
                if !api_url.ends_with('/') {
                    return None;
                }
                let api_key = get_var("DEMO_API_KEY")?;
                let api_secret = get_var("DEMO_API_SECRET")?;
                Some(Config {
                    api_url_base: api_url,
                    api_key,
                    api_secret,
                    db_url,
                    history_length_days,
                    deactivated_purge_days,
                })
            }
            "live" => {
                let api_url = get_var("LIVE_API_URL")?;
                if !api_url.ends_with('/') {
                    return None;
                }
                let api_key = get_var("LIVE_API_KEY")?;
                let api_secret = get_var("LIVE_API_SECRET")?;
                Some(Config {
                    api_url_base: api_url,
                    api_key,
                    api_secret,
                    db_url,
                    history_length_days,
                    deactivated_purge_days,
                })
            }
            _ => None,
        }
    }
}
