const messagesTable = document.getElementById('messages-table');
const tbody = messagesTable.querySelector('tbody');
const pageNumberInput = document.getElementById('page-number');
const prevPageButton = document.getElementById('prev-page');
const nextPageButton = document.getElementById('next-page');

// Загрузка данных с сервера и отображение их в таблице
async function fetchMessages() {
    const startDateInput = document.getElementById('start-date');
    const endDateInput = document.getElementById('end-date');
    const inputSender = document.getElementById('input-sender');

    try {
        const response = await fetch('/api/messages', {            
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
              page: parseInt(pageNumberInput.value),
              sender: inputSender.value || null,
              datetime_start: startDateInput.value || null,
              datetime_stop: endDateInput.value || null
            })
        });

        if (!response.ok) {
            throw new Error('Ответ сети был неудовлетворительным:' + response.statusText);
        }

        const data = await response.json();
        displayMessages(data);
    } catch (error) {
        console.error('Ошибка при получении сообщений:', error);
    }
}

function formatDate(dateString) {
    if (dateString === null) {
        return "";
    }
    const date = new Date(dateString);
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    const month = (date.getMonth() + 1).toString().padStart(2, '0'); 
    return `${hours}:${minutes} ${day}.${month}`;
}

function formatString(dataString) {
    if (dataString === null) {
        return "";
    } else {
      return dataString;
    }
}

function formatSize(bytes) {
    if (bytes < 1024) {
        return `${bytes} B`;
    } else if (bytes < 1024 * 1024) {
        const kilobytes = bytes / 1024;
        return `${kilobytes.toFixed(2)} KB`;
    } else if (bytes < 1024 * 1024 * 1024) {
        const megabytes = bytes / (1024 * 1024);
        return `${megabytes.toFixed(2)} MB`;
    } else {
        const gigabytes = bytes / (1024 * 1024 * 1024);
        return `${gigabytes.toFixed(2)} GB`;
    }
}

function displayMessages(messages) {
    tbody.innerHTML = ''; // Очищаем таблицу перед добавлением новых данных

    messages.forEach(message => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${formatDate(message.date)}</td>
            <td>${message.sender}</td>
            <td>${message.recipient}</td>
            <td>${message.ip}</td>
            <td class="status-size-column">${formatSize(message.size)}</td>
            <td>${message.passed}</td>
            <td>${formatDate(message.data_box)}</td>
            <td class="status-size-column">${formatString(message.status)}</td>
        `;
        tbody.appendChild(row);
    });
}

// Загрузка данных при загрузке страницы
window.addEventListener('load', () => {
    fetchMessages().then(() => {
        console.log('Сообщения загружены');
    }).catch(error => {
        console.error('Возникла проблема с операцией выборки:', error);
    });
});

// Обработчик для кнопки "Обновить таблицу"
document.getElementById('apply-filter').addEventListener('click', () => {
    fetchMessages();
});

// Обработчик для кнопки "Предыдущая"
prevPageButton.addEventListener('click', () => {
    const currentPage = parseInt(pageNumberInput.value);
    if (currentPage > 1) {
        pageNumberInput.value = currentPage - 1;
        fetchMessages();
    }
});

// Обработчик для кнопки "Следующая"
nextPageButton.addEventListener('click', () => {
    const currentPage = parseInt(pageNumberInput.value);
    pageNumberInput.value = currentPage + 1;
    fetchMessages();
});

// Очищение таблицы
document.querySelector('.search-form__btn').addEventListener('click', function() {
    document.getElementById('input-sender').value = '';
});
