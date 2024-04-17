use chrono::{DateTime, Local};


pub fn curtimes() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64
}


pub fn ctshow() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
