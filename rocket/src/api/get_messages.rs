use rocket::{post, serde::json::Json, form::Form};
use sqlx::PgPool;
use crate::models::{QueryParams, Message};

#[post("/messages", data = "<params>")]
pub async fn get_messages(params: Form<QueryParams>, pool: &rocket::State<PgPool>) -> Result<Json<Vec<Message>>, rocket::response::Debug<sqlx::Error>> {
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * 100;

    let mut query = String::from(
        r#"
        SELECT
            m.date,
            s.email AS sender,
            r.email AS recipient,
            i.ip,
            m.size,
            CASE
                WHEN m.passed IS TRUE THEN 'PASSED'
                ELSE 'SPAM'
            END AS passed,
            ms.date as data_box,
            st.mess as status
        FROM message m
        INNER JOIN sender s ON m.sender_id = s.id
        INNER JOIN recipient r ON m.recipient_id = r.id
        INNER JOIN ip i ON i.id = m.ip_id
        LEFT JOIN mess_status ms ON ms.message_id = m.id
        LEFT JOIN status st ON ms.status_id = st.id
        "#,
    );

    let mut conditions = Vec::new();
    if let Some(sender) = &params.sender {
        conditions.push(format!("s.email = '{}'", sender));
    }
    if let Some(datetime_start) = &params.datetime_start {
        conditions.push(format!("m.date >= '{}'", datetime_start));
    }
    if let Some(datetime_stop) = &params.datetime_stop {
        conditions.push(format!("m.date <= '{}'", datetime_stop));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY m.date DESC LIMIT 100 OFFSET ");
    query.push_str(&offset.to_string());

    let messages: Vec<Message> = sqlx::query_as(&query)
        .fetch_all(pool.inner())
        .await?;

    Ok(Json(messages))
}
