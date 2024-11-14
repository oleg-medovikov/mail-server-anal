use rocket::{get, serde::json::Json};
use sqlx::PgPool;

#[get("/get_senders")]
pub async fn get_senders(pool: &rocket::State<PgPool>) -> Result<Json<Vec<String>>, rocket::response::Debug<sqlx::Error>> {
    let query = r#"
        SELECT DISTINCT
            email
        FROM
            sender
        where
            LENGTH(email) < 50;
        "#;

    let emails: Vec<String> = sqlx::query_scalar(query)
        .fetch_all(pool.inner())
        .await?;

    Ok(Json(emails))
}
