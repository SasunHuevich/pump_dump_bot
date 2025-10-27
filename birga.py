# -*- coding: utf-8 -*-
import telebot
import time
import os
from telebot.types import ReplyKeyboardRemove, \
    ReplyKeyboardMarkup, KeyboardButton, \
    InlineKeyboardMarkup, InlineKeyboardButton
from datetime import datetime
from dotenv import load_dotenv

load_dotenv()

bot = telebot.TeleBot(os.getenv("BOT_TOKEN"), threaded=False)   


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
    spis={}
    spis[str(message.chat.id)]={'bybit':'+','binance':'+'}
    file=open('vkl.txt', encoding='utf-8')
    for i in file:
        if(i.strip()!=''):
            pup=i.strip().split(';')
            if(pup[0] not in spis):
                spis[pup[0]]={'bybit':'+','binance':'+'}
            spis[pup[0]][pup[1]]=pup[2]
    file.close()
    markup = ReplyKeyboardMarkup(resize_keyboard=True)
    if(spis[str(message.chat.id)]['binance']=='+'):
        btn1 = KeyboardButton("✅ Выключить Binance")
    else:
        btn1 = KeyboardButton("⬜️ Включить Binance")
    if(spis[str(message.chat.id)]['bybit']=='+'):
        btn2 = KeyboardButton("✅ Выключить ByBit")
    else:
        btn2 = KeyboardButton("⬜️ Включить ByBit")
    btn3 = KeyboardButton("✖️ Назад")
    markup.add(btn1,btn2,btn3)
    messs='Включить / выключить сигналы с бирж:\n\n✅ – биржа ВКЛЮЧЕНА\n⬜️ - биржа ВЫКЛЮЧЕНА'
    msg = bot.send_message(message.chat.id, messs,reply_markup=markup)
    
def nastroiki(message):
    spis={}
    spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
    file=open('nastroiki.txt', encoding='utf-8')
    for i in file:
        pup=i.strip().split(';')
        if(pup[0]==str(message.chat.id)):
            if(pup[1]=='procent'):
                spis[str(message.chat.id)][pup[2]][0]=pup[3]
            else:
                spis[str(message.chat.id)][pup[2]][1]=pup[3]
    file.close()
    #messs='🤖 Я сканирую рынок на маленькие пампы, чтобы искать точки входа в ЛОНГ 🟢, большие пампы, чтобы искать точки входа в ШОРТ 🔴 и резкие просадки, чтобы торговать отскок 🔻 \n\nПримеры настроек: \n\n🟢 "Прислать лонговый сигнал, если монета выросла на 3% за 3 минуты"\n\n🔴  "Прислать шортовый сигнал, если монета выросла на 30% за 10 минут"\n\n🔻"Прислать сигнал о дампе, если монета упала на 10% за 15 минут"'
    #msg = bot.send_message(message.chat.id, messs)
    messs='✅Текущие настройки:\n\n📈 Лонг: '+spis[str(message.chat.id)]['long'][0]+'% за '+spis[str(message.chat.id)]['long'][1]+' мин.\n🔺 Шорт: '+spis[str(message.chat.id)]['short'][0]+'% за '+spis[str(message.chat.id)]['short'][1]+' мин.\n🔻 Падение: -'+spis[str(message.chat.id)]['dump'][0]+'% за '+spis[str(message.chat.id)]['dump'][1]+' мин.'
    msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())



def profil(message):
    #print(message)
    date_time = datetime.now().strftime("%Y-%m-%d")
    flag=0
    data=''
    file=open('dostup.txt', encoding='utf-8')
    for i in file:
        pup=i.strip().split(';')
        if(pup[0]==str(message.chat.id)):
            if(pup[2]=='+'):
                if(pup[1]>date_time):
                    flag=1
                    data=pup[1]
            else:
                flag=0
    file.close()   

    if(flag==1):                 
        messs='Профиль пользователя '+str(message.chat.id)+'\nПодписка действительна до '+data+'\nПо вопросам продления обратитесь к @lexcrypto1'
    else:
        messs='Профиль пользователя '+str(message.chat.id)+'\n🚨 Доступ к моим функциям закрыт.\nПо вопросам продления обратитесь к @lexcrypto1'
    
    msg = bot.send_message(message.chat.id, messs,reply_markup=profil_menu())


