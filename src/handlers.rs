use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::db::{UserRole, UserState};
use crate::texts::*;
use crate::misc;
use crate::handlers_ext::*;

pub async fn handle_message(pool: &PgPool, bot: &Bot, msg: Message)  -> ResponseResult<()>{
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

    if let Some(text) = msg.text() {
        if text == "/start" {
            let menu = misc::get_main_menu_by_role(&user_role);

            if let Err(e) = db::set_user_state(pool, user_id, &UserState::MainMenu).await {
                bot.send_message(msg.chat.id, format!("Ошибка БД: {}", e)).await?;
            }

            bot.send_message(msg.chat.id, START_MESSAGE)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(menu)
            .await?;

            return Ok(());
        }
    }

    match state {
        UserState::MainMenu => {
            main_menu::handle_main_menu(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::AddUserMenu => {
            add_user_menu::handle_add_user_menu(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::AddUserButton => {
            add_user_button::handle_add_user_button(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::AddAdminButton => {
            add_admin_button::handle_add_admin_button(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::DeleteUserButton => {
            delete_user_button::handle_delete_user_button(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::ChoosingExchangeMenu => {
            choosing_exchange_menu::handle_choosing_exchange_menu(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::ChoosingParametersMenu => {
            choosing_parameters_menu::handle_choosing_parameters_menu(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::LongPeriodButton => {
            choosing_parameters_buttons::period_long_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::LongPercentButton => {
            choosing_parameters_buttons::percent_long_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::ShortPeriodButton => {
            choosing_parameters_buttons::period_short_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::ShortPercentButton => {
            choosing_parameters_buttons::percent_short_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::DropPeriodButton => {
            choosing_parameters_buttons::period_drop_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
        UserState::DropPercentButton => {
            choosing_parameters_buttons::percent_drop_button::handle(pool, bot, msg, user_id, user_role).await?;
        }
    }
    
    Ok(())
}