#[macro_use]
mod sys;
#[macro_use]
mod base;
mod interface;


use sys::Error;


/*

cargo run -A unused_import
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

RUSTFLAGS="$RUSTFLAGS -Awarnings" cargo run

*/

fn main() {

    // panic_never_call_this!();

    println!("{} {}", "abc 123", s!("error"));
}
