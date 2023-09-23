// use std::fmt;
// use std::cmp::Ordering::{Less,Greater};
// use std::cmp::{Ordering, PartialOrd, Ord};
// use std::ops::{Add, Sub, Mul, Div, Deref, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};

// use num_bigint::BigInt;
// use num_bigint::Sign::{Minus, Plus};
// use num_traits::{FromPrimitive, ToPrimitive, Num};

use std::convert::TryInto;

use super::interface::Error;

use super::interface::field::*;

include!{"macro.rs"}

include!{"bytesfixed.rs"}


