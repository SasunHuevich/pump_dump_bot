use teloxide::prelude::*;

mod handlers;
mod buttons;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        handlers::handle_message(bot, msg).await
    }).await;
}