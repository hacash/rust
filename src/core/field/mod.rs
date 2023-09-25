use std::fmt;
use std::cmp::Ordering::{Less,Greater};
use std::cmp::{Ordering, PartialOrd, Ord};
use std::ops::{Deref, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};

// use num_bigint::BigInt;
// use num_bigint::Sign::{Minus, Plus};
// use num_traits::{FromPrimitive, ToPrimitive, Num};

use concat_idents::concat_idents;

use std::convert::TryInto;

use super::interface::Error;

use super::interface::field::*;

include!{"macro.rs"}

include!{"fixed_def.rs"}
include!{"fixed.rs"}
include!{"bytes_def.rs"}
include!{"bytes.rs"}
include!{"optional_def.rs"}
include!{"optional.rs"}
include!{"list_def.rs"}
include!{"list.rs"}
include!{"combine_def.rs"}
include!{"combine.rs"}


