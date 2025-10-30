from pybit.unified_trading import HTTP
import telebot
import time
from datetime import datetime
from binance.um_futures import UMFutures
from db import DatabaseManager
import logging
from dotenv import load_dotenv
import os

load_dotenv()

# Настройка логирования
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

bot = telebot.TeleBot(os.getenv("BOT_TOKEN"), threaded=False)

# Создаем экземпляр базы данных
db = DatabaseManager()

# Загружаем отслеживаемые монеты из базы данных
all_coins = db.get_tracked_coins('binance')

def prover_dubli(proshlie, user, sym, regim, vremya, prom):
    """Проверяет, был ли уже сигнал для данного пользователя/символа/режима."""
    try:
        recent = [i[0] for i in prom[:vremya]]
        last_time = proshlie.get(user, {}).get(regim, {}).get(sym)
        return last_time not in recent
    except Exception:
        return True

    
proshlie={}


def process_coin_binance(coin, dostupi, proshlie, cm_futures_client):
    """Обрабатывает один символ для всех пользователей через Binance API"""
    try:
        prom = cm_futures_client.klines(coin, "1m")
        prom.reverse()
        
        if not prom or len(prom) < 2:
            logger.warning(f"Binance: Нет данных для символа {coin}")
            return
        
        for user_id in dostupi:
            if not db.check_user_access(user_id) or not db.get_user_exchanges(user_id).get('binance', '+') == '+':
                continue
                
            try:
                settings = db.get_user_settings(user_id)
                
                for regim in settings:
                    try:
                        period_minutes = int(float(settings[regim][1]))
                        if period_minutes > len(prom):
                            logger.warning(f"Binance: Недостаточно данных для периода {period_minutes} мин для {coin}")
                            continue
                            
                        current_price = float(prom[0][2])  # Цена закрытия текущей свечи
                        past_price = float(prom[period_minutes - 1][3])  # Цена открытия прошлой свечи
                        
                        if past_price == 0:
                            logger.warning(f"Binance: Нулевая цена для {coin}")
                            continue
                            
                        razn = ((current_price - past_price) / past_price) * 100
                        
                        # Проверяем условия для каждого типа сигнала
                        signal_sent = False
                        
                        condition = \
                            (regim == 'long' and razn > float(settings[regim][0])) or \
                            (regim == 'short' and razn > float(settings[regim][0])) or \
                            (regim == 'dump' and razn < -float(settings[regim][0]))
                        if condition:
                            if prover_dubli(proshlie, user_id, coin, regim, period_minutes, prom):
                                signal_sent = send_signal_binance(user_id, coin, regim, razn, settings[regim][1], 
                                                               past_price, current_price, 'binance', proshlie, prom[0][0])
                        
                        if signal_sent:
                            time.sleep(0.1)  # Небольшая пауза между сообщениями
                            
                    except ValueError as e:
                        logger.error(f"Binance: Ошибка обработки данных для {coin}, пользователь {user_id}, режим {regim}: {e}")
                        continue
                    except IndexError as e:
                        logger.error(f"Binance: Ошибка индекса для {coin}, пользователь {user_id}, режим {regim}: {e}")
                        continue
                        
            except Exception as e:
                logger.error(f"Binance: Ошибка получения настроек для пользователя {user_id}: {e}")
                continue
                
    except Exception as e:
        error_str = str(e)
        if "Invalid symbol status" in error_str or "-1122" in error_str:
            logger.warning(f"Binance: Символ {coin} неактивен или удален с биржи")
            # Можно добавить автоматическую деактивацию символа
            # db.query("UPDATE tracked_coins SET is_active = FALSE WHERE symbol = %s AND exchange_name = 'binance'", (coin,))
        else:
            logger.error(f"Binance: Ошибка API для символа {coin}: {e}")
        return

def send_signal_binance(user_id, coin, regim, razn, period, past_price, current_price, market, proshlie, timestamp):
    """Отправляет сигнал пользователю"""
    try:
        tekkkk = datetime.now().strftime("%Y-%m-%d")
        signals_count = db.get_signals_count(user_id, tekkkk, coin, market)
        
        # Определяем тип сигнала и эмодзи
        if regim == 'long':
            signal_type = "📈Рост"
        elif regim == 'short':
            signal_type = "🔺Памп"
        else:  # dump
            signal_type = "🔻Падение"
        
        mess = f'🏦{market.upper()} • {period}м • {coin}\n{signal_type}: {round(razn, 2)}% ({past_price}-{current_price})\n🔔Сигнал за сутки: {signals_count + 1}'
        
        # Добавляем сигнал в базу данных
        db.add_signal(user_id, coin, market, regim, timestamp)
        
        # Обновляем историю сигналов
        if user_id not in proshlie:
            proshlie[user_id] = {'long': {}, 'short': {}, 'dump': {}}
        proshlie[user_id][regim][coin] = timestamp
        
        # Отправляем сообщение
        bot.send_message(int(user_id), mess)
        logger.info(f"Binance: Отправлен сигнал {regim} для {coin} пользователю {user_id}")
        return True
        
    except Exception as e:
        logger.error(f"Binance: Ошибка отправки сигнала для {coin} пользователю {user_id}: {e}")
        return False

while True:
    try:
        logger.info('Binance: Запуск сканирования...')
        
        # Инициализируем клиент Binance
        cm_futures_client = UMFutures()
        cm_futures_client = UMFutures(
            key=os.getenv("BINANCE_API_KEY"), 
            secret=os.getenv("BINANCE_API_SECRET"),
        )
        
        # Получаем всех пользователей с активным доступом
        dostupi = db.get_all_active_users()
        
        if not dostupi:
            logger.info("Binance: Нет активных пользователей")
            time.sleep(60)  # Ждем минуту если нет пользователей
            continue
        
        logger.info(f"Binance: Обрабатываем {len(all_coins)} символов для {len(dostupi)} пользователей")
        
        for coin in all_coins:
            process_coin_binance(coin, dostupi, proshlie, cm_futures_client)
            
        logger.info('Binance: Сканирование завершено')
        time.sleep(60)  # Пауза между циклами
        
    except KeyboardInterrupt:
        logger.info("Binance: Получен сигнал остановки")
        break
    except Exception as e:
        logger.error(f"Binance: Критическая ошибка в основном цикле: {e}")
        time.sleep(60)  # Пауза перед повторной попыткой

# Закрываем соединение с базой данных только при завершении программы
try:
    db.close()
    logger.info("Binance: Соединение с базой данных закрыто")
except:
    pass
