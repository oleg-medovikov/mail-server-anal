use sqlx::{Executor, PgPool};

pub async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            active BOOLEAN NOT NULL,
            token VARCHAR(255) NULL
        );
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
            date TIMESTAMP,
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
            date TIMESTAMP,
            status_id SERIAL REFERENCES status(id)
        );
        CREATE TABLE IF NOT EXISTS known_users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE
        );
        "#,
    )
    .await?;

    Ok(())
}
