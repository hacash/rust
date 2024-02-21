

// We need the trait in scope to use Utc::timestamp().
use chrono::{TimeZone, Utc, Duration};

use wasm_bindgen::prelude::*;

use crate::core::field;
use crate::core::field::*;
use crate::core::interface::field::*;
use crate::core::interface::transaction::*;
use crate::core::protocol::action;
use crate::core::protocol::action::*;
use crate::core::protocol::transaction;

/******** sdk ********/

macro_rules! or_return {
    ($tip:expr, $gain:expr) => (
        match $gain {
            Ok(obj) => obj,
            Err(e) => {
                return format!("[ERROR] {}: {}", $tip, e)
            }
        }
    )
}


include!{"amount.rs"}
include!{"account.rs"}
include!{"sign.rs"}
include!{"transfer.rs"}

