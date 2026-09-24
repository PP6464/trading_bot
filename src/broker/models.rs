use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Position {
    #[serde(rename = "averagePricePaid")]
    pub average_price_paid: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "currentPrice")]
    pub current_price: f64,
    pub instrument: Instrument,
    pub quantity: f64,
    #[serde(rename = "quantityAvailableForTrading")]
    pub quantity_available_for_trading: f64,
    #[serde(rename = "quantityInPies")]
    pub quantity_in_pies: f64,
    #[serde(rename = "walletImpact")]
    pub wallet_impact: WalletImpact,
}

#[derive(Debug, Deserialize)]
pub struct Instrument {
    pub currency: String,
    pub isin: String,
    pub name: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct WalletImpact {
    pub currency: String,
    #[serde(rename = "currentValue")]
    pub current_value: f64,
    #[serde(rename = "fxImpact")]
    pub fx_impact: f64,
    #[serde(rename = "totalCost")]
    pub total_cost: f64,
    #[serde(rename = "unrealizedProfitLoss")]
    pub unrealized_profit_loss: f64,
}