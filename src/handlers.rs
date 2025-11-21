use teloxide::prelude::*;
use sqlx::PgPool;


use crate::buttons;
use crate::db;
use crate::db::{UserRole, UserState};
use crate::texts::*;
use crate::misc;

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
                        let menu = misc::get_main_menu_by_role(&user_role);

                        bot.send_message(msg.chat.id, START_MESSAGE)
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
        }

        UserState::AddUserButton => {
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
        }
    }
    
    Ok(())
}