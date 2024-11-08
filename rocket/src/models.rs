use std::fmt;
use chrono::NaiveDateTime;


pub struct MessageInfo {
    pub date: NaiveDateTime,
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


