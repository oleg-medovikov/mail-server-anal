use actix_web::{web::Data, get, Responder, HttpResponse, HttpRequest};
use crate::AppState;
use serde::Serialize;
use sqlx::{FromRow, query_scalar, query_as, query, Error};

mod func;
use func::parse_log_content::parse_log_content;

async fn insert_sender(state: Data<AppState>, sender: &str) -> Result<i32, Error> {
    // Проверяем, существует ли уже пользователь с таким username
    let row: Option<i32> = query_as("SELECT id FROM senders WHERE username = $1")
        .bind(sender)
        .fetch_optional(&state.db)
        .await?
        .map(|r: (i32,)| r.0);

    match row {
        Some(id) => Ok(id), // Пользователь найден, возвращаем его id
        None => {
            // Пользователь не найден, выполняем вставку
            let result = query("INSERT INTO senders (username) VALUES ($1) RETURNING id")
                .bind(sender)
                .fetch_one(&state.db)
                .await?;
            let id: i32 = result.get("id");
            Ok(id)
        }
    }
}

async fn insert_recipients(state: Data<AppState>, recipients: &[String]) -> Result<Vec<i32>, Error> {
    let mut recipient_ids = Vec::new();

    for recipient in recipients {
        // Проверяем, существует ли уже получатель с таким username
        let row: Option<i32> = query_as("SELECT id FROM recipients WHERE username = $1")
            .bind(recipient)
            .fetch_optional(&state.db)
            .await?
            .map(|r: (i32,)| r.0);

        match row {
            Some(id) => recipient_ids.push(id), // Получатель найден, добавляем его id
            None => {
                // Получатель не найден, выполняем вставку
                let result = query("INSERT INTO recipients (username) VALUES ($1) RETURNING id")
                    .bind(recipient)
                    .fetch_one(&state.db)
                    .await?;
                let id: i32 = result.get("id");
                recipient_ids.push(id);
            }
        }
    }

    Ok(recipient_ids)
}


#[get("/read_log_files")]
pub async fn read_log_files(state: Data<AppState>, req: HttpRequest) -> impl Responder {

}
