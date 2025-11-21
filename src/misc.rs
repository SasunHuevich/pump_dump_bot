use teloxide::types::{KeyboardMarkup};

use crate::db::{UserRole};
use crate::buttons;

pub fn get_main_menu_by_role(user_role: &UserRole) -> KeyboardMarkup {
    match user_role {
        UserRole::Admin => buttons::admin_menu(),
        UserRole::User => buttons::main_menu(),
        UserRole::NoAccess => unreachable!(),
    }
}