use teloxide::prelude::*;
use sqlx::PgPool;
use chrono::NaiveDate;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};

pub async fn handle_add_admin_button(pool: &PgPool, bot: Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
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
                let new_admin_id = match text.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Некорректный ID. Введите число.")
                        .await?;
                        return Ok(());
                    }
                };

                match db::add_admin(pool, new_admin_id).await {
                    Ok(_) => {
                        if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                            return Ok(())
                        }

                        bot.send_message(msg.chat.id, format!("Администратор {} успешно добавлен!", new_admin_id))
                            .reply_markup(buttons::add_user_menu())
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка при добавлении администратора: {}", e))
                            .await?;
                    }
                }   

                
            }
        }
    }

    Ok(())
}