use rocket::{post, serde::json::Json, State};
use sqlx::{PgPool, query_scalar};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TokenRequest {
    token: String,
}

#[derive(Serialize)]
pub struct TokenStatus {
    token_valid: bool,
}

#[post("/check_token", format = "json", data = "<token_request>")]
pub async fn check_token(pool: &State<PgPool>, token_request: Json<TokenRequest>) -> Result<Json<TokenStatus>, rocket::response::Debug<sqlx::Error>> {
    // Ищем пользователя по токену
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE token = $1)"#)
        .bind(&token_request.token)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| rocket::response::Debug(e))?;

    let token_status = TokenStatus { token_valid: exists };
    Ok(Json(token_status))
}
