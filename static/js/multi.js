document.addEventListener('DOMContentLoaded', () => {
    const urlParams = new URLSearchParams(window.location.search);
    const email = urlParams.get('email');
    const inputSender = document.getElementById('input-sender');

    if (email && inputSender) {
        inputSender.value = email;
    }

    const sendersTable = document.getElementById('senders-table');
    const recipientsTable = document.getElementById('recipients-table');

    function handleEmailClick(event) {
        const email = event.target.textContent.trim();
        if (email) {
            window.location.href = `main.html?email=${encodeURIComponent(email)}`;
        }
    }

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
});