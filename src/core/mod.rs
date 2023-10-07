pub type Error = String;

include!{"macro.rs"}

pub mod interface;

#[macro_use]
pub mod base;

#[macro_use]
pub mod field;

pub mod component;

pub mod protocol;
