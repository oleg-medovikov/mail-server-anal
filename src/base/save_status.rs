use sqlx::{query, query_as, PgPool};
use crate::models::MessageStatus;

pub async fn save_status(pool: &PgPool, mess: MessageStatus) -> Result<(), sqlx::Error> {
    // Сначала добавляем sender
    let status_id: i32 = query_as::<_, (i32,)>(
        r#"
        WITH inserted AS (
            INSERT INTO status (mess)
            VALUES ($1)
            ON CONFLICT (mess)
            DO NOTHING
            RETURNING id
        )
        SELECT id FROM status
        UNION ALL
        SELECT id FROM status WHERE mess = $1
        LIMIT 1;
        "#
    )
    .bind(&mess.status)
    .fetch_one(pool)
    .await?
    .0;

    query(
        r#"
        INSERT INTO mess_status (
            message_id, date, status_id
        )
        VALUES ($1, $2, $3)
        ON CONFLICT (message_id)
        DO NOTHING;
        "#
    )
    .bind(mess.message_id)
    .bind(mess.date)
    .bind(status_id)
    .execute(pool)
    .await?;

    Ok(())
}
