use rocket::{post, State};
use rocket::serde::json::Json;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct DropTokenRequest {
    token: String,
}

#[post("/drop_token", format = "json", data = "<drop_token_request>")]
pub async fn drop_token(pool: &State<PgPool>, drop_token_request: Json<DropTokenRequest>) -> Result<Json<String>, rocket::response::Debug<sqlx::Error>> {
    let token = drop_token_request.token.clone();

    // Ищем пользователя по токену
    match sqlx::query("UPDATE users SET token = NULL WHERE token = $1")
        .bind(&token)
        .execute(pool.inner())
        .await
    {
        Ok(_) => Ok(Json("Token deleted".to_string())),
        Err(e) => Err(rocket::response::Debug(e)),
    }
}
