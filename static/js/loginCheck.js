// Проверяем наличие токена в localStorage
const authToken = localStorage.getItem('authToken');

if (!authToken) {
  // Если токена нет, перенаправляем на страницу входа
  window.location.href = '/login.html';
} else {
  // Отправляем запрос к API для проверки токена
  fetch('/api/check_token', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ 
      token: authToken
    })
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
    }
  })
  .catch(error => {
    console.error('There was a problem with the fetch operation:', error);
    // В случае ошибки также перенаправляем на страницу входа
    window.location.href = '/login.html';
  });
}
