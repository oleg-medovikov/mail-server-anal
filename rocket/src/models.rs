use std::fmt;
use chrono::{DateTime, Utc};


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
