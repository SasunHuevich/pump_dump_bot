# Обновление контейнеров на сервере

Этот гайд показывает, как обновить бэкенд-сервис на сервере с помощью Docker и Docker Compose.

## 1️⃣ Сохраняем образы локально

```bash
docker compose build
docker save -o all_images.tar postgres:16 pump_dump_bot-bot:latest
```

- `postgres:16` — образ PostgreSQL
- `pump_dump_bot-bot:latest` — новый образ бэкенда
- `all_images.tar` — имя файла для переноса на сервер

## 2️⃣ Копируем tar-файл на сервер

```bash
scp all_images.tar root@138.124.73.82:/root/
```

- Замените `root@138.124.73.82` на ваш сервер
- Файл попадёт в `/root/`

## 3️⃣ Загружаем образы на сервер

```bash
ssh root@138.124.73.82
docker load -i /root/all_images.tar
```

- `docker load` импортирует образы в Docker на сервере

## 4️⃣ Перезапускаем контейнеры через Docker Compose

```bash
cd pump_dump_bot
docker compose up -d
```

- `-d` — запуск в фоне
- Контейнеры будут обновлены новыми образами

## 5️⃣ Проверка

```bash
docker ps
docker images
```

- Убедитесь, что контейнеры запущены и используют новые образы