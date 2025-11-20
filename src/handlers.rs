use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::db::UserRole;

pub async fn handle_message(pool: &PgPool, bot: Bot, msg: Message)  -> ResponseResult<()>{
    if let Some(text) = msg.text() {
            match text {
                "/start" => {
                    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap();
                    let user_role = match db::get_user_role(pool, user_id).await {
                        Ok(role) => role,
                        Err(e) => {
                            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                            return Ok(());
                        }
                    };

                    let menu = match user_role {
                        UserRole::Admin => buttons::admin_menu(),
                        UserRole::User => buttons::main_menu(),
                        UserRole::NoAccess => {
                            bot.send_message(msg.chat.id, "У вас нет доступа").await?;
                            return Ok(());
                        }
                    };

                    let welcome_text = r#"👋🏻Привет\.
Этот бот сканирует рынок на:

📈Небольшой рост, для входа в ЛОНГ\;
🔺Памп \(резкий рост\), для входа в ШОРТ\;
🔻Дамп \(резкое падение цены\), для торговли на коррекцию падения\."#;

                    bot.send_message(msg.chat.id, welcome_text)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .reply_markup(menu)
                    .await?;
                }

                other => {
                bot.send_message(msg.chat.id, format!("Вы написали: {other}"))
                .await?;
            }
            }  
        }
        Ok(())
}