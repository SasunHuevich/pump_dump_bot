use teloxide::prelude::*;
use sqlx::PgPool;
use chrono::NaiveDate;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};

pub async fn handle_add_user_button(pool: &PgPool, bot: Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
   if let Some(text) = msg.text() {
        match text {
            "✖️ Назад" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                    return Ok(())
                }

                bot.send_message(msg.chat.id, "Выберите действие:")
                .reply_markup(buttons::add_user_menu())
                .await?;

                return Ok(());
            }
            _ => {
                let parts: Vec<&str> = text.split_whitespace().collect();

                if parts.len() != 2 {
                    bot.send_message(msg.chat.id, "Неверный формат! Отправьте в виде:\n<user_id> <YYYY-MM-DD>")
                    .await?;
                    return Ok(());
                }

                let new_user_id = match parts[0].parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Некорректный ID. Введите число.")
                        .await?;
                        return Ok(());
                    }
                };

                let subscription_end_date = match NaiveDate::parse_from_str(parts[1], "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Некорректная дата, формат должен быть YYYY-MM-DD.")
                        .await?;
                        return Ok(());
                    }
                };

                match db::add_user(pool, new_user_id, subscription_end_date).await {
                    Ok(_) => {
                        if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                            return Ok(())
                        }

                        bot.send_message(msg.chat.id, format!("Пользователь {} успешно добавлен!", new_user_id))
                            .reply_markup(buttons::add_user_menu())
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка при добавлении пользователя: {}", e))
                            .await?;
                    }
                }   

                
            }
        }
    }

    Ok(())
}