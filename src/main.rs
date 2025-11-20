use teloxide::prelude::*;

mod handlers;
mod buttons;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let bot = Bot::from_env();
    let db = db::init_db().await;

    db::add_user(&db, 76767, "GG").await.unwrap();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        handlers::handle_message(bot, msg).await
    }).await;
}