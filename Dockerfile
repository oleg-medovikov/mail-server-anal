# Используем официальный образ Rust для сборки
FROM rust:latest as builder

# Создаем рабочую директорию
WORKDIR /usr/src/rocket_web

# Копируем файлы проекта в контейнер
COPY . .

# Собираем проект
RUN cargo build --release

# Используем минимальный образ для запуска
FROM debian:buster-slim

# Копируем собранный бинарный файл из предыдущего этапа
COPY --from=builder /usr/src/rocket_web/target/release/rocket_web /usr/local/bin/rocket_web

# Указываем команду для запуска приложения
CMD ["rocket_web"]
