use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use log::{info, warn, error};

use crate::db;
use crate::db::{UnifiedPrice};


#[derive(serde::Deserialize)]
pub struct BinancePrice {
    pub symbol: String,
    pub price: String,
    pub time: i64,
}

pub async fn fetch_binance_prices(client: &Client) -> Result<Vec<UnifiedPrice>, reqwest::Error> {
    let resp = client
        .get("https://fapi.binance.com/fapi/v1/ticker/price")
        .send()
        .await?
        .json::<Vec<BinancePrice>>()
        .await?;

    let unified: Vec<UnifiedPrice> = resp
        .into_iter()
        .filter_map(|p| {
        match p.price.parse::<f64>() {
            Ok(price) => Some(UnifiedPrice {
                currency: p.symbol,
                price,
                exchange: "binance".to_string(),
                timestamp: p.time,
            }),
            Err(e) => {
                warn!("Не удалось распарсить цену для {}: {}", p.symbol, e);
                None
            }
        }
    })
    .collect();

    Ok(unified)
}

pub async fn update_binance_prices(pool: &PgPool, client: &Client) {
    match fetch_binance_prices(client).await {
        Ok(prices) => {
            if let Err(e) = db::insert_currency_prices_bulk(pool, &prices).await {
                error!("Ошибка insert_prices_bulk: {}", e);
            }
        }
        Err(e) => {
            error!("Ошибка получения цен с Binance: {}", e);
        }
    }
}