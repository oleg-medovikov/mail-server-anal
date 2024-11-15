// Загрузка данных с сервера и отображение их в таблице
export async function fetchMessages() {
    console.log('Fetching messages...');

    const url = "/api/messages";

    const json = {
        "page": 1,
        "sender": null,
        "datetime_start": null,
        "datetime_stop": null,
    };

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(json)
        });

        if (!response.ok) {
            throw new Error('Network response was not ok: ' + response.statusText);
        }

        const data = await response.json();
        console.log('Messages fetched:', data);
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

function displayMessages(messages) {
    const tbody = document.querySelector('tbody'); // Предполагается, что у вас есть элемент <tbody> в HTML
    tbody.innerHTML = ''; // Очищаем таблицу перед добавлением новых данных

    messages.forEach(message => {
        const row = document.createElement('tr');
        row.innerHTML = `
            <td>${formatDate(message.date)}</td>
            <td>${message.sender}</td>
            <td>${message.recipient}</td>
            <td>${message.ip}</td>
            <td>${message.size}</td>
            <td>${message.passed}</td>
            <td>${formatDate(message.data_box)}</td>
            <td>${message.status}</td>
        `;
        tbody.appendChild(row);
    });
}

