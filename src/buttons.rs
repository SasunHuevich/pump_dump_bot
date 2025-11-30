use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::db;
use crate::db::{EnableExchanges};

pub fn main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("🏦 Выбор биржи"),
            KeyboardButton::new("⚒️ Настроить"),
        ],
        vec![
            KeyboardButton::new("🔐 Получить доступ"),
            KeyboardButton::new("💬 Поддержка"),
        ],
    ])
    .resize_keyboard()
}

pub fn admin_menu() -> KeyboardMarkup {
    let mut buttons = main_menu().keyboard.clone();
    
    buttons.push(vec![
        KeyboardButton::new("👥 Управление пользователями"),
    ]);

    KeyboardMarkup::new(buttons).resize_keyboard()
}

pub fn add_user_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("➕ Пользователь"),
            KeyboardButton::new("➕ Админ"),
        ],
        vec![
            KeyboardButton::new("➖ Пользователь/Aдмин"),
        ],
        vec![
            KeyboardButton::new("✖️ Назад"),
        ],
    ])
    .resize_keyboard()
}

pub fn back_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("✖️ Назад"),
        ],
    ])
    .resize_keyboard()
}

pub fn exchange_menu(user_id: i64, enable_exchanges: db::EnableExchanges) -> KeyboardMarkup{
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(
                format!("Binance: {}", if enable_exchanges.binance { "✅" } else { "❌" })
            ),
            KeyboardButton::new(
                format!("Bybit: {}", if enable_exchanges.bybit { "✅" } else { "❌" })
            ),
            KeyboardButton::new(
                format!("Bingx: {}", if enable_exchanges.bingx { "✅" } else { "❌" })
            ),
        ],
        vec![
            KeyboardButton::new("✖️ Назад"),
        ],
    ])
    .resize_keyboard()
}

pub fn parameters_menu() -> KeyboardMarkup{
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(
                format!("📈 Период лонг")
            ),
            KeyboardButton::new(
                format!("% лонга")
            ),
        ],
        vec![
            KeyboardButton::new(
                format!("🔺 Период памп")
            ),
            KeyboardButton::new(
                format!("% пампа")
            ),
        ],
        vec![
            KeyboardButton::new(
                format!("🔻 Период падение")
            ),
            KeyboardButton::new(
                format!("% падения")
            ),
        ],
        vec![
            KeyboardButton::new("✖️ Назад"),
        ],
    ])
    .resize_keyboard()
}