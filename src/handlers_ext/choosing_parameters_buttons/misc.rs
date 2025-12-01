use teloxide::prelude::*;
use sqlx::PgPool;
use chrono::NaiveDate;

use crate::buttons;
use crate::db;
use crate::db::{UserState, UserRole};

pub async fn back_to_choosing_parameters_menu(
    bot: &Bot,
    pool: &PgPool,
    chat_id: ChatId,
    user_id: i64,
) -> ResponseResult<()> {
    if let Err(e) = db::set_user_state(pool, user_id, &UserState::ChoosingParametersMenu).await {
        bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
        return Ok(())
    }

    let user_settings = match db::get_user_settings_by_id(pool, user_id).await {
        Ok(settings) => settings,
        Err(e) => {
            bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?;
            return Ok(());
        }
    };

    let message = format!(
        "Текущие настройки:\n\n\
        📈 Лонг: {}% за {} мин.\n\
        🔺 Шорт: {}% за {} мин.\n\
        🔻 Падение: {}% за {} мин.",
        user_settings.long_percent,
        user_settings.long_minutes,
        user_settings.short_percent,
        user_settings.short_minutes,
        user_settings.drop_percent,
        user_settings.drop_minutes
    );

    // TODO: во всех местах гед смена состояния если что-то пошло не так надо бы вернуть обратно
    bot.send_message(chat_id, message)
    .reply_markup(buttons::parameters_menu())
    .await?;

    Ok(())
}

pub async fn update_time_setting(
    bot: &Bot,
    pool: &PgPool,
    chat_id: ChatId,
    user_id: i64,
    text: &str,
    field_name: &str
) -> ResponseResult<()> {
    let new_time = match text.parse::<i64>() {
        Ok(time) => time,
        Err(_) => {
            bot.send_message(chat_id, "Некорректное время. Введите целое число.")
            .await?;
            return Ok(());
        }
    };

    if new_time < 1 || new_time > 30 {
        bot.send_message(chat_id, "Некорректное время. Время должно быть от 1 до 30 минут.")
        .await?;
        return Ok(());
    }

    if let Err(e) = db::update_user_setting(pool, user_id, field_name, new_time).await {
        bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?;
        return Ok(()); //TODO
    }

    let message = format!(
        "✔️ Период времени сохранён: {} мин.",
        new_time
    );

    if let Err(e) = db::set_user_state(pool, user_id, &UserState::ChoosingParametersMenu).await {
        bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?; //TODO: Все такие места обработать по нормальному
        return Ok(())
    }

    bot.send_message(chat_id, message)
    .reply_markup(buttons::parameters_menu())
    .await?;

    Ok(())
}

pub async fn update_percent_setting(
    bot: &Bot,
    pool: &PgPool,
    chat_id: ChatId,
    user_id: i64,
    text: &str,
    field_name: &str
) -> ResponseResult<()> {
    let new_percent = match text.parse::<f64>() {
        Ok(p) => p,
        Err(_) => {
            bot.send_message(chat_id, "Некорректный процент. Введите число.")
                .await?;
            return Ok(());
        }
    };

    if new_percent < 0.5 || new_percent > 100.0 {
        bot.send_message(chat_id, "Процент должен быть от 0.5 до 100.")
            .await?;
        return Ok(());
    }

    if let Err(e) = db::update_user_setting(pool, user_id, field_name, new_percent).await {
        bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?; //TODO: в таких местах *** потом найду где именно ошибка
        return Ok(());
    }

    if let Err(e) = db::set_user_state(pool, user_id, &UserState::ChoosingParametersMenu).await {
        bot.send_message(chat_id, format!("Ошибка БД: {}", e)).await?;
        return Ok(());
    }

    let message = format!("✔️ Процент сохранён: {}%", new_percent);

    bot.send_message(chat_id, message)
        .reply_markup(buttons::parameters_menu())
        .await?;

    Ok(())
}