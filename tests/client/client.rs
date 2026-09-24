#[cfg(test)]
mod client_tests {
    use trading_bot::client::client::Client;
    use trading_bot::config::config::Config;
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use reqwest::Method;

    fn client() -> Client {
        Client::from_config(&Config {
            api_url_base: "https://api.example/v1/".to_owned(),
            api_key: "key".to_owned(),
            api_secret: "secret".to_owned(),
        })
            .expect("valid configuration should create a client")
    }

    #[test]
    fn creates_authenticated_get_request_with_resolved_url() {
        let request = client()
            .get("equity/account/info")
            .build()
            .expect("request should build");

        assert_eq!(request.method(), Method::GET);
        assert_eq!(
            request.url().as_str(),
            "https://api.example/v1/equity/account/info"
        );
        assert_eq!(
            request.headers()["Authorization"],
            format!("Basic {}", STANDARD.encode("key:secret"))
        );
    }

    #[test]
    fn creates_authenticated_post_request() {
        let request = client()
            .post("orders")
            .build()
            .expect("request should build");

        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.url().as_str(), "https://api.example/v1/orders");
        assert_eq!(
            request.headers()["Authorization"],
            format!("Basic {}", STANDARD.encode("key:secret"))
        );
    }

    #[test]
    fn rejects_invalid_base_url() {
        let config = Config {
            api_url_base: "not a url".to_owned(),
            api_key: "key".to_owned(),
            api_secret: "secret".to_owned(),
        };

        assert!(Client::from_config(&config).is_none());
    }

    #[test]
    #[should_panic(expected = "Endpoint starts with a '/'")]
    fn rejects_endpoint_starting_with_slash() {
        let _ = client().get("/orders");
    }
}
