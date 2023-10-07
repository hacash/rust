pub type Error = String;

include!{"macro.rs"}

#[macro_use]
pub mod base;

#[macro_use]
pub mod interface;

#[macro_use]
pub mod field;

#[macro_use]
pub mod component;

#[macro_use]
pub mod protocol;

#[macro_use]
pub mod store;
