use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};
use crate::texts::*;

pub async fn handle_main_menu(pool: &PgPool, bot: Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "/start" => {
                let menu = misc::get_main_menu_by_role(&user_role);

                bot.send_message(msg.chat.id, START_MESSAGE)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .reply_markup(menu)
                .await?;
            }

            "🏦 Выбор биржи" => {
                bot.send_message(msg.chat.id, format!("Затычка: 🏦 Выбор бирж")).await?;
            }

            "⚒️ Настроить" => {
                bot.send_message(msg.chat.id, format!("Затычка: ⚒️ Настроить")).await?;
            }

            "🔐 Получить доступ" => {
                bot.send_message(msg.chat.id, format!("Затычка: 🔐 Получить доступ")).await?;
            }

            "💬 Поддержка" => {
                bot.send_message(msg.chat.id, format!("Затычка: 💬 Поддержка")).await?;
            }

            "👥 Управление пользователями" => {
                if user_role != UserRole::Admin {
                    bot.send_message(msg.chat.id, format!("Недоступно!")).await?;
                    return Ok(())
                }

                if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                }


                bot.send_message(msg.chat.id, format!("Выберите действие:"))
                .reply_markup(buttons::add_user_menu())
                .await?;
            }

            other => {
                bot.send_message(msg.chat.id, format!("Вы написали: {other}")).await?;
            }
        }
    }

    Ok(())
}