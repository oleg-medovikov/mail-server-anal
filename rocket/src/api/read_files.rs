use serde::Serialize;
use rocket::{get, http::Status, serde::json::Json};
use sqlx::PgPool;
use chrono::{Datelike, Utc, TimeZone, NaiveDateTime};
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::models::{MessageInfo, MessageStatus};
use crate::base::save_message::save_message;
use crate::base::save_status::save_status;

fn parsed_status(line: &str) -> Option<MessageStatus> {
    // Находим дату
    let date_end = line.find(" lda")?;
    let now = Utc::now();
    let year = now.year();
    let date_str = year.to_string() + " " + &line[..date_end].replace("  ", " ");
    // Создаем NaiveDateTime из строки
    let naive_date = NaiveDateTime::parse_from_str(&date_str, "%Y %b %e %H:%M:%S").expect("Не смог прочесть дату");
    let date = Utc.from_utc_datetime(&naive_date);
    
    // Находим статус
    let status = match line.rsplit(':').next() {
        Some(result) => result.trim().to_string(),
        None =>  "не найдена часть строки".to_string(),
    };
    // Находим Message-ID
    let re = Regex::new(r"Info: msgid=<([^>]+)>").unwrap();
    // Ищем первое совпадение в строке с использованием match
    let message_id = match re.captures(&line) {
        Some(cap) => cap[1].to_string(),
        None => "не найден".to_string(),
    };

    Some(MessageStatus {
        message_id: message_id.to_string(),
        date: date.into(),
        status: status.to_string(),
    })
}


fn parse_line(line: &str) -> Option<MessageInfo> {
    // Находим дату
    let date_end = line.find(" mail")?;
    let now = Utc::now();
    let year = now.year();
    let date_str = year.to_string() + " " + &line[..date_end].replace("  ", " ");
    // Создаем NaiveDateTime из строки
    let naive_date = NaiveDateTime::parse_from_str(&date_str, "%Y %b %e %H:%M:%S").expect("Не смог прочесть дату");
    let date = Utc.from_utc_datetime(&naive_date);
    
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
        date: date.into(),
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
    
    let file = File::open("/home/user/mail_log/lda.log").expect("Unable to open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        
        if let Some(parsed_status) = parsed_status(&line) {
            let _ = save_status(&pool, parsed_status).await;
        }
    }
   
    let file = File::open("/home/user/mail_log/mail.log").expect("Unable to open file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        
        if line.contains("CLEAN") || line.contains("SPAM") {
            if let Some(parsed_info) = parse_line(&line) {
                let _ = save_message(&pool, parsed_info).await;
            }
        }
    }

    let response = HelloResponse {
        message: String::from("Файлы прочитаны"),
    };


    Ok(Json(response))
}