def dostup(message):
    messs='Для поддержки обратитесь в чат @lexcrypto1'
    msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())


def send_id_add(message):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=int(message.text.strip())
            
            messs='Введите дату окончания подписки для пользователя '+str(znach)
            msg = bot.send_message(message.chat.id, messs,reply_markup=nazad_menu())
            bot.register_next_step_handler(msg, send_data_add,str(znach))
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    

def send_data_add(message,data):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=message.text.strip()
            file=open('dostup.txt','a', encoding='utf-8')
            file.write(str(data)+';'+znach+';+\n')
            file.close()
            messs='Пользователь '+str(znach)+' добавлен до '+znach+'.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
            
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())

def send_id_del(message):
    try:
        if(message.text.strip()!='<< Назад'):
            znach=int(message.text.strip())
            file=open('dostup.txt','a', encoding='utf-8')
            file.write(str(znach)+';;-\n')
            file.close()
            messs='Пользователь '+str(znach)+' удалён.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=add_menu())

def period_long(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=int(message.text.strip())
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';time;long;'+str(znach)+'\n')
            file.close()
            messs='✔️ Период времени сохранён '+str(znach)+' мин.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def period_short(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=int(message.text.strip())
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';time;short;'+str(znach)+'\n')
            file.close()
            messs='✔️ Период времени сохранён '+str(znach)+' мин.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def period_dump(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=int(message.text.strip())
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';time;dump;'+str(znach)+'\n')
            file.close()
            messs='✔️ Период времени сохранён '+str(znach)+' мин.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def procent_long(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=float(message.text.strip().replace(',','.'))
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';procent;long;'+str(znach)+'\n')
            file.close()
            messs='✔️ Процент изменения цены сохранён '+str(znach)+' %.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    
def procent_short(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=float(message.text.strip().replace(',','.'))
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';procent;short;'+str(znach)+'\n')
            file.close()
            messs='✔️ Процент изменения цены сохранён '+str(znach)+' %.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

def procent_dump(message):
    try:
        if(message.text.strip()!='✖️ Отмена'):
            znach=float(message.text.strip().replace(',','.'))
            file=open('nastroiki.txt','a', encoding='utf-8')
            file.write(str(message.chat.id)+';procent;dump;'+str(znach)+'\n')
            file.close()
            messs='✔️ Процент изменения цены сохранён '+str(znach)+' %.'
            msg = bot.send_message(message.chat.id, messs,reply_markup=nastroyki_menu())
        else:
            bot.clear_step_handler_by_chat_id(message.chat.id)
            messs='❌ Ввод отменен'
            msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    except:
        messs='Ошибка'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())


