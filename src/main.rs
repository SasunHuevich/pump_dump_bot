use teloxide::prelude::*;
use std::sync::Arc;

mod handlers;
mod buttons;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let bot = Bot::from_env();
    let db_pool = Arc::new(db::init_db().await);

    teloxide::repl(bot, {
        let db_pool = Arc::clone(&db_pool);
        move |bot: Bot, msg: Message| {
            let db_pool = Arc::clone(&db_pool);
            async move { handlers::handle_message(&db_pool, bot, msg).await }
        }
    })
    .await;
}