use teloxide::prelude::*;
use std::sync::Arc;
use tokio;
use reqwest::Client;
use log::{info, warn, error};

mod handlers;
mod buttons;
mod db;
mod texts;
mod misc;
mod handlers_ext;
mod exchanges;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot = Bot::from_env();
    let db_pool = Arc::new(db::init_db().await);

    {
        let db_pool = Arc::clone(&db_pool);
        tokio::spawn(async move {
            let client = Client::new();

            loop {
                exchanges::update_binance_prices(&db_pool, &client).await;
                exchanges::update_bybit_prices(&db_pool, &client).await;

                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            }
        });
    }

    teloxide::repl(bot, {
        let db_pool = Arc::clone(&db_pool);
        move |bot: Bot, msg: Message| {
            let db_pool = Arc::clone(&db_pool);
            async move { handlers::handle_message(&db_pool, bot, msg).await }
        }
    })
    .await;
}