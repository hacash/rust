
#[macro_use]
mod sys;
#[macro_use]
mod base;
mod interface;
mod core;

use crate::interface::field::*;


/*

cargo run -A unused_import
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

RUSTFLAGS="$RUSTFLAGS -Awarnings" cargo run

*/

fn main() {

    let addrhac = core::field::AddrHac::new();

    println!("{} {} {}", "abc 123", s!("error"), addrhac.amount);

    // panic_never_call_this!();
}
