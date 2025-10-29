# -*- coding: utf-8 -*-
import os
import telebot
import time
from telebot.types import ReplyKeyboardMarkup, KeyboardButton
from datetime import datetime
from db import DatabaseManager
from dotenv import load_dotenv

load_dotenv()

bot = telebot.TeleBot(os.getenv("BOT_TOKEN"), threaded=False)   

db = DatabaseManager()   

@bot.message_handler(commands=['start'])
def start_command(message):
    messs='👋🏻Привет.\nЭтот бот сканирует рынок на:\n\n📈Небольшой рост, для входа в ЛОНГ;\n🔺Памп (резкий рост), для входа в ШОРТ;\n🔻Дамп (резкое падения цены), для торговли на коррекцию падения.\n\nПример настроек:\n\n(Ссылка на видео)'
    msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

@bot.message_handler(commands=['addnew'])
def get_command(message): 
    messs='Выберите действие: '
    msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    
def add_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("Добавить пользователя")
    btn2 = KeyboardButton("Удалить пользователя")
    btn3 = KeyboardButton("✖️ Назад")
    markup.add(btn1,btn2)
    markup.add(btn3)
    return markup

def main_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("🏦 Выбор биржи")
    btn2 = KeyboardButton("⚒️ Настроить")
    btn3 = KeyboardButton("🔐 Получить доступ")
    btn4 = KeyboardButton("💬 Поддержка")
    markup.add(btn1,btn2)
    markup.add(btn3,btn4)
    return markup

def nazad_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("<< Назад")
    markup.add(btn1)
    return markup

def nastroyki_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("📈 Период лонг")
    btn2 = KeyboardButton("% роста")
    btn3 = KeyboardButton("🔺 Период памп")
    btn4 = KeyboardButton("% пампа")
    btn5 = KeyboardButton("🔻 Период падение")
    btn6 = KeyboardButton("% падения")
    btn7 = KeyboardButton("✖️ Назад")
    markup.add(btn1,btn2)
    markup.add(btn3,btn4)
    markup.add(btn5,btn6)
    markup.add(btn7)
    return markup

def profil_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("💬 Поддержка")
    btn2 = KeyboardButton("✖️ Назад")
    markup.add(btn1,btn2)
    return markup

def izmen_menu():
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    btn1 = KeyboardButton("✖️ Отмена")
    markup.add(btn1)
    return markup
    
def birgi(message):
    """Показывает настройки бирж для пользователя"""
    exchanges = db.get_user_exchanges(message.chat.id)
    
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    if(exchanges['binance']=='+'):
        btn1 = KeyboardButton("✅ Выключить Binance")
    else:
        btn1 = KeyboardButton("⬜️ Включить Binance")
    if(exchanges['bybit']=='+'):
        btn2 = KeyboardButton("✅ Выключить ByBit")
    else:
        btn2 = KeyboardButton("⬜️ Включить ByBit")
    btn3 = KeyboardButton("✖️ Назад")
    markup.add(btn1,btn2,btn3)
    messs='Включить / выключить сигналы с бирж:\n\n✅ – биржа ВКЛЮЧЕНА\n⬜️ - биржа ВЫКЛЮЧЕНА'
    msg = bot.send_message(message.chat.id, messs,reply_markup=markup)
    
def nastroiki(message):
    """Показывает текущие настройки пользователя"""
    settings = db.get_user_settings(message.chat.id)
    
    messs='✅Текущие настройки:\n\n📈 Лонг: '+settings['long'][0]+'% за '+settings['long'][1]+' мин.\n🔺 Шорт: '+settings['short'][0]+'% за '+settings['short'][1]+' мин.\n🔻 Падение: -'+settings['dump'][0]+'% за '+settings['dump'][1]+' мин.'
    msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())

def profil(message):
    """Показывает профиль пользователя"""
    access = db.get_user_access(message.chat.id)
    
    if access and db.check_user_access(message.chat.id):
        messs=f'Профиль пользователя {message.chat.id}\nПодписка действительна до {access["subscription_end_date"]}\nПо вопросам продления обратитесь к @lexcrypto1'
    else:
        messs=f'Профиль пользователя {message.chat.id}\n🚨 Доступ к моим функциям закрыт.\nПо вопросам продления обратитесь к @lexcrypto1'
    
    msg = bot.send_message(message.chat.id, messs,reply_markup=profil_menu())

def dostup(message):
    messs='Для поддержки обратитесь в чат @lexcrypto1'
    msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def submit_cancel_add_menu(message):
    bot.clear_step_handler_by_chat_id(message.chat.id)
    messs='❌ Ввод отменен'
    msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def send_id_add(message):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=int(message.text.strip())
            
            messs='Введите дату окончания подписки для пользователя '+str(znach)
            msg = bot.send_message(message.chat.id, messs,reply_markup=nazad_menu())
            bot.register_next_step_handler(msg, send_data_add,str(znach))
        else:
            submit_cancel_add_menu(message)
    except Exception:
        messs='Ошибка ввода. Укажите целое число ID.'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    
def send_data_add(message,data):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=message.text.strip()
            db.add_user_access(int(data), znach, True)
            messs=f'Пользователь {data} добавлен до {znach}.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
            
        else:
            submit_cancel_add_menu(message)
    except Exception:
        messs='Ошибка при сохранении данных.'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())

