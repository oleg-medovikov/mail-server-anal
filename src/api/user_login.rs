use rocket::{post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{query_scalar, PgPool};
use bcrypt::verify;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
    token: String,
}

#[post("/user_login", format = "json", data = "<credentials>")]
pub async fn user_login(pool: &State<PgPool>, credentials: Json<Credentials>) -> Result<Json<LoginResponse>, rocket::response::Debug<sqlx::Error>> {
    let credentials = credentials.into_inner();

    // Ищем пользователя по username
    let exists: bool = query_scalar(r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 and active = true)"#)
        .bind(&credentials.username)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| rocket::response::Debug(e))?;

    if !exists {
        return Err(rocket::response::Debug(sqlx::Error::RowNotFound));
    }

    // Получаем хеш пароля из базы данных
    let user_hash: String = query_scalar(r#"SELECT password_hash FROM users WHERE username = $1"#)
        .bind(&credentials.username)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| rocket::response::Debug(e))?;

    if let Ok(valid) = verify(credentials.password.as_bytes(), &user_hash) {
        if valid {
            // Генерируем UUID для токена
            let token = Uuid::new_v4().to_string();

            // Обновляем строку в базе данных с новым токеном
            sqlx::query!(
                r#"UPDATE users SET token = $1 WHERE username = $2"#,
                token,
                credentials.username
            )
            .execute(pool.inner())
            .await
            .map_err(|e| rocket::response::Debug(e))?;
            // Возвращаем токен в ответе JSON
            Ok(Json(LoginResponse {
                message: "Logged in successfully".to_string(),
                token,
            }))
        } else {
            Err(rocket::response::Debug(sqlx::Error::RowNotFound))
        }
    } else {
        Err(rocket::response::Debug(sqlx::Error::RowNotFound))
    }
}
