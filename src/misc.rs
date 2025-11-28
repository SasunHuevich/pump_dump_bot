use teloxide::types::{KeyboardMarkup};
use sqlx::PgPool;
use log::{info, warn, error};
use teloxide::prelude::*;
use std::collections::HashMap;
use teloxide::types::ChatId;

use crate::db::{UserRole, PriceRow, UserSettings};
use crate::buttons;
use crate::db;

pub fn get_main_menu_by_role(user_role: &UserRole) -> KeyboardMarkup {
    match user_role {
        UserRole::Admin => buttons::admin_menu(),
        UserRole::User => buttons::main_menu(),
        UserRole::NoAccess => unreachable!(),
    }
}

pub fn get_grouped_prices(rows: Vec<PriceRow>) -> HashMap<(String, String), Vec<f64>> {
    let mut prices_map: HashMap<(String, String), Vec<f64>> = HashMap::new();

    info!("Входящий вектор rows, длина = {}", rows.len());

    for row in rows {
        let key = (
            row.currency.trim().to_string(), 
            row.exchange.trim().to_string()
        );

        let vec_ref = prices_map.entry(key.clone()).or_default();
        
        vec_ref.push(row.price);
    }

    prices_map
}

#[derive(Debug)]
struct PriceChange {
    currency: String,
    exchange: String, 
    change_percent: f64,
}

pub fn compute_all_procent_for_intevals(prices_map: &HashMap<(String, String), Vec<f64>>, max_interval_minutes: i32) -> HashMap<i32, Vec<PriceChange>> {
    info!("compute_all_procent");
    let mut interval_percent_map: HashMap<i32, Vec<PriceChange>> = HashMap::new();

    for interval_minutes in 1..=max_interval_minutes {
        let mut changes = Vec::new();

        for ((currency, exchange), prices) in prices_map {
            if prices.len() < 2 {
                warn!(
                    "Недостаточно данных для вычисления изменения цены: currency={} exchange={} data_points={}",
                    currency,
                    exchange,
                    prices.len()
                );
                continue;
            }

            let actual_interval = prices.len().min(interval_minutes as usize);

            let start_price = prices[prices.len() - actual_interval as usize];
            let end_price = prices[prices.len() - 1];

            let change_percent = ((end_price - start_price) / start_price) * 100.0;

            changes.push(
                PriceChange {
                    currency: currency.clone(),
                    exchange: exchange.clone(),
                    change_percent,
                }
            );
        }

        interval_percent_map.insert(interval_minutes, changes);
    }

    interval_percent_map
}

fn exchange_allowed(user: &UserSettings, exchange: &str) -> bool {
    match exchange {
        "binance" => user.binance,
        "bybit" => user.bybit,
        "bingx" => user.bingx,
        _ => false,
    }
}

pub async fn send_tg_message(bot: &Bot, user_id: i64, text: &str) { // TODO: отправлять все сообщеиня только через эту функцию !
    info!("Отправка сообщения пользователю {}: {}", user_id, text);

    match bot.send_message(ChatId(user_id), text).send().await {
        Ok(response) => {
            info!(
                "Сообщение пользователю {} успешно отправлено. Telegram message_id={}",
                user_id,
                response.id
            );
        }

        Err(err) => {
            let err_str = err.to_string();

            if err_str.contains("429") {
                error!(
                    "Ошибка 429 (Too Many Requests) при отправке сообщения пользователю {}. \
                        Telegram просит замедлиться.",
                    user_id
                );
            } else {
                error!(
                    "Ошибка отправки сообщения пользователю {}: {}",
                    user_id, err_str
                );
            }
        }
    }
}

pub async fn check_interval_for_user(
    pool: &PgPool,
    bot: &Bot,
    user: &UserSettings,
    interval: i32,
    threshold_percent: f64,
    notification_type: &str,
    interval_percent_map: &HashMap<i32, Vec<PriceChange>>
) -> bool {
    let Some(changes) = interval_percent_map.get(&interval) else { return false; };

    for change in changes {
        if !exchange_allowed(&user, &change.exchange) {
            continue;
        }

        if notification_type == "drop" {
            if change.change_percent > threshold_percent {
                continue;
            }
        } else {
            if change.change_percent < threshold_percent {
                continue;
            }
        }

        if let Err(e) = db::insert_notification(pool, user.user_id, &change.currency, notification_type).await {
            // TODO: логировать то-что уведомление записалось тоже было бы хорошо
            error!("Ошибка записи уведомления: {}", e); // TODO: возможно для таких штук тоже нужно отдельнубю функцию враппер
        }

        let count_today = match db::count_notifications_for_user(
            pool,
            user.user_id,
            &change.currency,
            notification_type
        ).await {
            Ok(n) => n,
            Err(e) => {
                error!("Ошибка чтения кол-ва уведомлений: {}", e);
                0
            }
        };

        let msg = format!(
            "🏦{} • {}м • {}\n\
            {}: {:.2}%\n\
            🔔Сигнал за сутки: {}",
            change.exchange.to_uppercase(),
            interval,
            change.currency.to_uppercase(),
            match notification_type {
                "drop" => "🔻Падение",
                "short" => "🔺Памп",
                _ => "📈Рост",
            },
            change.change_percent,
            count_today
        );

        send_tg_message(&bot, user.user_id, &msg).await;

        return true;
    }
    
    false
}

pub async fn process_notifications(pool: &PgPool, bot: &Bot, max_interval_minutes: i32) {
    let rows = match db::get_prices_sort_created(pool, max_interval_minutes).await {
        Ok(r) => r,
        Err(e) => {
            error!("Не удалось получить цены: {}", e);
            return;
        }
    };

    let prices_map = get_grouped_prices(rows);

    let interval_percent_map = compute_all_procent_for_intevals(&prices_map, max_interval_minutes);

    let users = match db::get_user_settings(pool).await {
        Ok(u) => u,
        Err(e) => {
            error!("Не удалось загрузить user_settings: {}", e);
            return;
        }
    };

    for user in users {
        if let Some(end_date) = user.subscription_end_date {
            let now = chrono::Utc::now().naive_utc();
            if now > end_date {
                continue;
            }
        }

        // ------------------ LONG ------------------
        for interval in 1..=user.long_minutes {
            if check_interval_for_user(
                pool,
                bot,
                &user,
                interval,
                user.long_percent,
                "long",
                &interval_percent_map,
            ).await {
                break;
            }
        }

        // ------------------ SHORT ------------------
        for interval in 1..=user.short_minutes {
            if check_interval_for_user(
                pool,
                bot,
                &user,
                interval,
                user.short_percent,
                "short",
                &interval_percent_map,
            ).await {
                break;
            }
        }

        // ------------------ DROP ------------------
        for interval in 1..=user.drop_minutes {
            if check_interval_for_user(
                pool,
                bot,
                &user,
                interval,
                user.drop_percent,
                "drop",
                &interval_percent_map,
            ).await {
                break;
            }
        }
    }
}