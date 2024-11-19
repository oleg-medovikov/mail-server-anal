// Проверяем наличие токена в localStorage
if (!localStorage.getItem('authToken')) {
  // Если токена нет, перенаправляем на страницу входа
  window.location.href = '/login.html';
} else {
  // Отправляем запрос к API для проверки токена
  fetch(`/check_token`, {
    method: 'GET',
    headers: {
      'Authorization': localStorage.getItem('authToken')
    }
  })
  .then(response => {
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    return response.json();
  })
  .then(data => {
    if (!data.token_valid) {
      // Если токен не валиден, перенаправляем на страницу входа
      window.location.href = '/login.html';
    } else { // Если токен валидный - мы вытаскиваем пользователя
      fetch(`/user_get`, { 
        method: 'GET',
        headers: {
          'Authorization': localStorage.getItem('authToken')
        }
      })
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json(); // Преобразуем тело ответа в JSON
      })
      .then(data => {  
        localStorage.setItem('fio', data.fio); 
        localStorage.setItem('groups', data.groups);
        document.getElementById('UserFIO').innerHTML = localStorage.getItem('fio');
        document.getElementById('UserGroups').innerHTML = localStorage.getItem('groups');
      })
      .catch(error => {
        console.error('There has been a problem with your fetch operation:', error);
      });
    }
  })
  .catch(error => {
    console.error('There was a problem with the fetch operation:', error);
  });
}
