use teloxide::prelude::*;
use sqlx::PgPool;


use crate::buttons;
use crate::db;
use crate::db::{UserRole, UserState};

pub async fn handle_message(pool: &PgPool, bot: Bot, msg: Message)  -> ResponseResult<()>{
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap();

    let user_role = match db::get_user_role(pool, user_id).await {
        Ok(role) => role,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
            return Ok(());
        }
    };

    if let UserRole::NoAccess = user_role {
        let message_text = format!(
            "Профиль пользователя {}\n\n🚨 Доступ к моим функциям закрыт.\nПо вопросам продления обратитесь к @lexcrypto1",
            msg.chat.id
        );
        bot.send_message(msg.chat.id, message_text).await?;
        return Ok(());
    }

    let state = match db::get_user_state(pool, user_id).await {
        Ok(state) => state,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
            return Ok(());
        }
    };

    match state {
        UserState::MainMenu => {
            if let Some(text) = msg.text() {
                match text {
                    "/start" => {
                        let menu = match user_role {
                            UserRole::Admin => buttons::admin_menu(),
                            UserRole::User => buttons::main_menu(),
                            UserRole::NoAccess => unreachable!(),
                        };

                        let welcome_text = r#"👋🏻Привет\.
Этот бот сканирует рынок на:

📈Небольшой рост, для входа в ЛОНГ\;
🔺Памп \(резкий рост\), для входа в ШОРТ\;
🔻Дамп \(резкое падение цены\), для торговли на коррекцию падения\."#;

                        bot.send_message(msg.chat.id, welcome_text)
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
        }

        UserState::AddUserMenu => {
            if let Some(text) = msg.text() {
                match text {
                    "➕ Пользователь" => {
                        bot.send_message(msg.chat.id, format!("Затычка: ➕ Пользователь")).await?;
                    }

                    "➖ Пользователь" => {
                        bot.send_message(msg.chat.id, format!("Затычка: ➖ Пользователь")).await?;
                    }

                    "➕ Админ" => {
                        bot.send_message(msg.chat.id, format!("Затычка: ➕ Админ")).await?;
                    }

                    "➖ Админ" => {
                        bot.send_message(msg.chat.id, format!("Затычка: ➖ Админ")).await?;
                    }

                    "✖️ Назад" => {
                        bot.send_message(msg.chat.id, format!("Затычка: ✖️ Назад")).await?;
                    }

                    other => {
                        bot.send_message(msg.chat.id, format!("Вы написали: {other}")).await?;
                    }
                }
            }
        }
    }
    
    Ok(())
}