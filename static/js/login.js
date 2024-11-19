document.getElementById('loginForm').addEventListener('submit', function(event) {
  event.preventDefault(); 
  // const username = document.getElementById('username').value;
  // const password = document.getElementById('password').value;


  fetch(`api/user_login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ 
      username: event.target.username.value,
      password: event.target.password.value
    })
  })
  .then(response => {
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    return response.json();
  })
  .then(data => {
    localStorage.setItem('authToken', data.token);
    window.location.href = 'index.html';
  })
  .catch(error => {
    console.error('There was a problem with the fetch operation:', error);

    alert('Login failed. Please check your credentials.');
  });
});
