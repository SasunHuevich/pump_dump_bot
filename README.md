# pump_dump_bot

Telegram-бот на Rust, который следит за ценами фьючерсов на **Binance**, **Bybit** и **BingX**
и присылает уведомление, когда монета резко меняется в цене:

- 📈 **Лонг**: небольшой рост за короткий период, сигнал на вход в лонг;
- 🔺 **Памп**: резкий рост, сигнал на вход в шорт;
- 🔻 **Дамп**: резкое падение, сигнал на торговлю коррекции.

Каждый пользователь сам выбирает биржи и настраивает пороги: процент изменения и окно в минутах (до 30).
Доступ к боту выдают администраторы, у подписки есть дата окончания.

## Как работает

1. Раз в минуту фоновая задача забирает цены по всем фьючерсам с публичных API бирж
   (API-ключи не нужны) и сохраняет их в PostgreSQL.
2. Для каждого пользователя с активным доступом считается изменение цены на его интервалах.
   Если порог превышен, бот отправляет уведомление. Повторные уведомления по той же монете ограничиваются.
3. Старые цены и уведомления периодически удаляются.

### Роли

| Роль | Что может |
|---|---|
| `user` | Выбирать биржи, настраивать пороги, проверять срок доступа |
| `admin` | Всё то же + добавлять/удалять пользователей и админов, выдавать подписку до даты |
| `no_access` | Ничего: бот отвечает, что доступ закрыт |

Первый админ задаётся в [docker-entrypoint-initdb.d/init_db.sql](docker-entrypoint-initdb.d/init_db.sql):
замените `user_id` в `INSERT INTO users` на свой Telegram ID (узнать его можно, например, у [@userinfobot](https://t.me/userinfobot)).

## Структура

| Путь | Что там |
|---|---|
| [src/main.rs](src/main.rs) | Точка входа: фоновый цикл сбора цен и обработчик сообщений |
| [src/exchanges.rs](src/exchanges.rs) | Получение цен с Binance, Bybit, BingX |
| [src/misc.rs](src/misc.rs) | Расчёт изменений цены и рассылка уведомлений |
| [src/db.rs](src/db.rs) | Запросы к PostgreSQL (sqlx) |
| [src/handlers.rs](src/handlers.rs), [src/handlers_ext/](src/handlers_ext/) | Меню и состояния диалога |
| [src/buttons.rs](src/buttons.rs), [src/texts.rs](src/texts.rs) | Клавиатуры и тексты |
| [docker-entrypoint-initdb.d/init_db.sql](docker-entrypoint-initdb.d/init_db.sql) | Схема БД, выполняется при первом старте Postgres |

Стек: Rust (edition 2024), [teloxide](https://github.com/teloxide/teloxide), sqlx, reqwest, tokio, PostgreSQL 16.

## Запуск через Docker Compose

1. Создайте бота у [@BotFather](https://t.me/BotFather) и получите токен.
2. Создайте `.env` из примера и впишите в него токен:
   ```bash
   cp .env.example .env
   ```
3. Укажите свой Telegram ID в `init_db.sql` (см. «Роли»).
4. Запустите:
   ```bash
   docker compose up -d --build
   ```

Логи: `docker compose logs -f bot`.

> `init_db.sql` выполняется только при создании пустого тома `db_data`.
> Пересоздать базу с нуля: `docker compose down -v`.

## Локальный запуск без Docker

Нужны Rust 1.91+ и запущенный PostgreSQL со схемой из `init_db.sql`. В `.env`:

```
TELOXIDE_TOKEN=<токен>
DATABASE_URL=postgresql://user:pass@localhost:5432/bot_db
RUST_LOG=info
```

```bash
cargo run --release
```

## Деплой на сервер без сборки на нём

Образы собираются локально и переносятся архивом:

```bash
docker compose build
docker save -o all_images.tar postgres:16 pump_dump_bot-bot:latest
scp all_images.tar user@your-server:/root/

ssh user@your-server
docker load -i /root/all_images.tar
cd pump_dump_bot          # тут должны лежать docker-compose.yaml, .env и docker-entrypoint-initdb.d/
docker compose up -d
docker ps                 # проверить, что контейнеры запущены
```
