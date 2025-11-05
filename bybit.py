from pybit.unified_trading import HTTP
import telebot
import time
import os
from datetime import datetime
from dotenv import load_dotenv

load_dotenv()

bot = telebot.TeleBot(os.getenv("BOT_TOKEN"), threaded=False)

signals={}
file=open('signals.txt', encoding='utf-8')
for i in file:
    pup=i.strip().split(';')
    if(pup[0] not in signals):
        signals[pup[0]]={}
    if(pup[1] not in signals[pup[0]]):
        signals[pup[0]][pup[1]]={}
    if(pup[2] not in signals[pup[0]][pup[1]]):
        signals[pup[0]][pup[1]][pup[2]]={'bybit':0,'binance':0}
    signals[pup[0]][pup[1]][pup[2]][pup[3]]=signals[pup[0]][pup[1]][pup[2]][pup[3]]+1
file.close()

all_coins=[]
file=open('resss.txt', encoding='utf-8')
for i in file:
    all_coins.append(i.strip())
file.close()

def have_dostup(id,dostup):
    date_time = datetime.now().strftime("%Y-%m-%d")
    flag=0
    if(id in dostup):
        if(dostup[id]>date_time):
            flag=1
    if(flag==1):
        return True
    else:
        return False
    
def bybit_vkl(id,vklu):
    flag=0
    if(id in vklu):
        if(vklu[id]['bybit']=='+'):
            flag=1
    else:
        flag=1
    if(flag==1):
        return True
    else:
        return False


def prover_dubli(proshlie,user,sym,regim,vremya,prom):
    bili=[]
    flag=1
    try:
        for i in prom[:vremya]:
            bili.append(i[0])
        if(proshlie[user][regim][sym] in bili):
            flag=0
        else:
            flag=1
    except:
        flag=1
    if(flag==1):
        return True
    else:
        return False
proshlie={}


