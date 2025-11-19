CREATE TYPE user_role AS ENUM ('user', 'admin');

CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,
    username VARCHAR(50),
    role user_role NOT NULL DEFAULT 'user',
    subscription_end_date TIMESTAMP
);

CREATE TABLE user_settings (
    user_id BIGINT PRIMARY KEY REFERENCES users(id),
    
    long_percent NUMERIC(5,2),
    long_minutes INT,

    short_percent NUMERIC(5,2),
    short_minutes INT,

    drop_percent NUMERIC(5,2),
    drop_minutes INT
);

CREATE TABLE currency_prices (
    id BIGSERIAL PRIMARY KEY,
    currency VARCHAR(15) NOT NULL,
    price NUMERIC(18,8) NOT NULL,
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
    user_id BIGINT NOT NULL REFERENCES users(id),
    currency VARCHAR(15) NOT NULL,
    notification_type notification_enum NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_user_notifications_user_type_date
ON user_notification (user_id, notification_type, created_at);