@bot.message_handler(content_types=['text'])
def func(message):
    if(message.text == "🏦 Выбор биржи"):
        birgi(message)

    if(message.text == "✅ Выключить Binance"):
        file=open('vkl.txt','a', encoding='utf-8')
        file.write(str(message.chat.id)+';binance;-\n')
        file.close()
        messs='Сигналы с биржи Binance выключены'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    if(message.text == "⬜️ Включить Binance"):
        file=open('vkl.txt','a', encoding='utf-8')
        file.write(str(message.chat.id)+';binance;+\n')
        file.close()
        messs='Сигналы с биржи Binance включены'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    if(message.text == "✅ Выключить ByBit"):
        file=open('vkl.txt','a', encoding='utf-8')
        file.write(str(message.chat.id)+';bybit;-\n')
        file.close()
        messs='Сигналы с биржи ByBit выключены'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    if(message.text == "⬜️ Включить ByBit"):
        file=open('vkl.txt','a', encoding='utf-8')
        file.write(str(message.chat.id)+';bybit;+\n')
        file.close()
        messs='Сигналы с биржи ByBit включены'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    if(message.text == "✖️ Назад"):
        messs='❌ Ввод отменен'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())
    if(message.text == "✖️ Отмена"):
        bot.clear_step_handler_by_chat_id(message.chat.id)
        messs='❌ Ввод отменен'
        msg = bot.send_message(message.chat.id, messs,reply_markup=main_menu())

    if(message.text == "⚒️ Настроить"):
        nastroiki(message)
    if(message.text == "💬 Поддержка"):
        dostup(message)
    if(message.text == "🔐 Получить доступ"):
        profil(message)
    if(message.text == "💌 Суппорт"):
        messs='🤖 Есть вопросы по моей работе? Отпиши @lexcrypto1'
        msg = bot.send_message(message.chat.id, messs,reply_markup=profil_menu())
    
    if(message.text == "📈 Период лонг"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='🕑 Текущий период времени, за который должен произойти рост – '+spis[str(message.chat.id)]['long'][1]+' мин\n\nВведи новый период времени: от 1 до 30 минут'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, period_long)

    if(message.text == "🔺 Период памп"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='🕑 Текущий период времени, за который должен произойти памп – '+spis[str(message.chat.id)]['short'][1]+' мин\n\nВведи новый период времени: от 1 до 30 минут'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, period_short)
    if(message.text == "🔻 Период падение"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='🕑 Текущий период времени, за который должно произойти падение – '+spis[str(message.chat.id)]['dump'][1]+' мин\n\nВведи новый период времени: от 1 до 30 минут'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, period_dump)

    if(message.text == "% роста"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='📈Текущий % изменения цены роста – '+str(round(float(spis[str(message.chat.id)]['long'][0]),1))+'%\n\nВведи новый % изменения цены: от 0.5% до 100%'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, procent_long)

    if(message.text == "% пампа"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='🔺Текущий % изменения цены пампа –'+str(round(float(spis[str(message.chat.id)]['short'][0]),1))+'%\n\nВведи новый % изменения цены: от 0.5% до 100%'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, procent_short)

    if(message.text == "% падения"):
        spis={}
        spis[str(message.chat.id)]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        file=open('nastroiki.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            if(pup[0]==str(message.chat.id)):
                if(pup[1]=='procent'):
                    spis[str(message.chat.id)][pup[2]][0]=pup[3]
                else:
                    spis[str(message.chat.id)][pup[2]][1]=pup[3]
        file.close()
        messs='🔻 Текущий % изменения цены падения – '+str(round(float(spis[str(message.chat.id)]['dump'][0]),1))+'%\n\nВведи новый % изменения цены: от 0.5% до 100%'
        msg = bot.send_message(message.chat.id, messs,reply_markup=izmen_menu())
        bot.register_next_step_handler(msg, procent_dump)

    if(message.text == "Добавить пользователя"):
        dostupi={}
        file=open('dostup.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            
            if(pup[2]=='+'):
                dostupi[pup[0]]=pup[1]
            else:
                del dostupi[pup[0]]
        file.close()
        messs=''
        for user in dostupi:
            messs=messs+user+'  '+dostupi[user]+'\n'
        
        msg = bot.send_message(message.chat.id, messs.strip())
        messs='Введите ID пользователя для добавления: '
        msg = bot.send_message(message.chat.id, messs,reply_markup=nazad_menu())
        bot.register_next_step_handler(msg, send_id_add)
    
    if(message.text == "Удалить пользователя"):
        dostupi={}
        file=open('dostup.txt', encoding='utf-8')
        for i in file:
            pup=i.strip().split(';')
            
            if(pup[2]=='+'):
                dostupi[pup[0]]=pup[1]
            else:
                del dostupi[pup[0]]
        file.close()
        messs=''
        for user in dostupi:
            messs=messs+user+'  '+dostupi[user]+'\n'
        
        msg = bot.send_message(message.chat.id, messs.strip())
        messs='Введите ID пользователя для удаления: '
        msg = bot.send_message(message.chat.id, messs,reply_markup=nazad_menu())
        bot.register_next_step_handler(msg, send_id_del)
    


while(True):
    try:
        bot.infinity_polling(timeout=10, long_polling_timeout = 5)
    except:
        print('Error!!!!')
