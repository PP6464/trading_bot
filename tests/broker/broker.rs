#[cfg(test)]
mod broker_tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    use trading_bot::broker::broker::Broker;
    use trading_bot::client::client::Client;
    use trading_bot::config::config::Config;

    fn client_for(listener: &TcpListener) -> Client {
        Client::from_config(&Config {
            api_url_base: format!("http://{}/v1/", listener.local_addr().unwrap()),
            api_key: "key".to_owned(),
            api_secret: "secret".to_owned(),
        })
        .expect("test configuration should create a client")
    }

    #[tokio::test]
    async fn gets_and_deserializes_positions() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("test server should bind");
        let client = client_for(&listener);
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("client should connect");
            let mut request = [0; 2048];
            let bytes_read = stream
                .read(&mut request)
                .expect("request should be readable");
            let request = String::from_utf8_lossy(&request[..bytes_read]);

            assert!(request.starts_with("GET /v1/equity/positions HTTP/1.1\r\n"));
            assert!(request.contains("authorization: Basic a2V5OnNlY3JldA=="));

            let body = r#"[{
                "averagePricePaid": 100.5,
                "createdAt": "2026-01-01T00:00:00Z",
                "currentPrice": 110.0,
                "instrument": {
                    "currency": "EUR",
                    "isin": "TEST00000001",
                    "name": "Test instrument",
                    "ticker": "TEST"
                },
                "quantity": 2.0,
                "quantityAvailableForTrading": 2.0,
                "quantityInPies": 0.0,
                "walletImpact": {
                    "currency": "EUR",
                    "currentValue": 220.0,
                    "fxImpact": 0.0,
                    "totalCost": 201.0,
                    "unrealizedProfitLoss": 19.0
                }
            }]
            "#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("response should be writable");
        });

        let broker = Broker::new(client);
        let positions = broker
            .get_positions()
            .await
            .expect("valid position response should deserialize");

        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0].instrument.ticker, "TEST");
        assert_eq!(positions[0].quantity, 2.0);
        assert_eq!(positions[0].wallet_impact.unrealized_profit_loss, 19.0);
        server.join().expect("test server should finish");
    }

    #[tokio::test]
    async fn returns_an_error_for_invalid_position_json() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("test server should bind");
        let client = client_for(&listener);
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("client should connect");
            let mut request = [0; 1024];
            stream
                .read(&mut request)
                .expect("request should be readable");

            let body = "{invalid json}";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .expect("response should be writable");
        });

        let broker = Broker::new(client);
        assert!(broker.get_positions().await.is_err());
        server.join().expect("test server should finish");
    }
}
