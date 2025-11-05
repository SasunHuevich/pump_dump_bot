# Pump & Dump Bot

Бот для мониторинга криптовалютных пар на Bybit и Binance и генерации сигналов на основе свечей (kline).

---

## Что нужно для запуска

Установка Docker:

```bash
sudo apt install -y docker || sudo dnf install -y docker
```

---

## Запуск и остановка бота

### Запуск

Запуск бота через скрипт:
```bash
./run_bot.sh
```

### Остановка

Остановка бота через скрипт:
```bash
./stop_bot.sh
```

Иногда Docker-контейнер не хочет останавливаться (ошибка Permission denied...). В этом случае можно принудительно завершить процесс контейнера, а потом повторить остановку:
```bash
kill -9 $(sudo docker inspect -f '{{.State.Pid}}' pump_dump_bot) && ./stop_bot.sh
```
