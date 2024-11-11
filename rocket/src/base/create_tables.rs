use sqlx::{Executor, PgPool};

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS sender (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS recipient (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS ip (
            id SERIAL PRIMARY KEY,
            ip VARCHAR(255) NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS message (
            id VARCHAR(255) PRIMARY KEY,
            date TIMESTAMPTZ,
            passed BOOLEAN,
            size INTEGER,
            sender_id SERIAL REFERENCES sender(id),
            recipient_id SERIAL REFERENCES recipient(id),
            ip_id SERIAL REFERENCES ip(id)
        );
        CREATE TABLE IF NOT EXISTS status(
            id SERIAL PRIMARY KEY,
            mess VARCHAR(255) NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS mess_status (
            message_id VARCHAR(255) PRIMARY KEY,
            date TIMESTAMPTZ,
            status_id SERIAL REFERENCES status(id)
        );
        "#,
    )
    .await?;

    Ok(())
}
