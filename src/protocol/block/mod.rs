use std::fmt::{Debug, Formatter};

use concat_idents::concat_idents;

use crate::sys::*;
use crate::base::field::*;
use crate::core::field::*;

use crate::interface::field::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;

use crate::protocol::*;


include!("origin.rs");
include!("intro.rs");
include!("v1.rs");
include!("block.rs");
