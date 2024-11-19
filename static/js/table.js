const messagesTable = document.getElementById('messages-table');
const tbody = messagesTable.querySelector('tbody');

// Загрузка данных с сервера и отображение их в таблице
async function fetchMessages() {
    const pageNumber = document.getElementById('page-number');
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
              page: parseInt(pageNumber.value),
              sender: inputSender.value || null,
              datetime_start: startDateInput.value || null,
              datetime_stop: endDateInput.value || null
            })
        });

        if (!response.ok) {
            throw new Error('Network response was not ok: ' + response.statusText);
        }

        const data = await response.json();
        displayMessages(data);
    } catch (error) {
        console.error('Error fetching messages:', error);
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
            <td class="column-status">${formatSize(message.size)}</td>
            <td>${message.passed}</td>
            <td>${formatDate(message.data_box)}</td>
            <td class="column-status">${formatString(message.status)}</td>
        `;
        tbody.appendChild(row);
    });
}

// Загрузка данных при загрузке страницы
window.addEventListener('load', () => {
    fetchMessages().then(() => {
        console.log('Messages loaded');
    }).catch(error => {
        console.error('There was a problem with the fetch operation:', error);
    });
});

// Обработчик для кнопки "Обновить таблицу"
document.getElementById('apply-filter').addEventListener('click', () => {
    fetchMessages();
});
