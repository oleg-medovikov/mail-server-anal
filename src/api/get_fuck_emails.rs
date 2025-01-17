use rocket::{get, response::content::RawHtml};
use sqlx::PgPool;
use rocket::response::Debug;

#[get("/fuck_emails")]
pub async fn get_fuck_emails(pool: &rocket::State<PgPool>) -> Result<RawHtml<String>, Debug<sqlx::Error>> {
    // Формируем SQL-запрос
    let query = r#"SELECT email 
            FROM sender
            WHERE email LIKE '%@zdrav.spb.ru'
                AND email NOT IN (SELECT email FROM known_users)
                AND id IN (SELECT sender_id FROM message 
                    WHERE date > CURRENT_DATE - INTERVAL '14 days')
                ORDER BY email;"#;

    // Выполняем запрос к базе данных
    let emails: Vec<String> = sqlx::query_scalar(query)
        .fetch_all(pool.inner())
        .await
        .map_err(Debug)?;

    // Формируем HTML-страницу
    let html_content = if emails.is_empty() {
        "<html><body><h1>No emails found</h1></body></html>".to_string()
    } else {
        format!(
            "<html><body><h1>Email List</h1><ul>{}</ul></body></html>",
            emails.iter().map(|email| format!("<li>{}</li>", email)).collect::<String>()
        )
    };

    // Возвращаем HTML
    Ok(RawHtml(html_content))
}
