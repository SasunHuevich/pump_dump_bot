import psycopg2
from psycopg2.extras import DictCursor
from datetime import datetime, date
import logging

# Настройка логирования
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class DatabaseManager:
    def __init__(self):
        self.conn = psycopg2.connect(
            database='botlextest2',
            user='postgres',
            password='tarkov-ass@',
            host='localhost'
        )
        self.conn.autocommit = True
        # Гарантируем наличие необходимых таблиц/индексов
        self._ensure_schema()

    def _ensure_schema(self):
        """Создаёт недостающие таблицы и индексы. Безопасно вызывать многократно."""
        ddl = """
        CREATE TABLE IF NOT EXISTS user_access (
            user_id BIGINT PRIMARY KEY,
            subscription_end_date DATE,
            is_active BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS user_settings (
            id SERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL,
            signal_type VARCHAR(10) NOT NULL,
            setting_type VARCHAR(10) NOT NULL,
            value DECIMAL(10,2) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id, signal_type, setting_type),
            FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE
        );

        -- Структура под код (по одному ряду на пользователя с полями binance/bybit)
        CREATE TABLE IF NOT EXISTS user_exchanges (
            user_id BIGINT PRIMARY KEY,
            binance BOOLEAN DEFAULT TRUE,
            bybit BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS signals (
            id SERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL,
            signal_date DATE NOT NULL,
            symbol VARCHAR(20) NOT NULL,
            exchange_name VARCHAR(20) NOT NULL,
            signal_type VARCHAR(10) NOT NULL,
            timestamp BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES user_access(user_id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tracked_coins (
            id SERIAL PRIMARY KEY,
            symbol VARCHAR(20) NOT NULL,
            exchange_name VARCHAR(20) NOT NULL,
            is_active BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(symbol, exchange_name)
        );

        -- Индексы
        CREATE INDEX IF NOT EXISTS idx_user_access_active ON user_access(is_active);
        CREATE INDEX IF NOT EXISTS idx_user_access_end_date ON user_access(subscription_end_date);
        CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id);
        CREATE INDEX IF NOT EXISTS idx_signals_user_date ON signals(user_id, signal_date);
        CREATE INDEX IF NOT EXISTS idx_signals_symbol_date ON signals(symbol, signal_date);
        CREATE INDEX IF NOT EXISTS idx_tracked_coins_active ON tracked_coins(is_active);
        """
        try:
            with self.conn.cursor() as cur:
                cur.execute(ddl)
        except Exception as e:
            logger.error(f"Failed to ensure DB schema: {e}")
            raise
    
    def query(self, sql, params=None, fetch=False):
        """Выполняет SQL запрос"""
        try:
            with self.conn.cursor(cursor_factory=DictCursor) as cur:
                cur.execute(sql, params)
                if fetch:
                    return cur.fetchall()
                return cur.rowcount
        except Exception as e:
            logger.error(f"Database query error: {e}")
            raise
    
    def get_user_access(self, user_id):
        """Получает информацию о доступе пользователя"""
        sql = """
        SELECT user_id, subscription_end_date, is_active 
        FROM user_access 
        WHERE user_id = %s AND is_active = TRUE
        ORDER BY created_at DESC 
        LIMIT 1
        """
        result = self.query(sql, (user_id,), fetch=True)
        return result[0] if result else None
    
    def check_user_access(self, user_id):
        """Проверяет, есть ли у пользователя активный доступ"""
        access = self.get_user_access(user_id)
        if not access:
            return False
        
        today = date.today()
        return access['subscription_end_date'] > today
    
    def add_user_access(self, user_id, end_date, is_active=True):
        """Добавляет или обновляет доступ пользователя"""
        sql = """
        INSERT INTO user_access (user_id, subscription_end_date, is_active)
        VALUES (%s, %s, %s)
        ON CONFLICT (user_id) 
        DO UPDATE SET 
            subscription_end_date = EXCLUDED.subscription_end_date,
            is_active = EXCLUDED.is_active,
            updated_at = CURRENT_TIMESTAMP
        """
        self.query(sql, (user_id, end_date, is_active))
    
    def revoke_user_access(self, user_id):
        """Отзывает доступ пользователя"""
        sql = "UPDATE user_access SET is_active = FALSE WHERE user_id = %s"
        self.query(sql, (user_id,))
    
    def get_user_settings(self, user_id):
        """Получает настройки пользователя"""
        sql = """
        SELECT signal_type, setting_type, value 
        FROM user_settings 
        WHERE user_id = %s
        """
        results = self.query(sql, (user_id,), fetch=True)
        
        # Преобразуем в формат, совместимый со старым кодом
        settings = {
            'long': ['3', '5'],
            'short': ['7', '20'], 
            'dump': ['7', '20']
        }
        
        for row in results:
            signal_type = row['signal_type']
            setting_type = row['setting_type']
            value = str(row['value'])
            
            if setting_type == 'procent':
                settings[signal_type][0] = value
            else:  # time
                settings[signal_type][1] = value
        
        return settings
    
    def update_user_setting(self, user_id, signal_type, setting_type, value):
        """Обновляет настройку пользователя"""
        sql = """
        INSERT INTO user_settings (user_id, signal_type, setting_type, value)
        VALUES (%s, %s, %s, %s)
        ON CONFLICT (user_id, signal_type, setting_type)
        DO UPDATE SET value = EXCLUDED.value
        """
        self.query(sql, (user_id, signal_type, setting_type, value))
    
    def get_user_exchanges(self, user_id):
        """Получает настройки бирж для пользователя (одна строка на пользователя)."""
        sql = """
        SELECT binance, bybit
        FROM user_exchanges
        WHERE user_id = %s
        LIMIT 1
        """
        result = self.query(sql, (user_id,), fetch=True)
        if not result:
            # По умолчанию включены обе биржи (для обратной совместимости)
            return {'bybit': '+', 'binance': '+'}
        row = result[0]
        return {
            'binance': '+' if row['binance'] else '-',
            'bybit': '+' if row['bybit'] else '-',
        }
    
    def update_user_exchange(self, user_id, exchange_name, is_enabled):
        """Обновляет настройку биржи для пользователя (одна строка на пользователя).
        exchange_name: 'binance' или 'bybit'
        """
        if exchange_name not in ('binance', 'bybit'):
            raise ValueError('exchange_name must be "binance" or "bybit"')
        # Подготовим значения для upsert
        binance_val = is_enabled if exchange_name == 'binance' else None
        bybit_val = is_enabled if exchange_name == 'bybit' else None

        # Вставляем строку с дефолтами, затем обновляем нужное поле
        sql_insert = """
        INSERT INTO user_exchanges (user_id)
        VALUES (%s)
        ON CONFLICT (user_id) DO NOTHING
        """
        self.query(sql_insert, (user_id,))

        sql_update = f"""
        UPDATE user_exchanges
        SET {exchange_name} = %s,
            updated_at = CURRENT_TIMESTAMP
        WHERE user_id = %s
        """
        self.query(sql_update, (is_enabled, user_id))
    
    def get_signals_count(self, user_id, signal_date, symbol, exchange_name):
        """Получает количество сигналов для пользователя за день"""
        sql = """
        SELECT COUNT(*) as count 
        FROM signals 
        WHERE user_id = %s AND signal_date = %s AND symbol = %s AND exchange_name = %s
        """
        result = self.query(sql, (user_id, signal_date, symbol, exchange_name), fetch=True)
        return result[0]['count'] if result else 0
    
    def add_signal(self, user_id, signal_date, symbol, exchange_name, signal_type, timestamp):
        """Добавляет новый сигнал"""
        sql = """
        INSERT INTO signals (user_id, signal_date, symbol, exchange_name, signal_type, timestamp)
        VALUES (%s, %s, %s, %s, %s, %s)
        """
        self.query(sql, (user_id, signal_date, symbol, exchange_name, signal_type, timestamp))
    
    def get_tracked_coins(self, exchange_name):
        """Получает список отслеживаемых монет для биржи"""
        sql = """
        SELECT symbol 
        FROM tracked_coins 
        WHERE exchange_name = %s AND is_active = TRUE
        ORDER BY symbol
        """
        results = self.query(sql, (exchange_name,), fetch=True)
        return [row['symbol'] for row in results]
    
    def add_tracked_coin(self, symbol, exchange_name):
        """Добавляет монету для отслеживания"""
        sql = """
        INSERT INTO tracked_coins (symbol, exchange_name)
        VALUES (%s, %s)
        ON CONFLICT (symbol, exchange_name) DO NOTHING
        """
        self.query(sql, (symbol, exchange_name))
    
    def get_all_active_users(self):
        """Получает всех пользователей с активным доступом"""
        sql = """
        SELECT user_id, subscription_end_date 
        FROM user_access 
        WHERE is_active = TRUE AND subscription_end_date > CURRENT_DATE
        """
        results = self.query(sql, fetch=True)
        return {row['user_id']: row['subscription_end_date'].strftime('%Y-%m-%d') for row in results}
    
    def close(self):
        """Закрывает соединение с базой данных"""
        if self.conn:
            self.conn.close()

# Создаем глобальный экземпляр для обратной совместимости
db = DatabaseManager()

# Функции для обратной совместимости
def query(sql, params=None, fetch=False):
    return db.query(sql, params, fetch)
