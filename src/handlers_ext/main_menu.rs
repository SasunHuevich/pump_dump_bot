use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::db::{UserState, UserRole};
use crate::handlers_ext::choosing_parameters_buttons::misc::back_to_choosing_parameters_menu;

pub async fn handle(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
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

                let menu = buttons::exchange_menu(enable_exchanges);

                bot.send_message(
                    msg.chat.id,
                    "Включить / выключить сигналы с бирж:\n\n✅ - биржа ВКЛЮЧЕНА\n❌ - биржа ВЫКЛЮЧЕНА",
                )
                .reply_markup(menu)
                .await?;
            }

            "⚒️ Настроить" => {
                back_to_choosing_parameters_menu(bot, pool, msg.chat.id, user_id).await?;
            }

            "🔐 Проверить доступ" => {
                match db::get_subscription_end_date(pool, user_id).await {
                    Ok(Some(date)) => {
                        let today = chrono::Local::now().naive_local().date();

                        let status = if date >= today {
                            format!("✅ Активна до {}", date.format("%d-%m-%Y"))
                        } else {
                            format!("❌ Неактивна с {}", date.format("%d-%m-%Y"))
                        };

                        let message = format!(
                            "👤 Профиль пользователя: {}\n🔐 Статус подписки:\n{}.",
                            user_id,
                            status
                        );

                        bot.send_message(msg.chat.id, &message).await?;
                    }

                    Ok(None) => {
                        let message = format!(
                            "👤 Профиль пользователя: {}\n🔐 Статус подписки:\n❌ Не активна",
                            user_id
                        );
                        
                        bot.send_message(msg.chat.id, &message).await?;
                    }

                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Ошибка чтения подписки: {}", e)).await?;
                    }
                }
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

            _ => {
                bot.send_message(msg.chat.id, "Не понял команду 😕").await?;
            }
        }
    }

    Ok(())
}