
#[cfg(test)]
mod config_tests {
    use trading_bot::config::config::Config;
    use std::collections::HashMap;

    fn config_from(values: &[(&str, &str)]) -> Option<Config> {
        let values = values
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect::<HashMap<_, _>>();

        Config::from_env(|key| values.get(key).cloned())
    }

    #[test]
    fn loads_demo_configuration() {
        let config = config_from(&[
            ("ENV", "demo"),
            ("DEMO_API_URL", "https://demo.example/api/"),
            ("DEMO_API_KEY", "demo-key"),
            ("DEMO_API_SECRET", "demo-secret"),
        ])
            .expect("demo configuration should load");

        assert_eq!(config.api_url_base, "https://demo.example/api/");
        assert_eq!(config.api_key, "demo-key");
        assert_eq!(config.api_secret, "demo-secret");
    }

    #[test]
    fn loads_live_configuration() {
        let config = config_from(&[
            ("ENV", "live"),
            ("LIVE_API_URL", "https://live.example/api/"),
            ("LIVE_API_KEY", "live-key"),
            ("LIVE_API_SECRET", "live-secret"),
        ])
            .expect("live configuration should load");

        assert_eq!(config.api_url_base, "https://live.example/api/");
        assert_eq!(config.api_key, "live-key");
        assert_eq!(config.api_secret, "live-secret");
    }

    #[test]
    fn rejects_unknown_environment() {
        assert!(config_from(&[("ENV", "staging")]).is_none());
    }

    #[test]
    fn rejects_api_url_without_trailing_slash() {
        assert!(config_from(&[
            ("ENV", "demo"),
            ("DEMO_API_URL", "https://demo.example/api"),
            ("DEMO_API_KEY", "demo-key"),
            ("DEMO_API_SECRET", "demo-secret"),
        ])
            .is_none());
    }

    #[test]
    fn rejects_missing_credentials() {
        assert!(config_from(&[
            ("ENV", "live"),
            ("LIVE_API_URL", "https://live.example/api/"),
            ("LIVE_API_KEY", "live-key"),
        ])
            .is_none());
    }
}