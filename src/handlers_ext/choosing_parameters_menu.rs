use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole, EnableExchanges};

pub async fn handle_choosing_parameters_menu(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "✖️ Назад" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::MainMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(());
                }

                let menu = misc::get_main_menu_by_role(&user_role);
                bot.send_message(msg.chat.id, "✖️ Назад")
                    .reply_markup(menu)
                    .await?;
            }
            "📈 Период лонг" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::LongPeriodButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "🕑 Введите новый период времени: от 1 до 30 минут"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            "% лонга" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::LongPercentButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "📈 Введите новый % изменения цены: от 0.5% до 100%"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            "🔺 Период памп" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::ShortPeriodButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "🕑 Введите новый период времени: от 1 до 30 минут"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            "% пампа" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::ShortPercentButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "🔺 Введите новый % изменения цены: от 0.5% до 100%"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            "🔻 Период падение" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::DropPeriodButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "🕑 Введите новый период времени: от 1 до 30 минут"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            "% падения" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::DropPercentButton).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    return Ok(()); // Эту функцию сделать с логированием и и ф отдельную функцию чтобы так часто не повторяться
                }

                bot.send_message(
                    msg.chat.id,
                    "🔻 Введите новый % изменения цены: от 0.5% до 100%"
                )
                .reply_markup(buttons::back_menu())
                .await?;
            }
            other => {
                bot.send_message(msg.chat.id, format!("Не понял команду 😕: {}", other)).await?;
            }
        }
    }


    Ok(())
}