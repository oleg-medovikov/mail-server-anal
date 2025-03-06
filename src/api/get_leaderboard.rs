use rocket::{post, serde::json::Json, State};
use sqlx::{PgPool, query_as, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InputRequest {
    datetime_start: String,
    datetime_stop:  String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Recipient {
    pub count: i64,
    pub recipient: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sender {
    pub count: i64,
    pub sender: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub recipients: Vec<Recipient>,
    pub senders: Vec<Sender>,
}

#[post("/leaderboard", format = "json", data = "<input_request>")]
pub async fn get_leaderboard(pool: &State<PgPool>, input_request: Json<InputRequest>) -> Result<Json<Data>, rocket::response::Debug<sqlx::Error>> {
    // Получаем данные из запроса
    let datetime_start = &input_request.datetime_start;
    let datetime_stop = &input_request.datetime_stop;

    // Запрос для получения списка получателей
    let recipients: Vec<Recipient> = query_as(
        r#"
        SELECT COUNT(*) AS count, r.email as recipient
        FROM message m
        INNER JOIN recipient r ON (m.recipient_id = r.id)
        WHERE date BETWEEN TO_TIMESTAMP($1, 'YYYY-MM-DD') 
              AND TO_TIMESTAMP($2, 'YYYY-MM-DD')
        GROUP BY r.email
        ORDER BY count DESC
        LIMIT 20
        "#,
    )
    .bind(datetime_start)
    .bind(datetime_stop)
    .fetch_all(pool.inner())
    .await?;

    // Запрос для получения списка отправителей
    let senders: Vec<Sender> = query_as(
        r#"
        SELECT COUNT(*) AS count, s.email as sender
        FROM message m 
        INNER JOIN sender s ON (m.sender_id = s.id)
        WHERE date BETWEEN TO_TIMESTAMP($1, 'YYYY-MM-DD') 
              AND TO_TIMESTAMP($2, 'YYYY-MM-DD')
        GROUP BY s.email
        ORDER BY count DESC
        LIMIT 20
        "#,
    )
    .bind(datetime_start)
    .bind(datetime_stop)
    .fetch_all(pool.inner())
    .await?;

    // Формируем ответ
    let data = Data {
        recipients,
        senders,
    };

    Ok(Json(data))
}
