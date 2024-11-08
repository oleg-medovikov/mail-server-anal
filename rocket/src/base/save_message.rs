use sqlx::PgPool;
use crate::models::MessageInfo;
use sqlx::query;


pub async fn save_message (pool: &PgPool, mess: MessageInfo) -> Result<(), sqlx::Error> {
  // сначала добавляем sender

    let sender_id = query(
    r#"
        INSERT INTO sender (email)
        values ($1)
         ON CONFLICT (email)
         DO NOTHING
         RETURNING id;
    "#
    )
    .bind(&mess.sender)
    .fetch_one(pool)
    .await?;

    let recipient_id = query(
    r#"
        INSERT INTO recipient (email)
        values ($1)
         ON CONFLICT (email)
         DO NOTHING
         RETURNING id;
    "#
    )
    .bind(&mess.recipient)
    .fetch_one(pool)
    .await?;

    let ip_id:i32 = query(
    r#"
        INSERT INTO ip (ip)
        values ($1)
         ON CONFLICT (ip)
         DO NOTHING
         RETURNING id;
    "#
    )
    .bind(&mess.ip_address)
    .fetch_one(pool)
    .await
    .expect("не смог получить id ip");

    let message = query(
    r#"
        INSERT INTO message (
            id, date, passed, size, sender_id, recipient_id, ip_id
            )
        values ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (id)
         DO NOTHING
         RETURNING id;
    "#
    )
    .bind(&mess.message_id)
    .bind(&mess.date)
    .bind(&mess.passed)
    .bind(&mess.size)
    .bind(sender_id)
    .bind(recipient_id)
    .bind(ip_id)
    .fetch_one(pool)
    .await?;

    Ok(())
}
