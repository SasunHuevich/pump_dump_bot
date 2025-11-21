use teloxide::prelude::*;
use sqlx::PgPool;

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

                bot.send_message(msg.chat.id, "Выберите действие:").await?;

                return Ok(());
            }
            _ => {
                    match text.parse::<i64>() {
                    Ok(new_user_id) => {
                        match db::add_user(pool, new_user_id).await {
                            Ok(_) => {
                                if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                                    return Ok(())
                                }

                                bot.send_message(msg.chat.id, format!("Пользователь {} успешно добавлен!", new_user_id))
                                    .await?;
                            }
                            Err(e) => {
                                bot.send_message(msg.chat.id, format!("Ошибка при добавлении пользователя: {}", e))
                                    .await?;
                            }
                        }
                    }
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Некорректный ID. Пожалуйста, отправьте числовой ID пользователя.")
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}