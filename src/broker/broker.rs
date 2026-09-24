use crate::client::client::Client;
use super::models::Position;

pub struct Broker {
    client: Client,
}

impl Broker {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_positions(&self) -> reqwest::Result<Vec<Position>> {
        self.client
            .get("equity/positions")
            .send()
            .await?
            .json()
            .await
    }
}