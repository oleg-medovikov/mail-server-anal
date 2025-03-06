document.addEventListener('DOMContentLoaded', () => {
    const sendersTable = document.getElementById('senders-table');
    const recipientsTable = document.getElementById('recipients-table');

    function handleEmailClick(event) {
        const email = event.target.textContent.trim();
        if (email) {
            window.location.href = `main.html?email=${encodeURIComponent(email)}`;
        }
    }

    function addEventListeners() {
        if (sendersTable) {
            sendersTable.addEventListener('click', (event) => {
                if (event.target.tagName === 'TD' && event.target.classList.contains('sender-column')) {
                    handleEmailClick(event);
                }
            });
        }

        if (recipientsTable) {
            recipientsTable.addEventListener('click', (event) => {
                if (event.target.tagName === 'TD' && event.target.classList.contains('recipient-column')) {
                    handleEmailClick(event);
                }
            });
        }
    }

    // Пример функции для загрузки данных
    function loadData() {
        console.log('Загрузка данных...');
        fetchMessages('senders').then(() => {
            console.log('Данные для отправителей загружены');
            addEventListeners();
        }).catch(error => {
            console.error('Ошибка при получении данных для отправителей:', error);
        });

        fetchMessages('recipients').then(() => {
            console.log('Данные для получателей загружены');
            addEventListeners();
        }).catch(error => {
            console.error('Ошибка при получении данных для получателей:', error);
        });
    }

    document.querySelectorAll('.filter-buttons a').forEach(link => {
        link.addEventListener('click', (event) => {
            event.preventDefault(); // Предотвращаем переход по ссылке

            const table = link.getAttribute('data-table');
            const days = link.getAttribute('data-days');
            const endDate = new Date();
            const startDate = new Date();
            startDate.setDate(endDate.getDate() - days);

            document.getElementById('start-date').value = startDate.toISOString().split('T')[0];
            document.getElementById('end-date').value = endDate.toISOString().split('T')[0];

            fetchMessages(table).then(() => {
                // Удаляем класс active у всех ссылок для текущей таблицы
                document.querySelectorAll(`.filter-buttons a[data-table="${table}"]`).forEach(btn => btn.classList.remove('active'));
                // Добавляем класс active к нажатой ссылке
                link.classList.add('active');
            }).catch(error => {
                console.error('Ошибка при получении сообщений:', error);
            });
        });
    });

    async function fetchMessages(table) {
        const startDateInput = document.getElementById('start-date');
        const endDateInput = document.getElementById('end-date');
        const inputSender = document.getElementById('input-sender');

        try {
            const response = await fetch('/api/leaderboard', {            
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    datetime_start: startDateInput.value || null,
                    datetime_stop: endDateInput.value || null
                })
            });

            if (!response.ok) {
                throw new Error('Ответ сети был неудовлетворительным:' + response.statusText);
            }

            const data = await response.json();
            displayMessages(data, table);
        } catch (error) {
            console.error('Ошибка при получении сообщений:', error);
            throw error; // Передаем ошибку дальше, чтобы обработать ее в обработчике события
        }
    }

    function displayMessages(data, table) {
        const tbody = document.querySelector(`#${table}-table tbody`);
        tbody.innerHTML = ''; // Очищаем таблицу перед добавлением новых данных

        const items = table === 'senders' ? data.senders : data.recipients;

        items.forEach(item => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td class="${table.slice(0, -1)}-column">${item[table.slice(0, -1)]}</td>
                <td class="quantity-column">${item.count}</td>
            `;
            tbody.appendChild(row);
        });
    }

    // Загрузка данных при загрузке страницы
    window.addEventListener('load', () => {
        loadData();
    });
});
