from pybit.unified_trading import HTTP
import telebot
import time
from datetime import datetime
from db import DatabaseManager
import logging

# Настройка логирования
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

bot = telebot.TeleBot('6653096426:AAFS69aAx7gG8JPew5HtjSIPdqUSsQB6bi4', threaded=False) 

# Создаем экземпляр базы данных
db = DatabaseManager()

# Загружаем отслеживаемые монеты из базы данных
all_coins = db.get_tracked_coins('bybit')

def have_dostup(user_id):
    """Проверяет, есть ли у пользователя активный доступ"""
    return db.check_user_access(user_id)
    
def bybit_vkl(user_id):
    """Проверяет, включена ли биржа ByBit для пользователя"""
    exchanges = db.get_user_exchanges(user_id)
    return exchanges.get('bybit', '+') == '+'

def prover_dubli(proshlie, user, sym, regim, vremya, prom):
    """Проверяет, был ли уже сигнал для данного пользователя/символа/режима."""
    try:
        recent = [i[0] for i in prom[:vremya]]
        last_time = proshlie.get(user, {}).get(regim, {}).get(sym)
        return last_time not in recent
    except Exception:
        return True

proshlie = {}

def process_symbol(sym, dostupi, proshlie):
    """Обрабатывает один символ для всех пользователей"""
    time_now = int(str(time.time()).split('.')[0] + '000')
    time_prev = time_now - (60 * 150 * 1000)
    
    try:
        session = HTTP(
            testnet=False,
            api_key="guYItls8Ai4eqpRFW8",
            api_secret="3uLS0LBZ3GoEz9h8uoRTncVVWxsta7ty6mwU",
        )
        
        prom = session.get_kline(
                category="linear",
                symbol=sym,
                interval='1',
                start=time_prev,
                end=time_now,
            )
        
        if not prom.get('result') or not prom['result'].get('list'):
            logger.warning(f"ByBit: Нет данных для символа {sym}")
            return
        
        kline_data = prom['result']['list']
        if len(kline_data) < 2:
            logger.warning(f"ByBit: Недостаточно данных для символа {sym}")
            return
        
        for user_id in dostupi:
            if not have_dostup(user_id) or not bybit_vkl(user_id):
                continue
                
            try:
                settings = db.get_user_settings(user_id)
                
                for regim in settings:
                    try:
                        period_minutes = int(float(settings[regim][1]))
                        if period_minutes > len(kline_data):
                            logger.warning(f"ByBit: Недостаточно данных для периода {period_minutes} мин для {sym}")
                            continue
                            
                        current_price = float(kline_data[0][2])  # Цена закрытия текущей свечи
                        past_price = float(kline_data[period_minutes - 1][3])  # Цена открытия прошлой свечи
                        
                        if past_price == 0:
                            logger.warning(f"ByBit: Нулевая цена для {sym}")
                            continue
                            
                        razn = ((current_price - past_price) / past_price) * 100
                        
                        # Проверяем условия для каждого типа сигнала
                        signal_sent = False

                        condition = \
                            (regim == 'long' and razn > float(settings[regim][0])) or \
                            (regim == 'short' and razn > float(settings[regim][0])) or \
                            (regim == 'dump' and razn < -float(settings[regim][0]))
                        
                        if condition:
                            if prover_dubli(proshlie, user_id, sym, regim, period_minutes, kline_data):
                                signal_sent = send_signal(user_id, sym, regim, razn, settings[regim][1], 
                                                        past_price, current_price, 'bybit', proshlie, kline_data[0][0])

                        if signal_sent:
                            time.sleep(0.1)  # Небольшая пауза между сообщениями
                            
                    except ValueError as e:
                        logger.error(f"ByBit: Ошибка обработки данных для {sym}, пользователь {user_id}, режим {regim}: {e}")
                        continue
                    except IndexError as e:
                        logger.error(f"ByBit: Ошибка индекса для {sym}, пользователь {user_id}, режим {regim}: {e}")
                        continue
                        
            except Exception as e:
                logger.error(f"ByBit: Ошибка получения настроек для пользователя {user_id}: {e}")
                continue
                
    except Exception as e:
        error_str = str(e)
        if "Invalid symbol" in error_str or "symbol not found" in error_str.lower():
            logger.warning(f"ByBit: Символ {sym} неактивен или удален с биржи")
            # Можно добавить автоматическую деактивацию символа
            # db.query("UPDATE tracked_coins SET is_active = FALSE WHERE symbol = %s AND exchange_name = 'bybit'", (sym,))
        else:
            logger.error(f"ByBit: Ошибка API для символа {sym}: {e}")
        return

def send_signal(user_id, sym, regim, razn, period, past_price, current_price, exchange, proshlie, timestamp):
    """Отправляет сигнал пользователю"""
    try:
        tekkkk = datetime.now().strftime("%Y-%m-%d")
        signals_count = db.get_signals_count(user_id, tekkkk, sym, exchange)
        
        # Определяем тип сигнала и эмодзи
        if regim == 'long':
            signal_type = "📈Рост"
        elif regim == 'short':
            signal_type = "🔺Памп"
        else:  # dump
            signal_type = "🔻Падение"
        
        mess = f'🏦{exchange.upper()} • {period}м • {sym}\n{signal_type}: {round(razn, 2)}% ({past_price}-{current_price})\n🔔Сигнал за сутки: {signals_count + 1}'
        
        # Добавляем сигнал в базу данных
        db.add_signal(user_id, tekkkk, sym, exchange, regim, timestamp)
        
        # Обновляем историю сигналов
        if user_id not in proshlie:
            proshlie[user_id] = {'long': {}, 'short': {}, 'dump': {}}
        proshlie[user_id][regim][sym] = timestamp
        
        # Отправляем сообщение
        bot.send_message(int(user_id), mess)
        logger.info(f"ByBit: Отправлен сигнал {regim} для {sym} пользователю {user_id}")
        return True
        
    except Exception as e:
        logger.error(f"ByBit: Ошибка отправки сигнала для {sym} пользователю {user_id}: {e}")
        return False

while True:
    try:
        logger.info('ByBit: Запуск сканирования...')
        
        # Получаем всех пользователей с активным доступом
        dostupi = db.get_all_active_users()
        
        if not dostupi:
            logger.info("ByBit: Нет активных пользователей")
            time.sleep(60)  # Ждем минуту если нет пользователей
            continue
        
        logger.info(f"ByBit: Обрабатываем {len(all_coins)} символов для {len(dostupi)} пользователей")
        
        for sym in all_coins:
            process_symbol(sym, dostupi, proshlie)
            
        logger.info('ByBit: Сканирование завершено')
        time.sleep(60)  # Пауза между циклами
        
    except KeyboardInterrupt:
        logger.info("ByBit: Получен сигнал остановки")
        break
    except Exception as e:
        logger.error(f"ByBit: Критическая ошибка в основном цикле: {e}")
        time.sleep(60)  # Пауза перед повторной попыткой

# Закрываем соединение с базой данных только при завершении программы
try:
    db.close()
    logger.info("ByBit: Соединение с базой данных закрыто")
except:
    pass