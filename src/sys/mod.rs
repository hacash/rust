// use concat_idents::concat_idents;

use std::collections::{ HashMap };

pub type Error = String;

include!{"panic.rs"}
include!{"string.rs"}
include!{"number.rs"}
include!{"hex.rs"}
include!{"ini.rs"}

