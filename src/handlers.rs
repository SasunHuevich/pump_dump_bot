use teloxide::prelude::*;
use sqlx::PgPool;

use crate::buttons;
use crate::db;
use crate::db::{UserRole, UserState};
use crate::texts::*;
use crate::misc;
use crate::handlers_ext::{main_menu, add_user_menu, add_user_button, add_admin_button};

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
    }
    
    Ok(())
}