use std::fmt;
use chrono::{DateTime, Utc};
use rocket::form::FromForm;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromForm)]
pub struct QueryParams {
    pub page: Option<u32>,
    pub sender: Option<String>,
    pub datetime_start: Option<String>,
    pub datetime_stop: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct Message {
    pub date: DateTime<Utc>,
    pub sender: String,
    pub recipient: String,
    pub ip: String,
    pub size: i32,
    pub passed: String,
    pub data_box: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

pub struct MessageInfo {
    pub date: DateTime<Utc>,
    pub sender: String,
    pub recipient: String,
    pub ip_address: String,
    pub message_id: String,
    pub size: i32,
    pub passed: bool,
}
// Реализуем трейт Display для структуры
impl fmt::Display for MessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
        f,
        "{{ date: {}, sender: {}, recipient: {}, ip_address: {}, message_id: {}, size: {}, passed: {} }}",
        self.date,
        self.sender,
        self.recipient,
        self.ip_address,
        self.message_id,
        self.size,
        self.passed,
    )}
}

pub struct MessageStatus {
    pub date: DateTime<Utc>,
    pub message_id: String,
    pub status: String,
}
