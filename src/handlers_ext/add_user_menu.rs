use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};

pub async fn handle_add_user_menu(pool: &PgPool, bot: Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "➕ Пользователь" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                bot.send_message(msg.chat.id, format!("Введите ID пользователя для добавления:")).await?;
            }

            "➖ Пользователь" => {
                bot.send_message(msg.chat.id, format!("Затычка: ➖ Пользователь")).await?;
            }

            "➕ Админ" => {
                bot.send_message(msg.chat.id, format!("Затычка: ➕ Админ")).await?;
            }

            "✖️ Назад" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::MainMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                let menu = misc::get_main_menu_by_role(&user_role);

                bot.send_message(msg.chat.id, format!("Затычка: ✖️ Назад"))
                .reply_markup(menu)
                .await?;
            }

            other => {
                bot.send_message(msg.chat.id, format!("Вы написали: {other}")).await?;
            }
        }
    }

    Ok(())
}