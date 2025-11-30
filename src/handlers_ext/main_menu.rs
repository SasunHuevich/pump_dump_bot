use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};
use crate::texts::*;

pub async fn handle_main_menu(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
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
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::ChoosingExchangeMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                }

                 let enable_exchanges = match db::get_exchanges(pool, user_id).await {
                    Ok(ex) => ex,
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                        return Ok(());
                    }
                };

                let menu = buttons::exchange_menu(user_id, enable_exchanges);

                bot.send_message(
                    msg.chat.id,
                    "Включить / выключить сигналы с бирж:\n\n✅ - биржа ВКЛЮЧЕНА\n❌ - биржа ВЫКЛЮЧЕНА",
                )
                .reply_markup(menu)
                .await?;
            }

            "⚒️ Настроить" => {
                if let Err(e) = db::set_user_state(pool, user_id, &UserState::ChoosingParametersMenu).await {
                    bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                    //TODO: ретурн
                }

                let user_settings = match db::get_user_settings_by_id(pool, user_id).await {
                    Ok(settings) => settings,
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                        return Ok(());
                    }
                };

                let message = format!(
                    "*Текущие настройки:*\n\n\
                    📈 *Лонг:* {}% за {} мин.\n\
                    🔺 *Шорт:* {}% за {} мин.\n\
                    🔻 *Падение:* {}% за {} мин.",
                    user_settings.long_percent,
                    user_settings.long_minutes,
                    user_settings.short_percent,
                    user_settings.short_minutes,
                    user_settings.drop_percent,
                    user_settings.drop_minutes
                );

                // TODO: во всех местах гед смена состояния если что-то пошло не так надо бы вернуть обратно
                bot.send_message(msg.chat.id, message)
                .reply_markup(buttons::parameters_menu())
                .await?;
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
                bot.send_message(msg.chat.id, "Не понял команду 😕").await?;
            }
        }
    }

    Ok(())
}