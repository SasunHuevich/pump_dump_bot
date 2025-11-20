use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            match text {
                "/start" => {
                    let welcome_text = r#"👋🏻Привет\.
Этот бот сканирует рынок на:

📈Небольшой рост, для входа в ЛОНГ\;
🔺Памп \(резкий рост\), для входа в ШОРТ\;
🔻Дамп \(резкое падение цены\), для торговли на коррекцию падения\."#;
                    bot.send_message(msg.chat.id, welcome_text)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
                }

                other => {
                bot.send_message(msg.chat.id, format!("Вы написали: {other}"))
                .await?;
            }
            }  
        }
        Ok(())
    }).await;
}