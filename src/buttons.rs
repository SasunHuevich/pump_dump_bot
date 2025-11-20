use teloxide::types::{KeyboardButton, KeyboardMarkup};

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
