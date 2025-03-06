const searchInput = document.querySelector('.search-form__input');
const suggestionsContainer = document.querySelector('.search-results');
let allSenders = [];

// Функция для получения списка отправителей
async function fetchSenders() {
    const response = await fetch('/api/get_senders');
    if (!response.ok) {
        throw new Error('Ответ сети был неудовлетворительным:');
    }
    const data = await response.json();
    allSenders = data;
}

// Функция для отображения подсказок
function showSuggestions(query) {
    const filteredSenders = allSenders.filter(sender => sender.toLowerCase().includes(query.toLowerCase()));
    suggestionsContainer.innerHTML = '';
    filteredSenders.slice(0, 10).forEach(sender => {
        const suggestionElement = document.createElement('div');
        suggestionElement.textContent = sender;
        suggestionElement.addEventListener('click', () => {
            searchInput.value = sender;
            suggestionsContainer.style.display = 'none';
            document.body.classList.remove('modal-open'); // Убираем класс, чтобы возобновить прокрутку
        });
        suggestionsContainer.appendChild(suggestionElement);
    });
    suggestionsContainer.style.display = filteredSenders.length > 0 ? 'block' : 'none';
    document.body.classList.add('modal-open'); // Добавляем класс, чтобы заблокировать прокрутку
}

// Обработчик ввода
searchInput.addEventListener('input', () => {
    const query = searchInput.value.trim();
    if (query.length > 0) {
        showSuggestions(query);
    } else {
        suggestionsContainer.style.display = 'none';
        document.body.classList.remove('modal-open'); // Убираем класс, чтобы возобновить прокрутку
    }
});

// Загрузка списка отправителей при загрузке страницы
window.addEventListener('load', () => {
    fetchSenders().then(() => {
        console.log('Senders loaded');
    }).catch(error => {
        console.error('Возникла проблема с операцией выборки:', error);
    });
});
