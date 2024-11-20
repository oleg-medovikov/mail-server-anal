document.getElementById('logoutButton').addEventListener('click', function(event) {
  event.preventDefault(); // Предотвращает стандартное действие кнопки (например, переход по ссылке)
  
  if (confirm('Вы уверены, что хотите выйти?')) {
    // Если пользователь нажал "ОК"
    console.log('Выход из системы');

    // Отправляем запрос на сервер для удаления токена
    fetch(`api/drop_token`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json'
      },
      body: JSON.stringify({ 
        token: localStorage.getItem('authToken')
      })
    })
    .then(response => {
      if (!response.ok) {
        throw new Error('Ответ сети был неудовлетворительным:');
      }
      localStorage.removeItem('authToken');
      window.location.href = 'login.html';
    })
    .catch(error => {
      console.error('Возникла проблема с операцией выборки:', error);
    });
  } else {
    // Если пользователь нажал "Отменить", ничего не происходит
    console.log('Отмена выхода');
  }
});
