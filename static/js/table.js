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

        // Создаем ячейки таблицы
        const dateCell = document.createElement('td');
        dateCell.textContent = formatDate(message.date);
        row.appendChild(dateCell);

        const senderCell = document.createElement('td');
        senderCell.textContent = message.sender;
        senderCell.classList.add('sender-cell'); // Добавляем класс
        senderCell.addEventListener('click', () => {
            const inputSender = document.getElementById('input-sender');
            inputSender.value = message.sender;
        });
        row.appendChild(senderCell);

        const recipientCell = document.createElement('td');
        recipientCell.textContent = message.recipient;
        recipientCell.classList.add('recipient-cell'); // Добавляем класс
        row.appendChild(recipientCell);

        const ipCell = document.createElement('td');
        ipCell.textContent = message.ip;
        ipCell.classList.add('ip-cell'); // Добавляем класс
        row.appendChild(ipCell);

        const sizeCell = document.createElement('td');
        sizeCell.textContent = formatSize(message.size);
        sizeCell.classList.add('status-size-column'); // Добавляем класс
        row.appendChild(sizeCell);

        const passedCell = document.createElement('td');
        passedCell.textContent = message.passed;
        passedCell.classList.add('passed-cell'); // Добавляем класс
        row.appendChild(passedCell);

        const dataBoxCell = document.createElement('td');
        dataBoxCell.textContent = formatDate(message.data_box);
        dataBoxCell.classList.add('data-box-cell'); // Добавляем класс
        row.appendChild(dataBoxCell);

        const statusCell = document.createElement('td');
        statusCell.textContent = formatString(message.status);
        statusCell.classList.add('status-size-column'); // Добавляем класс
        row.appendChild(statusCell);

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

document.getElementById('start-date').addEventListener('keypress', (event) => {
    if (event.key === 'Enter') {
        fetchMessages();
    }
});