def send_id_del(message):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=int(message.text.strip())
            db.revoke_user_access(znach)
            messs=f'Пользователь {znach} удалён.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
        else:
            submit_cancel_add_menu(message)
    except Exception:
        messs='Ошибка ввода. Укажите целое число ID.'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())

def _handle_int_setting(message, kind, field, success_template):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            value = int(message.text.strip())
            db.update_user_setting(message.chat.id, kind, field, value)
            messs = success_template.format(value=value)
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            submit_cancel_add_menu(message)
    except Exception:
        messs='Ошибка ввода. Укажите целое число.'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def _handle_float_setting(message, kind, field, success_template):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            value = float(message.text.strip().replace(',','.'))
            db.update_user_setting(message.chat.id, kind, field, value)
            messs = success_template.format(value=value)
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except Exception:
        messs='Ошибка ввода. Укажите число (например, 1.5).'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def period_time(message, kind):
    _handle_int_setting(
        message,
        kind=kind,
        field='time',
        success_template='✔️ Период времени сохранён {value} мин.'
    )

def procent_value(message, kind):
    _handle_float_setting(
        message,
        kind=kind,
        field='procent',
        success_template='✔️ Процент изменения цены сохранён {value} %.'
    )

@bot.message_handler(content_types=['text'])
def func(message):
    def _send(chat_id, text, markup=None):
        if markup is None:
            markup = main_menu()
        return bot.send_message(chat_id, text, reply_markup=markup)

    def _toggle_handler(exchange_key, display_name, enabled):
        def _inner(msg):
            db.update_user_exchange(msg.chat.id, exchange_key, enabled)
            status = 'включены' if enabled else 'выключены'
            bot.send_message(msg.chat.id, f'Сигналы с биржи {display_name} {status}')
            birgi(msg)
        return _inner

    def _prompt_period(kind, human_label, next_handler):
        def _inner(msg):
            settings = db.get_user_settings(msg.chat.id)
            current = settings[kind][1]
            text = f'🕑 Текущий период времени, за который должен(о) произойти {human_label} – {current} мин\n\nВведи новый период времени: от 1 до 30 минут'
            m = _send(msg.chat.id, text, izmen_menu())
            bot.register_next_step_handler(m, next_handler, kind)
        return _inner

    def _prompt_percent(kind, human_label, next_handler):
        def _inner(msg):
            settings = db.get_user_settings(msg.chat.id)
            current = round(float(settings[kind][0]), 1)
            text = f'{human_label} {current}%\n\nВведи новый % изменения цены: от 0.5% до 100%'
            m = _send(msg.chat.id, text, izmen_menu())
            bot.register_next_step_handler(m, next_handler, kind)
        return _inner

    def _add_user_flow(msg):
        m = _send(msg.chat.id, 'Введите ID пользователя для добавления: ', nazad_menu())
        bot.register_next_step_handler(m, send_id_add)

    def _delete_user_flow(msg):
        dostupi = db.get_all_active_users()
        if(dostupi):
            messs = ''
            for user_id, end_date in dostupi.items():
                messs += f'<code>{user_id}</code>  {end_date}\n'
            bot.send_message(msg.chat.id, messs.strip(), parse_mode="HTML")
            m = _send(msg.chat.id, 'Введите ID пользователя для удаления: ', nazad_menu())
            bot.register_next_step_handler(m, send_id_del)
        else:
            bot.send_message(msg.chat.id, 'Нет активных пользователей', reply_markup=add_menu())

    def _cancel(msg):
        bot.clear_step_handler_by_chat_id(msg.chat.id)
        _send(msg.chat.id, '❌ Ввод отменен')

    handlers = {
        '🏦 Выбор биржи': birgi,
        '✅ Выключить Binance': _toggle_handler('binance', 'Binance', False),
        '⬜️ Включить Binance': _toggle_handler('binance', 'Binance', True),
        '✅ Выключить ByBit': _toggle_handler('bybit', 'ByBit', False),
        '⬜️ Включить ByBit': _toggle_handler('bybit', 'ByBit', True),
        '✖️ Назад': lambda msg: _send(msg.chat.id, '❌ Ввод отменен'),
        '✖️ Отмена': _cancel,
        '⚒️ Настроить': nastroiki,
        '💬 Поддержка': dostup,
        '🔐 Получить доступ': profil,
        '💌 Суппорт': lambda msg: _send(msg.chat.id, '🤖 Есть вопросы по моей работе? Отпиши @lexcrypto1', profil_menu()),
        '📈 Период лонг': _prompt_period('long', 'рост', period_time),
        '🔺 Период памп': _prompt_period('short', 'памп', period_time),
        '🔻 Период падение': _prompt_period('dump', 'падение', period_time),
        '% роста': _prompt_percent('long', '📈Текущий % изменения цены роста –', procent_value),
        '% пампа': _prompt_percent('short', '🔺Текущий % изменения цены пампа –', procent_value),
        '% падения': _prompt_percent('dump', '🔻 Текущий % изменения цены падения –', procent_value),
        'Добавить пользователя': _add_user_flow,
        'Удалить пользователя': _delete_user_flow,
    }

    handler = handlers.get(message.text)
    if handler:
        handler(message)
        
while(True):
    try:
        bot.infinity_polling(timeout=10, long_polling_timeout = 5)
    except Exception:
        print('Error!!!!')
        time.sleep(2)
