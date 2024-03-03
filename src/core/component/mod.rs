use core::ops::{Index, IndexMut};

use super::Error;
use super::base::FieldJsonConfig;
use super::interface::field::*;
use super::interface::action::*;
use super::field_bnk::*;


include!{"macro.rs"}

include!{"balance.rs"}
include!{"sign.rs"}
include!{"coinbase.rs"}
include!{"dynlist.rs"}