while(True):

    print('ByBit running...')

    dostupi={}
    file=open('dostup.txt', encoding='utf-8')
    for i in file:
        pup=i.strip().split(';')
        
        if(pup[2]=='+'):
            dostupi[pup[0]]=pup[1]
        else:
            del dostupi[pup[0]]
    file.close()


    vkl={}
    file=open('vkl.txt', encoding='utf-8')
    for i in file:
        pup=i.strip().split(';')
        if(pup[0] not in vkl):
            vkl[pup[0]]={'bybit':'+','binance':'+'}
        vkl[pup[0]][pup[1]]=pup[2]
    file.close()


    spis={}
    file=open('nastroiki.txt', encoding='utf-8')
    for i in file:
        pup=i.strip().split(';')
        if(pup[0] not in spis):
            spis[pup[0]]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
        
        if(pup[1]=='procent'):
            spis[pup[0]][pup[2]][0]=pup[3]
        else:
            spis[pup[0]][pup[2]][1]=pup[3]
    file.close()

    slovar={}

    def safe_get_kline(session, **kwargs):
        for attempt in range(10):
            try:
                response = session.get_kline(**kwargs)
                if isinstance(response, dict):
                    if response.get("retCode") == 0:
                        return response
                    else:
                        print(f"[WARN] Попытка {attempt+1}/10: retCode={response.get('retCode')}")
                else:
                    print(f"[WARN] Попытка {attempt+1}/10: неожиданный тип ответа ({type(response)})")
            except Exception as e:
                print(f"[ERROR] Попытка {attempt+1}/10: {e}")
            time.sleep(3)

        raise Exception("get_kline failed after 10 retries (no 200/retCode=0)")


    for sym in all_coins[:]:
        time_now=int(str(time.time()).split('.')[0]+'000')
        time_prev=time_now-(60*150*1000)
        # print(sym)
        try:
            slovar[sym]=[]
            session = HTTP(
                testnet=False,
                api_key=os.getenv("BYBIT_API_KEY"),
                api_secret=os.getenv("BYBIT_API_SECRET"),
            )
            prom=safe_get_kline(
                category="linear",
                symbol=sym,
                interval='1',
                start=time_prev,
                end=time_now,
            )
            # print(prom)
            
            
            for user in dostupi:
                if(have_dostup(user,dostupi) and bybit_vkl(user,vkl)):
                    if(user not in spis):
                        spis[user]={'long':['3','5'],'short':['7','20'],'dump':['7','20']}
                    for regim in spis[user]:
                        razn=((float(prom['result']['list'][0][2])-float(prom['result']['list'][int(spis[user][regim][1])-1][3]))/float(prom['result']['list'][int(spis[user][regim][1])-1][3]))*100
                        # print(regim,razn)
                        if(regim=='long' and prover_dubli(proshlie,user,sym,regim,int(spis[user][regim][1]),prom['result']['list'])):
                            if(razn>float(spis[user][regim][0])):
                                tekkkk=datetime.now().strftime("%Y-%m-%d")
                                if(user not in signals):
                                    signals[user]={}
                                if(tekkkk not in signals[user]):
                                    signals[user][tekkkk]={}
                                if(sym not in signals[user][tekkkk]):
                                    signals[user][tekkkk][sym]={'bybit':0,'binance':0}
                                signals[user][tekkkk][sym]['bybit']=signals[user][tekkkk][sym]['bybit']+1
                                mess='🏦BYBIT • '+spis[user][regim][1]+'м • '+sym+'\n📈Рост: '+ str(round(razn,2))+'% ('+prom['result']['list'][int(spis[user][regim][1])-1][3]+'-'+prom['result']['list'][0][2]+')\n🔔Сигнал за сутки: '+str(signals[user][tekkkk][sym]['bybit'])
                                file=open('signals.txt','a', encoding='utf-8')
                                file.write(user+';'+datetime.now().strftime("%Y-%m-%d")+';'+sym+';'+'bybit;'+prom['result']['list'][0][0]+'\n')
                                file.close()
                                if(user not in proshlie):
                                    proshlie[user]={'long':{},'short':{},'dump':{}}
                                proshlie[user]['long'][sym]=prom['result']['list'][0][0]
                                try:
                                    bot.send_message(int(user), mess)
                                    time.sleep(0.1)
                                except:
                                    print('ERRORRR')
                                    #input('scsdcsdcdCdscs')
                        elif(regim=='short' and prover_dubli(proshlie,user,sym,regim,int(spis[user][regim][1]),prom['result']['list'])):
                            if(razn>float(spis[user][regim][0])):
                                tekkkk=datetime.now().strftime("%Y-%m-%d")
                                if(user not in signals):
                                    signals[user]={}
                                if(tekkkk not in signals[user]):
                                    signals[user][tekkkk]={}
                                if(sym not in signals[user][tekkkk]):
                                    signals[user][tekkkk][sym]={'bybit':0,'binance':0}
                                signals[user][tekkkk][sym]['bybit']=signals[user][tekkkk][sym]['bybit']+1
                                mess='🏦BYBIT • '+spis[user][regim][1]+'м • '+sym+'\n🔺Памп: '+ str(round(razn,2))+'% ('+prom['result']['list'][int(spis[user][regim][1])-1][3]+'-'+prom['result']['list'][0][2]+')\n🔔Сигнал за сутки: '+str(signals[user][tekkkk][sym]['bybit'])
                                file=open('signals.txt','a', encoding='utf-8')
                                file.write(user+';'+datetime.now().strftime("%Y-%m-%d")+';'+sym+';'+'bybit;'+prom['result']['list'][0][0]+'\n')
                                file.close()
                                if(user not in proshlie):
                                    proshlie[user]={'long':{},'short':{},'dump':{}}
                                proshlie[user]['short'][sym]=prom['result']['list'][0][0]
                                try:
                                    bot.send_message(int(user), mess)
                                    time.sleep(0.1)
                                except:
                                    print('ERRORRR')
                                    #input('scsdcsdcdCdscs')
                        elif(regim=='dump' and prover_dubli(proshlie,user,sym,regim,int(spis[user][regim][1]),prom['result']['list'])):
                            if(razn<-float(spis[user][regim][0])):
                                tekkkk=datetime.now().strftime("%Y-%m-%d")
                                if(user not in signals):
                                    signals[user]={}
                                if(tekkkk not in signals[user]):
                                    signals[user][tekkkk]={}
                                if(sym not in signals[user][tekkkk]):
                                    signals[user][tekkkk][sym]={'bybit':0,'binance':0}
                                signals[user][tekkkk][sym]['bybit']=signals[user][tekkkk][sym]['bybit']+1
                                mess='🏦BYBIT • '+spis[user][regim][1]+'м • '+sym+'\n🔻Падение: '+ str(round(razn,2))+'% ('+prom['result']['list'][int(spis[user][regim][1])-1][3]+'-'+prom['result']['list'][0][2]+')\n🔔Сигнал за сутки: '+str(signals[user][tekkkk][sym]['bybit'])
                                file=open('signals.txt','a', encoding='utf-8')
                                file.write(user+';'+datetime.now().strftime("%Y-%m-%d")+';'+sym+';'+'bybit;'+prom['result']['list'][0][0]+'\n')
                                file.close()
                                if(user not in proshlie):
                                    proshlie[user]={'long':{},'short':{},'dump':{}}
                                proshlie[user]['dump'][sym]=prom['result']['list'][0][0]
                                try:
                                    bot.send_message(int(user), mess)
                                    time.sleep(0.1)
                                except:
                                    print('ERRORRR')
                                    #input('scsdcsdcdCdscs')
        
        except:
            print('Miniii')
             
        
