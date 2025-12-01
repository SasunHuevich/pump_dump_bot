use teloxide::prelude::*;
use sqlx::PgPool;

use crate::handlers_ext::choosing_parameters_buttons::misc;

pub async fn handle(pool: &PgPool, bot: &Bot, msg: Message, user_id: i64)  -> ResponseResult<()> {
   if let Some(text) = msg.text() {
        match text {
            "✖️ Назад" => {
                // TODO: вообще по логике user_id модно из msg извлекать нахуя я его передаю
                misc::back_to_choosing_parameters_menu(bot, pool, msg.chat.id, user_id).await?;
            }
            _ => {
                misc::update_percent_setting(bot, pool, msg.chat.id, user_id, text, "drop_percent").await?;
            }
        }
    }

    Ok(())
}