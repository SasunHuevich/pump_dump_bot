FROM rust:1.91-bullseye AS builder

# Устанавливаем зависимости для сборки с OpenSSL
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Минимальный образ для запуска
FROM debian:bullseye-slim

# Устанавливаем runtime зависимости
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/pump_dump_bot .

CMD ["./pump_dump_bot"]
