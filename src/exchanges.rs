use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use log::{warn, error};

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
        .get("https://fapi.binance.com/fapi/v2/ticker/price")
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


#[derive(Deserialize)]
pub struct BybitPrice {
    pub symbol: String,
    pub last_price: String,
}

#[derive(Deserialize)]
struct BybitResponse {
    result: BybitResult,
}

#[derive(Deserialize)]
struct BybitResult {
    list: Vec<BybitPrice>,
}

pub async fn fetch_bybit_prices(client: &Client) -> Result<Vec<UnifiedPrice>, reqwest::Error> {
    let resp = client
        .get("https://api.bybit.com/v5/market/tickers?category=linear")
        .send()
        .await?
        .json::<BybitResponse>()
        .await?;

    let unified: Vec<UnifiedPrice> = resp.result.list
        .into_iter()
        .filter_map(|p| {
        match p.last_price.parse::<f64>() {
            Ok(last_price) => Some(UnifiedPrice {
                currency: p.symbol,
                price: last_price,
                exchange: "bybit".to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
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

pub async fn update_bybit_prices(pool: &PgPool, client: &Client) {
    match fetch_bybit_prices(client).await {
        Ok(prices) => {
            if let Err(e) = db::insert_currency_prices_bulk(pool, &prices).await {
                error!("Ошибка insert_prices_bulk: {}", e);
            }
        }
        Err(e) => {
            error!("Ошибка получения цен с Bybit: {}", e);
        }
    }
}


#[derive(Deserialize)]
pub struct BingxPrice {
    pub symbol: String,
    pub price: String,
    pub time: i64,
}

#[derive(Deserialize)]
struct BingxResponse {
    data: Vec<BingxPrice>,
}

pub async fn fetch_bingx_prices(client: &Client) -> Result<Vec<UnifiedPrice>, reqwest::Error> {
    let resp = client
        .get("https://open-api.bingx.com/openApi/swap/v2/quote/price")
        .send()
        .await?
        .json::<BingxResponse>()
        .await?;

    let unified: Vec<UnifiedPrice> = resp.data
        .into_iter()
        .filter_map(|p| {
        match p.price.parse::<f64>() {
            Ok(price) => Some(UnifiedPrice {
                currency: p.symbol,
                price: price,
                exchange: "bingx".to_string(),
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

pub async fn update_bingx_prices(pool: &PgPool, client: &Client) {
    match fetch_bingx_prices(client).await {
        Ok(prices) => {
            if let Err(e) = db::insert_currency_prices_bulk(pool, &prices).await {
                error!("Ошибка insert_prices_bulk: {}", e);
            }
        }
        Err(e) => {
            error!("Ошибка получения цен с Bingx: {}", e);
        }
    }
}