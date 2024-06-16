use regex::Regex;
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug)]
struct MailLog {
    time: NaiveDateTime,
    mess_id: String,
    size: i32,
    sender: String,
    recipients: Vec<String>,
}

pub fn parse_log_content(content: &str) -> Vec<MailLog> {
    let mut logs = Vec::new();

    let regexes: HashMap<&str, Regex> = vec![
        ("time", Regex::new(r"(?P<time>\w{3} \d{2} \d{2}:\d{2}:\d{2})").unwrap()),
        ("mess_id", Regex::new(r"Message-ID: <(?P<mess_id>.*)>").unwrap()),
        ("size", Regex::new(r"size: (?P<size>\d+)").unwrap()),
        ("sender", Regex::new(r"<(?P<sender>.*)> ->").unwrap()),
        ("recipients", Regex::new(r"-> <(?P<recipients>.*)>,").unwrap()),
    ].into_iter().collect();

    for line in content.lines() {
        let mut captures: HashMap<&str, String> = HashMap::new();

        for (key, re) in &regexes {
            if let Some(caps) = re.captures(line) {
                captures.insert(key, caps[key].to_string());
            }
        }

        if captures.len() == regexes.len() {
            let time_str = format!("{} {}", captures["time"], "2023"); // Предполагаем текущий год
            let time = NaiveDateTime::parse_from_str(&time_str, "%b %d %H:%M:%S %Y").unwrap();
            let mess_id = captures["mess_id"].clone();
            let size = captures["size"].parse::<i32>().unwrap();
            let sender = captures["sender"].clone();
            let recipients = captures["recipients"].split(',').map(|s| s.trim().to_string()).collect();

            logs.push(MailLog {
                time,
                mess_id,
                size,
                sender,
                recipients,
            });
        }
    }

    logs
}
