CREATE TYPE user_role AS ENUM ('user', 'admin', 'no_access');

CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,
    username VARCHAR(50),
    role user_role NOT NULL DEFAULT 'user',
    subscription_end_date TIMESTAMP DEFAULT NULL
);

CREATE TABLE user_settings (
    user_id BIGINT PRIMARY KEY REFERENCES users(user_id),

    binance BOOLEAN DEFAULT FALSE,
    bybit BOOLEAN DEFAULT FALSE,
    bingx BOOLEAN DEFAULT FALSE,
    
    long_percent DOUBLE PRECISION DEFAULT 3.0,
    long_minutes INT DEFAULT 5,

    short_percent DOUBLE PRECISION DEFAULT 7.0,
    short_minutes INT DEFAULT 20,

    drop_percent DOUBLE PRECISION DEFAULT -7.0,
    drop_minutes INT DEFAULT 20
);

CREATE INDEX idx_user_settings_user ON user_settings (user_id);

CREATE TABLE currency_prices (
    id BIGSERIAL PRIMARY KEY,
    currency VARCHAR(50) NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    exchange VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_currency_prices_time ON currency_prices (created_at);

CREATE TYPE notification_enum AS ENUM (
    'long',
    'short',
    'drop'
);

CREATE TABLE user_notifications (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(user_id),
    currency VARCHAR(15) NOT NULL,
    notification_type notification_enum NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_user_notifications_user_type_date
ON user_notifications (user_id, notification_type, created_at);

CREATE TYPE user_state AS ENUM (
    'main_menu', 
    'add_user_menu', 
    'add_user_button', 
    'add_admin_button', 
    'delete_user_button',
    'choosing_exchange_menu',
    'choosing_parameters_menu',
    'long_period_button',
    'long_percent_button',
    'short_period_button',
    'short_percent_button',
    'drop_period_button',
    'drop_percent_button'
    );

CREATE TABLE user_states (
    user_id BIGINT PRIMARY KEY REFERENCES users(user_id),
    state user_state NOT NULL
);

INSERT INTO users (user_id, username, role)
VALUES (277739613, 'Petrov', 'admin')
ON CONFLICT DO NOTHING;

INSERT INTO user_settings (user_id, long_percent, long_minutes)
VALUES (277739613, 1, 1)
ON CONFLICT DO NOTHING;