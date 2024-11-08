use serde::Serialize;
use rocket::{get, http::Status, serde::json::Json};
use sqlx::PgPool;
use chrono::{Datelike, Utc, NaiveDateTime};
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::models::MessageInfo;
use crate::base::save_message::save_message;


fn parse_line(line: &str) -> Option<MessageInfo> {
    // Находим дату
    let date_end = line.find(" mail")?;
    let now = Utc::now();
    let year = now.year();
    let date_str = year.to_string() + " " + &line[..date_end];
    // Создаем NaiveDateTime из строки
    let date = NaiveDateTime::parse_from_str(&date_str, "%Y %b %e %H:%M:%S")
        .expect("не удалось распарсить дату");

    // Находим отправителя
    let sender_start = line.find('<')? + 1;
    let sender_end = line.find('>')?;
    let sender = &line[sender_start..sender_end];

    // Находим получателя
    let recipient_start = line.find(" -> <")? + 5;
    let recipient_end = line[recipient_start..].find('>')? + recipient_start;
    let recipient = &line[recipient_start..recipient_end];

    // Создаем регулярное выражение для поиска IP-адреса
    let re = Regex::new(r"\[(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\]").unwrap();
    // Ищем первое совпадение в строке с использованием match
    let ip_address = match re.captures(&line) {
        Some(cap) => cap[1].to_string(),
        None => "не найден".to_string(),
    };

    // Находим Message-ID
    let re = Regex::new(r"Message-ID: <([^>]+)>").unwrap();
    // Ищем первое совпадение в строке с использованием match
    let message_id = match re.captures(&line) {
        Some(cap) => cap[1].to_string(),
        None => "не найден".to_string(),
    };
    // находим size 
    let re = Regex::new(r"size: (\d+),").unwrap();
    // Ищем первое совпадение в строке с использованием match
        let size = match re.captures(&line) {
        Some(cap) => {
            // Преобразуем строку в число i32
            match cap[1].parse::<i32>() {
                Ok(size) => size,
                Err(_) => -1, // Если преобразование не удалось, возвращаем -1
            }
        },
        None => -1, // Если совпадение не найдено, возвращаем -1
    };
    // находим passed
    let mut passed: bool = false;
    if line.contains("CLEAN") {
        passed = true;
    }

    Some(MessageInfo {
        date,
        sender: sender.to_string(),
        recipient: recipient.to_string(),
        ip_address: ip_address.to_string(),
        message_id: message_id.to_string(),
        size,
        passed
    })
}


// Define a struct for the response data
#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

// Create a handler function that returns a JSON response
#[get("/read_files")]
pub async fn read_files(pool: &rocket::State<PgPool>) -> Result<Json<HelloResponse>, Status> {
    
    let file = File::open("/home/user/mail_log/test.log").expect("Unable to open file!");
    let reader = BufReader::new(file);

    let mut response = HelloResponse {
        message: String::from("Файлы прочитаны"),
    };

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        
        if line.contains("CLEAN") || line.contains("SPAM") {
            if let Some(parsed_info) = parse_line(&line) {
                response = HelloResponse {
                    message: format!("Файлы прочитаны {} \n\n {}", &parsed_info, &line),
                };
                println!("Файлы прочитаны {} \n\n {}", &parsed_info, line);
                let _ = save_message(&pool, parsed_info).await;
                break
            }
        }
    }

    Ok(Json(response))
}

