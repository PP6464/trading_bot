use crate::config::config::Config;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use reqwest::Url;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    auth_header: String,
}

macro_rules! method_request_builder {
    ($method:ident) => {
        /// Returns a request builder for this method.
        /// Attaches the authorisation header automatically.
        /// Panics if the endpoint starts with '/' or is invalid.
        pub fn $method(&self, endpoint: &str) -> reqwest::RequestBuilder {
            assert!(!endpoint.starts_with('/'), "Endpoint starts with a '/'");
            let url = self.base_url.join(endpoint).expect("Endpoint is invalid");
            self.client
                .$method(url)
                .header("Authorization", &self.auth_header)
        }
    };
}

impl Client {
    /// Create a custom client from the env config.
    pub fn from_config(config: &Config) -> Option<Client> {
        Client {
            client: reqwest::Client::new(),
            base_url: Url::parse(&config.api_url_base).ok()?,
            auth_header: format!(
                "Basic {}",
                STANDARD.encode(format!("{}:{}", config.api_key, config.api_secret))
            ),
        }
        .into()
    }

    method_request_builder!(get);
    method_request_builder!(post);
}
