use sqlx::{query, query_as, PgPool};
use crate::models::MessageInfo;

pub async fn save_message(pool: &PgPool, mess: MessageInfo) -> Result<(), sqlx::Error> {
    // Сначала добавляем sender
    let sender_id: i32 = query_as::<_, (i32,)>(
        r#"
        WITH inserted AS (
            INSERT INTO sender (email)
            VALUES ($1)
            ON CONFLICT (email)
            DO NOTHING
            RETURNING id
        )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM sender WHERE email = $1
        LIMIT 1;
        "#
    )
    .bind(&mess.sender)
    .fetch_one(pool)
    .await?
    .0;

    let recipient_id: i32 = query_as::<_, (i32,)>(
        r#"
        WITH inserted AS (
            INSERT INTO recipient (email)
            VALUES ($1)
            ON CONFLICT (email)
            DO NOTHING
            RETURNING id
        )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM recipient WHERE email = $1
        LIMIT 1;
        "#
    )
    .bind(&mess.recipient)
    .fetch_one(pool)
    .await?
    .0;

    let ip_id: i32 = query_as::<_, (i32,)>(
        r#"
        WITH inserted AS (
            INSERT INTO ip (ip)
            VALUES ($1)
            ON CONFLICT (ip)
            DO NOTHING
            RETURNING id
        )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM ip WHERE ip = $1
        LIMIT 1;
        "#
    )
    .bind(&mess.ip_address)
    .fetch_one(pool)
    .await?
    .0;

    query(
        r#"
        INSERT INTO message (
            id, date, passed, size, sender_id, recipient_id, ip_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id)
        DO NOTHING;
        "#
    )
    .bind(mess.message_id)
    .bind(mess.date)
    .bind(mess.passed)
    .bind(mess.size)
    .bind(sender_id)
    .bind(recipient_id)
    .bind(ip_id)
    .execute(pool)
    .await?;

    Ok(())
}
