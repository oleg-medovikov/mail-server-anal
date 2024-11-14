#!/bin/bash
# Записываем время начала выполнения скрипта
start=$(date)
start_time=$(date +%s)
echo "== $start ========= Начинаем анализировать логи ======"
# проверяем наличие дириктории
if [ ! -d ~/mail_log ]; then
    mkdir ~/mail_log
fi
# Перемещаем старый файл mail.log в mail_log_old
if [ -f ~/mail_log/mail.log ]; then
    mv ~/mail_log/mail.log ~/mail_log/mail_log_old
fi
# Синхронизируем файлы с удаленного сервера
rsync -avz -e 'ssh -p 2222' --progress user@10.144.4.33:/var/log/mail.log ~/mail_log/mail.log > /dev/null 2>&1
# Создаем файл mail_log_delta, содержащий только новые строки
if [ -f ~/mail_log/mail_log_old ]; then
    diff -u ~/mail_log/mail_log_old ~/mail_log/mail.log | grep '^+' | sed 's/^+//' | tail -n +2 > ~/mail_log/mail_log_delta
else
    cp ~/mail_log/mail.log ~/mail_log/mail_log_delta
fi
echo "Новые строки сохранены в ~/mail_log/mail_log_delta."
# Перемещаем старый файл mail.log в mail_log_old
if [ -f ~/mail_log/lda.log ]; then
    mv ~/mail_log/lda.log ~/mail_log/lda_log_old
fi
# Синхронизируем файлы с удаленного сервера
rsync -avz -e 'ssh -p 2222' --progress user@10.144.4.33:/var/log/dovecot/lda.log ~/mail_log/lda.log > /dev/null 2>&1

# Создаем файл mail_log_delta, содержащий только новые строки
if [ -f ~/mail_log/lda_log_old ]; then
    diff -u ~/mail_log/lda_log_old ~/mail_log/lda.log | grep '^+' | sed 's/^+//' | tail -n +2 > ~/mail_log/lda_log_delta
else
    cp ~/mail_log/lda.log ~/mail_log/lda_log_delta
fi
echo "Новые строки сохранены в ~/mail_log/lda_log_delta."
# Отправляем curl запрос и дожидаемся ответа
response_code=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/api/read_files)

# Проверяем, что response_code содержит целочисленное значение
if [[ -n "$response_code" && "$response_code" =~ ^[0-9]+$ ]]; then
    if [ "$response_code" -eq 200 ]; then
        echo "Ответ от сервера получен. Код: $response_code"
    else
        echo "Ошибка: Сервер вернул код $response_code"
    fi
else
    echo "Ошибка: Не удалось получить код ответа от сервера."
fi
# Записываем время окончания выполнения скрипта
end_time=$(date +%s)
# Вычисляем время выполнения скрипта
elapsed_time=$((end_time - start_time))
# Выводим время выполнения скрипта
echo "Время выполнения скрипта: $elapsed_time секунд"
