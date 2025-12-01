use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::misc;
use crate::db::{UserState, UserRole};

pub async fn handle(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64, user_role: UserRole)  -> ResponseResult<()> {
    if let Some(text) = msg.text() {
    let exchanges = [("Binance", "binance"), ("Bybit", "bybit"), ("Bingx", "bingx")];
    let mut matched = false;

    for (label, db_name) in &exchanges {
        if text == format!("{label}: ✅") || text == format!("{label}: ❌") {
            matched = true;

            if let Err(e) = db::switch_exchange_state(pool, user_id, db_name).await {
                bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
                return Ok(());
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

            break;
        }
    }

    if !matched {
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
            other => {
                bot.send_message(msg.chat.id, format!("Не понял команду 😕: {}", other)).await?;
            }
        }
    }
}


    Ok(())
}