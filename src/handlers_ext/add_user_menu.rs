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
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e))
                    .await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                bot.send_message(
                    msg.chat.id,
                    "Введите ID пользователя и дату окончания подписки.\n\nФормат:\n<user_id> <YYYY-MM-DD>\n\nНапример:\n123456789 2025-01-15"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }

            "➕ Админ" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddAdminButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e))
                    .await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                bot.send_message(
                    msg.chat.id,
                    "Введите ID пользователя"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }

            "➖ Пользователь/Aдмин" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::DeleteUserButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e))
                    .await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                bot.send_message(
                    msg.chat.id,
                    "Введите ID пользователя"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }

            "✖️ Назад" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::MainMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                let menu = misc::get_main_menu_by_role(&user_role);

                bot.send_message(msg.chat.id, format!("✖️ Назад"))
                .reply_markup(menu)
                .await?;
            }

            other => {
                bot.send_message(msg.chat.id, "Не понял команду 😕").await?;
            }
        }
    }

    Ok(())
}