# Используем официальный Rust-образ
FROM rust:1.85 as builder

# Создаём рабочую директорию
WORKDIR /usr/src/bot

# Копируем файлы проекта
COPY . .

# Собираем проект в релизе
RUN cargo build --release

# Минимальный образ для запуска
FROM debian:buster-slim

WORKDIR /usr/src/bot

# Копируем бинарник из билдера
COPY --from=builder /usr/src/bot/target/release/pump_dump_bot .

# Команда запуска
CMD ["./pump_dump_bot"]