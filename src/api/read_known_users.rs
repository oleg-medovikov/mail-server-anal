use rocket::{get, State};
use sqlx::PgPool;
use std::fs::File;
use std::io;
use csv::ReaderBuilder;

#[get("/read_known_users")]
pub async fn read_known_users(pool: &State<PgPool>) -> io::Result<String> {
    // Открываем CSV файл
    let file = File::open("/home/user/mail_log/virtual_users.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // Указываем, что в файле нет заголовков
        .delimiter(b';') // Указываем разделитель ;
        .from_reader(file);

    // Собираем все email в вектор
    let mut emails = Vec::new();
    for result in rdr.records() {
        let record = result?; // Обрабатываем ошибку чтения CSV
        if record.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "CSV record does not have enough columns",
            ));
        }
        let email = record[1].to_string(); // Предполагаем, что email находится во втором столбце
        emails.push(email);
    }

    // Если emails пуст, возвращаем сообщение
    if emails.is_empty() {
        return Ok("No emails found in the CSV file.".to_string());
    }

    // Вставляем все email одним запросом
    sqlx::query(
        r#"
        INSERT INTO known_users (email)
        SELECT unnest($1::text[])
        ON CONFLICT (email) DO NOTHING
        "#,
    )
    .bind(&emails) // Привязываем вектор emails как массив текста
    .execute(pool.inner())
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(format!("Processed and inserted {} users into the database.", emails.len()))
}
