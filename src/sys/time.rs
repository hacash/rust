
pub fn curtimes() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u64
}