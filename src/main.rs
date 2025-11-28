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
    env_logger::Builder::from_env(env_logger::Env::default())
    .format_target(false)
    .format_timestamp_secs()
    .init();

    let bot = Arc::new(Bot::from_env());
    let db_pool = Arc::new(db::init_db().await);
    let MAX_INTRVAL_MINUTES: i32 = 30;
    info!("MAIN");

    {
        let db_pool = Arc::clone(&db_pool);
        let bot = Arc::clone(&bot);

        tokio::spawn(async move {
            let client = Client::new();

            loop {
                exchanges::update_binance_prices(&db_pool, &client).await;
                exchanges::update_bybit_prices(&db_pool, &client).await;
                exchanges::update_bingx_prices(&db_pool, &client).await;

                misc::process_notifications(&db_pool, &*bot, MAX_INTRVAL_MINUTES).await;

                db::delete_old_currency_prices(&db_pool).await;
                db::delete_old_user_notifications(&db_pool).await;
                
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            }
        });
    }

    // TODO: сейчас если нет соединения бинарь падает, но нужно чтобы он сам себя пробовал возродить наверн, типо чтобы после падеиня сам восстановился
    // хотя иногда контейнер стартует без интернета внутри мб так и лушче тут хз, мб через докер рестарт сделать
    teloxide::repl(bot, {
        let db_pool = Arc::clone(&db_pool);

        move |bot: Arc<Bot>, msg: Message| {
            let db_pool = Arc::clone(&db_pool);
            async move { handlers::handle_message(&db_pool, &*bot, msg).await }
        }
    })
    .await;
}