-- Схема базы данных для замены txt-файлов

-- Таблица пользователей и их доступа
CREATE TABLE IF NOT EXISTS user_access (
    user_id BIGINT PRIMARY KEY,
    subscription_end_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Таблица настроек пользователей
CREATE TABLE IF NOT EXISTS user_settings (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    signal_type VARCHAR(10) NOT NULL, -- 'long', 'short', 'dump'
    setting_type VARCHAR(10) NOT NULL, -- 'procent', 'time'
    value DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE,
    UNIQUE(user_id, signal_type, setting_type)
);

-- Таблица включения/выключения бирж для пользователей
CREATE TABLE IF NOT EXISTS user_exchanges (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    exchange_name VARCHAR(20) NOT NULL, -- 'bybit', 'binance'
    is_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE,
    UNIQUE(user_id, exchange_name)
);

-- Таблица сигналов
CREATE TABLE IF NOT EXISTS signals (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    signal_date DATE NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    exchange_name VARCHAR(20) NOT NULL,
    signal_type VARCHAR(10) NOT NULL, -- 'long', 'short', 'dump'
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE
);

-- Таблица списка монет для отслеживания
CREATE TABLE IF NOT EXISTS tracked_coins (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    exchange_name VARCHAR(20) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(symbol, exchange_name)
);

-- Индексы для оптимизации запросов
CREATE INDEX IF NOT EXISTS idx_user_access_active ON user_access(is_active);
CREATE INDEX IF NOT EXISTS idx_user_access_end_date ON user_access(subscription_end_date);
CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id);
CREATE INDEX IF NOT EXISTS idx_user_exchanges_user_id ON user_exchanges(user_id);
CREATE INDEX IF NOT EXISTS idx_signals_user_date ON signals(user_id, signal_date);
CREATE INDEX IF NOT EXISTS idx_signals_symbol_date ON signals(symbol, signal_date);
CREATE INDEX IF NOT EXISTS idx_tracked_coins_active ON tracked_coins(is_active);
