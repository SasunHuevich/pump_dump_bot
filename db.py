import psycopg2
from psycopg2.extras import DictCursor
from datetime import datetime, date
import logging
from dotenv import load_dotenv
import os

load_dotenv()   

# Настройка логирования
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class DatabaseManager:
    def __init__(self):
        self.conn = psycopg2.connect(
            database=os.getenv("DB_NAME"),
            user=os.getenv("DB_USER"),
            password=os.getenv("DB_PASSWORD"),
            host=os.getenv("DB_HOST")
        )
        self.conn.autocommit = True
        # Гарантируем наличие необходимых таблиц/индексов
        self._ensure_schema()

    def _ensure_schema(self):
        """Создаёт недостающие таблицы и индексы. Безопасно вызывать многократно."""
        ddl = """
        -- Новая объединённая таблица пользователей и их настроек/доступа/бирж
        CREATE TABLE IF NOT EXISTS "user" (
            user_id BIGINT PRIMARY KEY,
            subscription_end_date DATE,
            short_time DECIMAL(10,2) DEFAULT 20,
            short_procent DECIMAL(10,2) DEFAULT 7,
            long_time DECIMAL(10,2) DEFAULT 5,
            long_procent DECIMAL(10,2) DEFAULT 3,
            dump_time DECIMAL(10,2) DEFAULT 20,
            dump_procent DECIMAL(10,2) DEFAULT 7,
            binance BOOLEAN DEFAULT TRUE,
            bybit BOOLEAN DEFAULT TRUE
        );

        CREATE INDEX IF NOT EXISTS idx_user_subscription_end_date ON "user"(subscription_end_date);

        -- Таблица сигналов
        CREATE TABLE IF NOT EXISTS signals (
            id SERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL,
            coin VARCHAR(20) NOT NULL,
            market VARCHAR(20) NOT NULL,
            signal_type VARCHAR(10) NOT NULL,
            timestamp BIGINT NOT NULL,
            CONSTRAINT signals_user_fk FOREIGN KEY (user_id) REFERENCES "user"(user_id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_signals_user_coin_market ON signals(user_id, coin, market);
        CREATE INDEX IF NOT EXISTS idx_signals_coin_market ON signals(coin, market);

        -- Таблица списка монет для отслеживания
        CREATE TABLE IF NOT EXISTS tracked_coins (
            id SERIAL PRIMARY KEY,
            coin VARCHAR(20) NOT NULL,
            market VARCHAR(20) NOT NULL,
            is_active BOOLEAN DEFAULT TRUE,
            UNIQUE(coin, market)
        );

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
        """Получает информацию о доступе пользователя из новой таблицы user."""
        sql = """
        SELECT user_id, subscription_end_date
        FROM "user"
        WHERE user_id = %s
        """
        result = self.query(sql, (user_id,), fetch=True)
        return result[0] if result else None
    
    def check_user_access(self, user_id):
        """Проверяет, есть ли у пользователя активный доступ"""
        access = self.get_user_access(user_id)
        if not access:
            return False
        end_date = access.get('subscription_end_date')
        if end_date is None:
            return False
        today = date.today()
        return end_date > today
    
    def add_user_access(self, user_id, end_date, is_active=True):
        """Добавляет или обновляет доступ пользователя (user.subscription_end_date)."""
        sql = """
        INSERT INTO "user" (user_id, subscription_end_date)
        VALUES (%s, %s)
        ON CONFLICT (user_id)
        DO UPDATE SET 
            subscription_end_date = EXCLUDED.subscription_end_date
        """
        self.query(sql, (user_id, end_date))
    
    def revoke_user_access(self, user_id):
        """Отзывает доступ пользователя: ставим subscription_end_date = CURRENT_DATE."""
        sql = """
        UPDATE "user"
        SET subscription_end_date = CURRENT_DATE
        WHERE user_id = %s
        """
        self.query(sql, (user_id,))
    
    def get_user_settings(self, user_id):
        """Получает настройки пользователя из таблицы user в старом формате.

        Возвращает dict: {'long': [procent, time], 'short': [...], 'dump': [...]} как строки.
        """
        sql = """
        SELECT short_time, short_procent,
               long_time, long_procent,
               dump_time, dump_procent
        FROM "user"
        WHERE user_id = %s
        """
        row = self.query(sql, (user_id,), fetch=True)

        # Значения по умолчанию для обратной совместимости
        short_time = 20
        short_procent = 7
        long_time = 5
        long_procent = 3
        dump_time = 20
        dump_procent = 7

        if row:
            r = row[0]
            short_time = r['short_time'] if r['short_time'] is not None else short_time
            short_procent = r['short_procent'] if r['short_procent'] is not None else short_procent
            long_time = r['long_time'] if r['long_time'] is not None else long_time
            long_procent = r['long_procent'] if r['long_procent'] is not None else long_procent
            dump_time = r['dump_time'] if r['dump_time'] is not None else dump_time
            dump_procent = r['dump_procent'] if r['dump_procent'] is not None else dump_procent

        return {
            'long': [str(long_procent), str(long_time)],
            'short': [str(short_procent), str(short_time)],
            'dump': [str(dump_procent), str(dump_time)],
        }
    
    def update_user_setting(self, user_id, signal_type, setting_type, value):
        """Обновляет настройку пользователя в таблице user."""
        col_map = {
            ('short', 'time'): 'short_time',
            ('short', 'procent'): 'short_procent',
            ('long', 'time'): 'long_time',
            ('long', 'procent'): 'long_procent',
            ('dump', 'time'): 'dump_time',
            ('dump', 'procent'): 'dump_procent',
        }
        column = col_map.get((signal_type, setting_type))
        if not column:
            raise ValueError('Unsupported setting requested')

        # Обеспечим существование строки, затем обновим колонку
        sql_insert = """
        INSERT INTO "user" (user_id)
        VALUES (%s)
        ON CONFLICT (user_id) DO NOTHING
        """
        self.query(sql_insert, (user_id,))

        sql_update = f"""
        UPDATE "user"
        SET {column} = %s
        WHERE user_id = %s
        """
        self.query(sql_update, (value, user_id))
    
    def get_user_exchanges(self, user_id):
        """Получает настройки бирж для пользователя из таблицы user."""
        sql = """
        SELECT binance, bybit
        FROM "user"
        WHERE user_id = %s
        """
        result = self.query(sql, (user_id,), fetch=True)
        if not result:
            return {'bybit': '+', 'binance': '+'}
        row = result[0]
        return {
            'binance': '+' if row['binance'] else '-',
            'bybit': '+' if row['bybit'] else '-',
        }
    
    def update_user_exchange(self, user_id, exchange_name, is_enabled):
        """Обновляет настройку биржи в таблице user (binance/bybit)."""
        if exchange_name not in ('binance', 'bybit'):
            raise ValueError('exchange_name must be "binance" or "bybit"')

        sql_insert = """
        INSERT INTO "user" (user_id)
        VALUES (%s)
        ON CONFLICT (user_id) DO NOTHING
        """
        self.query(sql_insert, (user_id,))

        sql_update = f"""
        UPDATE "user"
        SET {exchange_name} = %s
        WHERE user_id = %s
        """
        self.query(sql_update, (is_enabled, user_id))
    
    def get_signals_count(self, user_id, coin, market, ts_from=None, ts_to=None, signal_type=None):
        """Получает количество сигналов для пользователя/монеты/маркета за интервал по timestamp (UNIX, ms). По умолчанию: все сигналы пользователя по coin/market."""
        conditions = ["user_id = %s", "coin = %s", "market = %s"]
        params = [user_id, coin, market]
        if ts_from is not None:
            conditions.append("timestamp >= %s")
            params.append(ts_from)
        if ts_to is not None:
            conditions.append("timestamp < %s")
            params.append(ts_to)
        if signal_type is not None:
            conditions.append("signal_type = %s")
            params.append(signal_type)
        sql = f"""
        SELECT COUNT(*) as count 
        FROM signals 
        WHERE {' AND '.join(conditions)}
        """
        result = self.query(sql, tuple(params), fetch=True)
        return result[0]['count'] if result else 0
    
    def add_signal(self, user_id, coin, market, signal_type, timestamp):
        """Добавляет новый сигнал. timestamp — unix time ms/сек в зависимости от стандарта в коде."""
        sql = """
        INSERT INTO signals (user_id, coin, market, signal_type, timestamp)
        VALUES (%s, %s, %s, %s, %s)
        """
        self.query(sql, (user_id, coin, market, signal_type, timestamp))
    
    def get_tracked_coins(self, market):
        """Получает список отслеживаемых монет для маркета (теперь coin/market)"""
        sql = """
        SELECT coin
        FROM tracked_coins
        WHERE market = %s AND is_active = TRUE
        ORDER BY coin
        """
        results = self.query(sql, (market,), fetch=True)
        return [row['coin'] for row in results]
    
    def add_tracked_coin(self, coin, market):
        """Добавляет монету для отслеживания (coin/market)"""
        sql = """
        INSERT INTO tracked_coins (coin, market)
        VALUES (%s, %s)
        ON CONFLICT (coin, market) DO NOTHING
        """
        self.query(sql, (coin, market))
    
    def get_all_active_users(self):
        """Получает всех пользователей с активным доступом из таблицы user."""
        sql = """
        SELECT user_id, subscription_end_date
        FROM "user"
        WHERE subscription_end_date IS NOT NULL AND subscription_end_date > CURRENT_DATE
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
