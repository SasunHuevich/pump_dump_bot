use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::db::{UserState};

pub async fn handle(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64)  -> ResponseResult<()> {
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
                let del_user_id = match text.parse::<i64>() {
                    Ok(id) => id,
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Некорректный ID. Введите число.")
                        .await?;
                        return Ok(());
                    }
                };

                if del_user_id == user_id {
                    bot.send_message(msg.chat.id, "Вы не можете удалить себя.")
                        .await?;
                        return Ok(()); 
                }

                match db::delete_user(pool, del_user_id).await {
                    Ok(_) => {
                        if let Err(e) = db::set_user_state(pool, user_id, &UserState::AddUserMenu).await {
                            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
                            return Ok(())
                        }

                        bot.send_message(msg.chat.id, format!("Пользователь {} успешно удалён!", del_user_id))
                            .reply_markup(buttons::add_user_menu())
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка при удалении пользователя: {}", e))
                            .await?;
                    }
                }   

                
            }
        }
    }

    Ok(())
}