
pub trait Json {
    fn to_json(&self) -> String { s!("*") }
    fn from_json(&mut self, _: &[u8]) -> Option<Error> { panic_never_call_this!(); None }
}

