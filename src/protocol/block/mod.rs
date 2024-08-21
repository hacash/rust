use std::fmt::{Debug, Formatter};

use concat_idents::concat_idents;

use crate::x16rs;
use crate::sys::*;
use crate::base::field::*;
use crate::core::field::*;
use crate::core::component::*;

use crate::interface::field::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;

use crate::protocol::*;

pub const BLOCK_VERSION_1: u8 = 1;

include!("util.rs");
include!("origin.rs");
include!("recent.rs");
include!("intro.rs");
include!("v1.rs");
include!("block.rs");
