use serde::Serialize;
use rocket::{get, http::Status, serde::json::Json};
use sqlx::PgPool;

use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct MessageInfo {
    date: String,
    sender: String,
    recipient: String,
    ip_address: String,
    message_id: String,
}
// Реализуем трейт Display для структуры
impl fmt::Display for MessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ date: {}, sender: {}, recipient: {}, ip_address: {}, message_id: {} }}", self.date, self.sender, self.recipient, self.ip_address, self.message_id)
    }
}


fn parse_line(line: &str) -> Option<MessageInfo> {
    // Находим дату
    let date_end = line.find(' ')?;
    let date = &line[..date_end];

    // Находим отправителя
    let sender_start = line.find('<')? + 1;
    let sender_end = line.find('>')?;
    let sender = &line[sender_start..sender_end];

    // Находим получателя
    let recipient_start = line.find(" -> <")? + 5;
    let recipient_end = line[recipient_start..].find('>')? + recipient_start;
    let recipient = &line[recipient_start..recipient_end];

    // Находим IP-адрес
    let ip_start = line.find('[')? + 1;
    let ip_end = line.find(']')?;
    let ip_address = &line[ip_start..ip_end];

    // Находим Message-ID
    let message_id_start = line.find("Message-ID: <")? + 12;
    let message_id_end = line[message_id_start..].find('>')? + message_id_start;
    let message_id = &line[message_id_start..message_id_end];

    Some(MessageInfo {
        date: date.to_string(),
        sender: sender.to_string(),
        recipient: recipient.to_string(),
        ip_address: ip_address.to_string(),
        message_id: message_id.to_string(),
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
                    message: format!("Файлы прочитаны {}", parsed_info),
                };
                break
            }
        }
    }

    Ok(Json(response))
}

