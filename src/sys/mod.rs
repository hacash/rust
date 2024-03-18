use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{ HashMap };

// use concat_idents::concat_idents;

pub type Error = String;

include!{"panic.rs"}
include!{"string.rs"}
include!{"error.rs"}
include!{"number.rs"}
include!{"hex.rs"}
include!{"ini.rs"}
include!{"time.rs"}

